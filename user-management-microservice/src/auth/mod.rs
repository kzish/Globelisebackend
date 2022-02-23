//! Endpoints for user authentication and authorization.

use std::collections::HashMap;

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::{
    async_trait,
    extract::{Extension, Form, FromRequest, Path, Query, RequestParts},
};
use email_address::EmailAddress;
use once_cell::sync::Lazy;
use rand::Rng;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use crate::{database::SharedDatabase, error::Error};

pub mod google;
pub mod password;
mod state;
pub mod token;
pub mod user;

use token::{create_access_token, RefreshToken};
use user::{Role, User};

pub use state::{SharedState, State};

/// Creates an account.
pub async fn signup(
    Form(request): Form<CreateAccountRequest>,
    Path(role): Path<Role>,
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
    let ulid = database.create_user(user, role).await?;

    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state.open_session(&database, ulid, role).await?;
    Ok(refresh_token)
}

/// Logs a user in.
pub async fn login(
    Form(request): Form<LoginRequest>,
    Path(role): Path<Role>,
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
    if let Some(ulid) = database.user_id(&email, role).await? {
        if let Some((
            User {
                password_hash: Some(hash),
                ..
            },
            _,
        )) = database.user(ulid, Some(role)).await?
        {
            if let Ok(true) = verify_encoded(&hash, password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;
                let refresh_token = shared_state.open_session(&database, ulid, role).await?;
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
    let role: Role = claims.role.parse().unwrap();

    let database = database.lock().await;
    if let Some((User { email, .. }, _)) = database.user(ulid, Some(role)).await? {
        let access_token = create_access_token(ulid, email, role)?;
        Ok(access_token)
    } else {
        Err(Error::Unauthorized("Invalid refresh token"))
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RedirectTo(pub String);

#[async_trait]
impl<B> FromRequest<B> for RedirectTo
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Query(params) = Query::<HashMap<String, String>>::from_request(req)
            .await
            .map_err(|_| Error::Unauthorized("No one-time token provided"))?;
        if let Some(redirect_url) = params.get("redirect_to") {
            Ok(RedirectTo(redirect_url.clone()))
        } else {
            Err(Error::BadRequest("Must specify a redirect URI"))
        }
    }
}
