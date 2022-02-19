//! Endpoints for user authentication and authorization.

use std::str::FromStr;

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::extract::{Extension, Form, Path};
use email_address::EmailAddress;
use once_cell::sync::Lazy;
use rand::Rng;
use rusty_ulid::Ulid;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

mod database;
mod error;
pub mod google;
pub mod onboarding;
pub mod password;
mod state;
mod token;
mod user;

pub use database::{Database, SharedDatabase};
use error::{Error, RegistrationError};
use token::{create_access_token, RefreshToken};
use user::{Role, User};

pub use state::{SharedState, State};

/// Creates an account.
pub async fn create_account(
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
    let email = EmailAddress::from_str(&email);
    // NOTE: EOR admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::EorAdmin) {
        return Err(Error::Unauthorized);
    }

    let is_valid_email = email.is_ok();
    let mut is_email_available = false;
    let is_password_at_least_8_chars = request.password.len() >= 8;
    let passwords_match = password == confirm_password;

    if is_valid_email {
        let database = database.lock().await;
        let email = email.unwrap();
        is_email_available = database.user_id(&email, role).await?.is_none();

        if is_email_available && is_password_at_least_8_chars && passwords_match {
            let salt: [u8; 16] = rand::thread_rng().gen();
            let hash = hash_encoded(password.as_bytes(), &salt, &HASH_CONFIG)
                .map_err(|_| Error::Internal)?;

            let user = User {
                email,
                password_hash: Some(hash),
                google: false,
                outlook: false,
            };
            let ulid = database.create_user(user, role).await?;

            let mut shared_state = shared_state.lock().await;
            let refresh_token = shared_state.open_session(&database, ulid, role).await?;
            return Ok(refresh_token);
        }
    }

    Err(Error::Registration(RegistrationError {
        is_valid_email,
        is_email_available,
        is_password_at_least_8_chars,
        passwords_match,
    }))
}

/// Logs a user in.
pub async fn login(
    Form(request): Form<LoginRequest>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    let email: EmailAddress = request.email.parse().map_err(|_| Error::BadRequest)?;

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
            if let Ok(true) = verify_encoded(&hash, request.password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;
                let refresh_token = shared_state.open_session(&database, ulid, role).await?;
                return Ok(refresh_token);
            }
        }
    }

    Err(Error::Unauthorized)
}

/// Gets a new access token.
pub async fn renew_access_token(
    claims: RefreshToken,
    Extension(database): Extension<SharedDatabase>,
) -> Result<String, Error> {
    let ulid: Ulid = claims
        .sub
        .parse()
        .map_err(|_| Error::Conversion("uuid parse error".into()))?;
    let role: Role = claims
        .role
        .parse()
        .map_err(|_| Error::Conversion("role parse error".into()))?;

    let database = database.lock().await;
    if let Some((User { email, .. }, _)) = database.user(ulid, Some(role)).await? {
        let access_token = create_access_token(ulid, email, role)?;
        Ok(access_token)
    } else {
        Err(Error::Unauthorized)
    }
}

/// Gets the public key for decoding tokens.
pub async fn public_key() -> String {
    (*token::PUBLIC_KEY).clone()
}

/// Request for creating a user.
#[derive(Deserialize)]
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
