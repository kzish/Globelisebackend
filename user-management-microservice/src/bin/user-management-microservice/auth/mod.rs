//! Endpoints for user authentication and authorization.

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::{
    extract::{ContentLengthLimit, Extension, Path},
    Json,
};
use common_utils::{
    custom_serde::{EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::{create_token, Token},
};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{UserRole, UserType},
};

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
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<CreateAccountRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Path(user_type): Path<UserType>,
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

    if database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .is_some()
    {
        return Err(GlobeliseError::bad_request(
            "User with that email already exists. Did you forgot the password?",
        ));
    }

    let ulid = database
        .create_user(
            body.email,
            Some(hash),
            false,
            false,
            user_type == UserType::Entity,
            user_type == UserType::Individual,
            false,
            false,
        )
        .await?;

    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state
        .open_session(&database, ulid, user_type)
        .await?;
    Ok(refresh_token)
}

/// Logs a user in.
pub async fn login(
    ContentLengthLimit(Json(body)): ContentLengthLimit<Json<LoginRequest>, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let password: String = body.password.nfc().collect();

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    let database = database.lock().await;
    if let Some(user) = database
        .find_one_user(None, Some(&body.email), None)
        .await?
    {
        if let Some(hash) = &user.password {
            if let Ok(true) = verify_encoded(hash, password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;

                let user_type = user.user_type()?;

                let refresh_token = shared_state
                    .open_session(&database, user.ulid, user_type)
                    .await?;
                Ok(refresh_token)
            } else {
                Err(GlobeliseError::unauthorized(
                    "User entered the wrong the password",
                ))
            }
        } else {
            Err(GlobeliseError::unauthorized(
                "User was not signed up using password authentication",
            ))
        }
    } else {
        Err(GlobeliseError::unauthorized(
            "Cannot find user with that email",
        ))
    }
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
    if let Some(User { email, .. }) = database
        .find_one_user(Some(ulid), None, Some(user_type))
        .await?
    {
        let mut user_roles = vec![];
        if database
            .get_is_user_fully_onboarded(ulid, user_type, UserRole::Client)
            .await?
        {
            user_roles.push(UserRole::Client);
        };
        if database
            .get_is_user_fully_onboarded(ulid, user_type, UserRole::Contractor)
            .await?
        {
            user_roles.push(UserRole::Contractor);
        };
        let access_token = UserAccessToken {
            ulid,
            email,
            user_type,
            user_roles,
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
    email: EmailWrapper,
    password: String,
    confirm_password: String,
}

/// Request for logging a user in.
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
