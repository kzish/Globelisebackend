//! Endpoints for admin authentication and authorization.

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::{create_token, Token},
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

use crate::database::SharedDatabase;

pub mod google;
pub mod password;
mod state;
pub mod token;

pub use state::{SharedState, State};
use token::RefreshToken;
use token::KEYS;

/// Creates an account.
pub async fn signup(
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<CreateAccountRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let password: String = body.password.nfc().collect();
    let confirm_password: String = body.confirm_password.nfc().collect();

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

    let database = database.lock().await;

    //Check for existing user
    if database
        .find_one_admin(None, Some(&body.email))
        .await?
        .is_some()
    {
        return Err(GlobeliseError::UnavailableEmail);
    }

    let ulid = database
        .insert_one_admin(body.email, hash, false, false)
        .await?;

    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state.open_session(ulid).await?;
    Ok(refresh_token)
}

/// Logs a admin in.
pub async fn login(
    ContentLengthLimit(Json(body)): ContentLengthLimit<Json<LoginRequest>, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let password: String = body.password.nfc().collect();

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    // Simplify this step
    let database = database.lock().await;
    if let Some(admin) = database.find_one_admin(None, Some(&body.email)).await? {
        {
            if let Ok(true) = verify_encoded(&admin.password, password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;
                let refresh_token = shared_state.open_session(admin.ulid).await?;
                Ok(refresh_token)
            } else {
                Err(GlobeliseError::unauthorized(
                    "Entered the wrong the password",
                ))
            }
        }
    } else {
        Err(GlobeliseError::unauthorized(
            "Cannot find admin with that email",
        ))
    }
}

/// Gets a new access token.
pub async fn access_token(
    claims: Token<RefreshToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let mut shared_state = shared_state.lock().await;
    let mut is_session_valid = false;
    let _ = shared_state
        .clear_expired_sessions(claims.payload.ulid)
        .await;
    if let Some(sessions) = shared_state.sessions(claims.payload.ulid).await? {
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
    if let Some(admin) = database
        .find_one_admin(Some(claims.payload.ulid), None)
        .await?
    {
        let access = AdminAccessToken {
            ulid: claims.payload.ulid,
            email: admin.email,
        };
        let (access_token, _) = create_token(access, &KEYS.encoding)?;
        Ok(access_token)
    } else {
        Err(GlobeliseError::unauthorized("Invalid refresh token"))
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
    email: EmailWrapper,
    password: String,
    confirm_password: String,
}

/// Request for logging a admin in.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoginRequest {
    email: EmailWrapper,
    password: String,
}

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});
