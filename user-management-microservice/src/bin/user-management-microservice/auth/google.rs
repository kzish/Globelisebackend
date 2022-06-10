//! Endpoint for handling Google authentication.

use axum::extract::{Extension, Form};
use common_utils::error::{GlobeliseError, GlobeliseResult};
use google_auth::IdToken;
use once_cell::sync::Lazy;

use super::{SharedDatabase, SharedState};

/// Log in as a Google user.
pub async fn login(
    Form(id_token): Form<IdToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let claims = id_token.decode(&*CLIENT_ID).await.map_err(|e| match e {
        google_auth::Error::Decoding(_) => GlobeliseError::unauthorized("Google login failed"),
        _ => GlobeliseError::Internal("Failed to decode Google ID token".into()),
    })?;

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;
    if let Some(user) = database
        .find_one_user(None, Some(&claims.email), None)
        .await?
    {
        if user.is_google {
            let user_type = user.user_type()?;
            let refresh_token = shared_state
                .open_session(&database, user.ulid, user_type)
                .await?;

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

#[cfg(not(debug_assertions))]
pub async fn login_page() -> Error {
    GlobeliseError::NotFound
}

#[cfg(debug_assertions)]
// Use absolute namespace to silence errors about unused imports.
pub async fn login_page() -> axum::response::Html<String> {
    use crate::env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL;

    axum::response::Html(format!(
        r##"
        <!DOCTYPE html>
        <html>
          <head>
            <title>Globelise Login Page</title>
          </head>
          <body>
            <script src="https://accounts.google.com/gsi/client" async defer></script>
            <div
              id="g_id_onload"
              data-client_id="{}"
              data-login_uri="{}/auth/google/login"
              data-auto_prompt="false"
            ></div>
            <div
              class="g_id_signin"
              data-type="standard"
              data-size="large"
              data-theme="outline"
              data-text="sign_in_with"
              data-shape="rectangular"
              data-logo_alignment="left"
            ></div>
          </body>
        </html>
        "##,
        (*CLIENT_ID),
        (*USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL)
    ))
}
