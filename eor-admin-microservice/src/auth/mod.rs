//! Endpoints for admin authentication and authorization.

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::extract::{Extension, Form};
use email_address::EmailAddress;
use once_cell::sync::Lazy;
use rand::Rng;
use rusty_ulid::Ulid;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

use crate::{database::SharedDatabase, error::Error};

pub mod admin;
pub mod google;
pub mod password;
mod state;
pub mod token;

use admin::Admin;
use token::{create_access_token, RefreshToken};

pub use state::{SharedState, State};

/// Creates an account.
pub async fn signup(
    Form(request): Form<CreateAccountRequest>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();
    let confirm_password: String = request.confirm_password.nfc().collect();

    // Frontend validation can be bypassed, so perform basic validation
    // in the backend as well.
    let email = email
        .parse::<EmailAddress>()
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

    let admin = Admin {
        email,
        password_hash: Some(hash),
        google: false,
        outlook: false,
    };
    let database = database.lock().await;
    let ulid = database.create_admin(admin).await?;

    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state.open_session(&database, ulid).await?;
    Ok(refresh_token)
}

/// Logs a admin in.
pub async fn login(
    Form(request): Form<LoginRequest>,
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
    if let Some(ulid) = database.admin_id(&email).await? {
        if let Some(Admin {
            password_hash: Some(hash),
            ..
        }) = database.admin(ulid).await?
        {
            if let Ok(true) = verify_encoded(&hash, password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;
                let refresh_token = shared_state.open_session(&database, ulid).await?;
                return Ok(refresh_token);
            }
        }
    }

    Err(Error::Unauthorized("Email login failed"))
}

/// Gets a new access token.
pub async fn access_token(
    claims: RefreshToken,
    Extension(database): Extension<SharedDatabase>,
) -> Result<String, Error> {
    let ulid: Ulid = claims.sub.parse().unwrap();

    let database = database.lock().await;
    if let Some(Admin { email, .. }) = database.admin(ulid).await? {
        let access_token = create_access_token(ulid, email)?;
        Ok(access_token)
    } else {
        Err(Error::Unauthorized("Invalid refresh token"))
    }
}

/// Gets the public key for decoding tokens.
pub async fn public_key() -> String {
    (*token::PUBLIC_KEY).clone()
}

/// Request for creating a admin.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateAccountRequest {
    email: String,
    password: String,
    confirm_password: String,
}

/// Request for logging a admin in.
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
