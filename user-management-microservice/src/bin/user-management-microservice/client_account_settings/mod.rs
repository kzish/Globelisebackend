pub mod client;
pub mod eor_admin;
use argon2::{hash_encoded, Config};
use axum::{Extension, Json};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::Deserialize;
use serde_with::serde_as;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::{
    auth::state::SharedState,
    database::{Database, SharedDatabase},
};

use once_cell::sync::Lazy;
use rand::Rng;
use unicode_normalization::UnicodeNormalization;

/// Replace the password for a user with the requested one.
///
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChangePasswordRequest {
    pub new_password: String,
    pub confirm_new_password: String,
}

/// Replace the password for a user with the requested one.
pub async fn user_change_password(
    claims: Token<UserAccessToken>,
    Json(request): Json<ChangePasswordRequest>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<()> {
    let new_password: String = request.new_password.nfc().collect();
    let confirm_new_password: String = request.confirm_new_password.nfc().collect();

    if new_password != confirm_new_password {
        return Err(GlobeliseError::bad_request("Passwords do not match"));
    }

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;

    // NOTE: This is not atomic, so this check is quite pointless.
    // Either rely completely on SQL or use some kind of transaction commit.
    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash = hash_encoded(new_password.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(GlobeliseError::internal)?;

    database
        .user_change_password(claims.payload.ulid, hash)
        .await?;
    shared_state
        .revoke_all_sessions(claims.payload.ulid)
        .await?;

    Ok(())
}

/// Replace the password for a user with the requested one.
pub async fn admin_change_password(
    claims: Token<AdminAccessToken>,
    Json(request): Json<ChangePasswordRequest>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<()> {
    let new_password: String = request.new_password.nfc().collect();
    let confirm_new_password: String = request.confirm_new_password.nfc().collect();

    if new_password != confirm_new_password {
        return Err(GlobeliseError::bad_request("Passwords do not match"));
    }

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;

    // NOTE: This is not atomic, so this check is quite pointless.
    // Either rely completely on SQL or use some kind of transaction commit.
    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash = hash_encoded(new_password.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(GlobeliseError::internal)?;

    database
        .admin_change_password(claims.payload.ulid, hash)
        .await?;
    shared_state
        .revoke_all_sessions(claims.payload.ulid)
        .await?;

    Ok(())
}

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});

impl Database {
    pub async fn user_change_password(
        &self,
        ulid: Uuid,
        new_password_hash: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE 
                users 
            SET 
                password = $1 
            WHERE 
                ulid = $2",
        )
        .bind(new_password_hash)
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    /// Updates a admin's password.
    pub async fn admin_change_password(
        &self,
        ulid: Uuid,
        new_password_hash: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE 
                admin_users 
            SET 
                password = $1 
            WHERE 
                ulid = $2",
        )
        .bind(new_password_hash)
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
