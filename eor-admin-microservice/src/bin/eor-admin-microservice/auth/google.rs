//! Endpoint for handling Google authentication.

use axum::extract::{Extension, Form};
use common_utils::error::{GlobeliseError, GlobeliseResult};
use google_auth::IdToken;
use once_cell::sync::Lazy;

use super::{admin::Admin, SharedDatabase, SharedState};

/// Sign up as an admin through Google sign-in.
pub async fn signup(
    Form(id_token): Form<IdToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let claims = id_token
        .decode_and_validate(&*CLIENT_ID)
        .await
        .map_err(|e| match e {
            google_auth::Error::Decoding(_) => GlobeliseError::unauthorized("Google login failed"),
            e => GlobeliseError::internal(e),
        })?;

    let admin = Admin {
        email: claims.email,
        password: None,
        is_google: true,
        is_outlook: false,
    };
    let database = database.lock().await;
    let ulid = database.create_admin(admin).await?;

    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state.open_session(&database, ulid).await?;
    Ok(refresh_token)
}

/// Log in as an admin through Google sign-in.
pub async fn login(
    Form(id_token): Form<IdToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let claims = id_token
        .decode_and_validate(&*CLIENT_ID)
        .await
        .map_err(|e| match e {
            google_auth::Error::Decoding(_) => GlobeliseError::unauthorized("Google login failed"),
            e => GlobeliseError::internal(e),
        })?;

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;
    if let Some(ulid) = database.admin_id(&claims.email).await? {
        if let Some(Admin {
            is_google: true, ..
        }) = database.admin(ulid).await?
        {
            let refresh_token = shared_state.open_session(&database, ulid).await?;

            Ok(refresh_token)
        } else {
            // TODO: Implement linking with an existing account.
            Err(GlobeliseError::unauthorized(
                "Linking Google with existing account is not implemented",
            ))
        }
    } else {
        Err(GlobeliseError::unauthorized("Google login failed"))
    }
}

/// The Google app's client ID.
static CLIENT_ID: Lazy<String> =
    Lazy::new(|| std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));
