//! Endpoints for user authentication and authorization.

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::{
    extract::{Extension, Path},
    Json,
};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::{create_token, Token},
};
use email_address::EmailAddress;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::SharedDatabase;

pub mod google;
pub mod password;
mod state;
pub mod token;
pub mod user;

use token::RefreshToken;
use user::User;

pub use state::{SharedState, State};

use self::token::KEYS;

/// Creates an account.
pub async fn signup(
    Json(request): Json<CreateAccountRequest>,
    Path(user_type): Path<UserType>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();
    let confirm_password: String = request.confirm_password.nfc().collect();

    // Frontend validation can be bypassed, so perform basic validation
    // in the backend as well.
    let email: EmailAddress = email.parse().map_err(GlobeliseError::bad_request)?;
    if password.len() < 8 {
        return Err(GlobeliseError::bad_request(
            "Password must be at least 8 characters long",
        ));
    }
    if password != confirm_password {
        return Err(GlobeliseError::bad_request("Passwords do not match"));
    }

    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash =
        hash_encoded(password.as_bytes(), &salt, &HASH_CONFIG).map_err(GlobeliseError::internal)?;

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
    Json(request): Json<LoginRequest>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();

    let email: EmailAddress = email.parse().map_err(GlobeliseError::bad_request)?;

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    let database = database.lock().await;
    if let Some((ulid, user_type)) = database.user_id(&email).await? {
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

    Err(GlobeliseError::unauthorized("Email login failed"))
}

/// Gets a new access token.
pub async fn access_token(
    claims: Token<RefreshToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let ulid = claims.payload.ulid;
    let user_type = claims.payload.user_type;

    let mut shared_state = shared_state.lock().await;
    let mut is_session_valid = false;
    let _ = shared_state.clear_expired_sessions(ulid).await;
    if let Some(sessions) = shared_state.sessions(ulid).await? {
        let encoded_claims = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA),
            &claims,
            &KEYS.encoding,
        )?;
        for (hash, _) in sessions.iter() {
            if let Ok(true) = verify_encoded(hash, encoded_claims.as_bytes()) {
                is_session_valid = true;
                break;
            }
        }
    }
    if !is_session_valid {
        return Err(GlobeliseError::unauthorized("Refresh token rejected"));
    }

    let database = database.lock().await;
    if let Some((User { email, .. }, _)) = database.user(ulid, Some(user_type)).await? {
        let access_token = UserAccessToken {
            ulid,
            email: email.to_string(),
            user_type,
        };
        let (access_token, _) = create_token(access_token, &KEYS.encoding)?;
        Ok(access_token)
    } else {
        Err(GlobeliseError::unauthorized(
            "User does not exist in the database",
        ))
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
#[serde(rename_all = "kebab-case")]
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
