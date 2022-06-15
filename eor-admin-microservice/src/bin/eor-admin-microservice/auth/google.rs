//! Endpoint for handling Google authentication.

use axum::extract::Extension;
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::AuthBearer,
};
use google_auth::IdToken;

use crate::env::GOOGLE_CLIENT_ID;

use super::{SharedDatabase, SharedState};

/// Log in as an admin through Google sign-in.
pub async fn login(
    AuthBearer(id_token): AuthBearer,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let claims = IdToken(id_token)
        .decode_and_validate(&*GOOGLE_CLIENT_ID)
        .await
        .map_err(|e| {
            GlobeliseError::unauthorized(format!("Failed to decode Google ID token because\n{}", e))
        })?;

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;
    if let Some(admin) = database.find_one_admin(None, Some(&claims.email)).await? {
        let refresh_token = shared_state.open_session(admin.ulid).await?;
        Ok(refresh_token)
    } else {
        let ulid = database
            .insert_one_admin(claims.email, None, true, false)
            .await?;

        let refresh_token = shared_state.open_session(ulid).await?;
        Ok(refresh_token)
    }
}
