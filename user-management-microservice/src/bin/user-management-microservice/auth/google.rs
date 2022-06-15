//! Endpoint for handling Google authentication.

use axum::extract::Extension;
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::AuthBearer,
};
use google_auth::IdToken;

use crate::env::GOOGLE_CLIENT_ID;

use super::{SharedDatabase, SharedState};

/// Log in as a Google user.
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
    if let Some(user) = database
        .find_one_user(None, Some(&claims.email), None)
        .await?
    {
        let user_type = user.user_type()?;
        let refresh_token = shared_state
            .open_session(&database, user.ulid, user_type)
            .await?;

        Ok(refresh_token)
    } else {
        Err(GlobeliseError::bad_request("Please signup first"))
    }
}
