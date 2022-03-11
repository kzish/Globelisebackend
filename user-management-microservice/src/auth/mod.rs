//! Endpoints for user authentication and authorization.

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::extract::{Extension, Form, Path};
use common_utils::token::{create_token, Token};
use email_address::EmailAddress;
use once_cell::sync::Lazy;
use rand::Rng;
use rusty_ulid::Ulid;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

use crate::{database::SharedDatabase, error::Error};

pub mod google;
pub mod password;
mod state;
pub mod token;
pub mod user;

use token::{AccessToken, RefreshToken};
use user::{User, UserType};

pub use state::{SharedState, State};

use self::token::KEYS;

/// Creates an account.
pub async fn signup(
    Form(request): Form<CreateAccountRequest>,
    Path(user_type): Path<UserType>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();
    let confirm_password: String = request.confirm_password.nfc().collect();

    // Frontend validation can be bypassed, so perform basic validation
    // in the backend as well.
    let email: EmailAddress = email
        .parse()
        .map_err(|_| Error::BadRequest("Not a valid email address"))?;
    if password.len() < 8 {
        return Err(Error::BadRequest(
            "Password must be at least 8 characters long",
        ));
    }
    if password != confirm_password {
        return Err(Error::BadRequest("Passwords do not match"));
    }

    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash = hash_encoded(password.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(|_| Error::Internal("Failed to hash password".into()))?;

    let user = User {
        email,
        password_hash: Some(hash),
        google: false,
        outlook: false,
    };
    let database = database.lock().await;
    let ulid = database.create_user(user, user_type).await?;

    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state
        .open_session(&database, ulid, user_type)
        .await?;
    Ok(refresh_token)
}

/// Logs a user in.
pub async fn login(
    Form(request): Form<LoginRequest>,
    Path(user_type): Path<UserType>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();

    let email: EmailAddress = email
        .parse()
        .map_err(|_| Error::BadRequest("Not a valid email address"))?;

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    let database = database.lock().await;
    if let Some(ulid) = database.user_id(&email, user_type).await? {
        if let Some((
            User {
                password_hash: Some(hash),
                ..
            },
            _,
        )) = database.user(ulid, Some(user_type)).await?
        {
            if let Ok(true) = verify_encoded(&hash, password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;
                let refresh_token = shared_state
                    .open_session(&database, ulid, user_type)
                    .await?;
                return Ok(refresh_token);
            }
        }
    }

    Err(Error::Unauthorized("Email login failed"))
}

/// Gets a new access token.
pub async fn access_token(
    claims: Token<RefreshToken>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<String, Error> {
    let ulid = claims.payload.ulid.parse::<Ulid>().unwrap();
    let user_type = claims.payload.user_type.parse::<UserType>().unwrap();
    let database = database.lock().await;
    if let Some((User { email, .. }, _)) = database.user(ulid, Some(user_type)).await? {
        let access_token = AccessToken {
            ulid: claims.payload.ulid,
            email: email.to_string(),
            user_type: claims.payload.user_type,
        };
        let (access_token, _) = create_token(access_token, &KEYS.encoding)?;
        Ok(access_token)
    } else {
        Err(Error::Unauthorized("User does not exist in the database"))
    }
}

/// Gets the public key for decoding tokens.
pub async fn public_key() -> String {
    (*token::PUBLIC_KEY).clone()
}

/// Request for creating a user.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateAccountRequest {
    email: String,
    password: String,
    confirm_password: String,
}

/// Request for logging a user in.
#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});
