//! Endpoint for handling Google authentication.

use std::collections::HashMap;

use axum::{
    extract::{Extension, Form, Path, Query, TypedHeader},
    headers::Cookie,
    http::{uri, Uri},
    response::Redirect,
};
use email_address::EmailAddress;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use rusty_ulid::Ulid;
use serde::Deserialize;
use time::Duration;

use super::{
    error::Error,
    token::one_time::{OneTimeToken, OneTimeTokenAudience},
    user::{Role, User},
    SharedDatabase, SharedState,
};

/// Log in as a Google user.
pub async fn login(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(id_token): Form<IdToken>,
    Path(role): Path<Role>,
    Query(params): Query<HashMap<String, String>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Redirect, Error> {
    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::BadRequest);
    }

    let redirect_uri: Uri = match params.get("redirect_uri") {
        Some(uri) => uri.parse().map_err(|_| Error::BadRequest)?,
        None => return Err(Error::BadRequest),
    };

    id_token.check_crsf_token(cookie)?;

    let OauthKeyList { keys } = OauthKeyList::new()
        .await
        .map_err(|_| Error::GooglePublicKeys)?;

    let claims = id_token.decode(&keys)?;
    let email: EmailAddress = claims.email.parse().unwrap(); // Google emails should be valid.

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;
    if let Some(ulid) = database.user_id(&email, role).await? {
        if let Some((User { google: true, .. }, _)) = database.user(ulid, Some(role)).await? {
            let one_time_token = shared_state
                .open_one_time_session::<Google>(&database, ulid, role)
                .await?;

            let redirect_uri = append_token_to_uri(redirect_uri, &one_time_token)?;
            Ok(Redirect::to(redirect_uri))
        } else {
            // TODO: Implement linking with an existing account.
            Err(Error::BadRequest)
        }
    } else {
        let user = User {
            email,
            password_hash: None,
            google: true,
            outlook: false,
        };
        let ulid = database.create_user(user, role).await?;
        let one_time_token: String = shared_state
            .open_one_time_session::<Google>(&database, ulid, role)
            .await?;
        let redirect_uri = append_token_to_uri(redirect_uri, &one_time_token)?;
        Ok(Redirect::to(redirect_uri))
    }
}

pub async fn get_refresh_token(
    claims: OneTimeToken<Google>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
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
    let mut shared_state = shared_state.lock().await;
    let refresh_token = shared_state.open_session(&database, ulid, role).await?;
    Ok(refresh_token)
}

fn append_token_to_uri(uri: Uri, token: &str) -> Result<Uri, Error> {
    let query = uri.path_and_query().ok_or(Error::BadRequest)?;
    let query = match query.query() {
        Some(_) => query.to_string() + "&token=" + token,
        None => query.to_string() + "?token=" + token,
    };

    let uri = uri::Builder::new()
        .scheme(uri.scheme().ok_or(Error::BadRequest)?.clone())
        .authority(uri.authority().ok_or(Error::BadRequest)?.clone())
        .path_and_query(query)
        .build()
        .map_err(|_| Error::BadRequest)?;
    Ok(uri)
}

/// Representation of Google's ID token.
#[derive(Deserialize, Debug)]
pub struct IdToken {
    credential: String,
    g_csrf_token: String,
}

impl IdToken {
    /// Validate the CRSF token.
    fn check_crsf_token(&self, cookie: Cookie) -> Result<(), Error> {
        if let Some(crsf_token) = cookie.get("g_csrf_token") {
            if crsf_token == self.g_csrf_token {
                return Ok(());
            }
        }
        Err(Error::Unauthorized)
    }

    /// Decode and validate the token.
    fn decode(&self, keys: &[OuathKey; 2]) -> Result<Claims, Error> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[(*CLIENT_ID).clone()]);
        validation.set_issuer(&["https://accounts.google.com"]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        // NOTE: Currently, only the second key works. This is subject to change.
        let TokenData { claims, .. } = decode::<Claims>(
            &*self.credential,
            &DecodingKey::from_rsa_components(&*keys[1].n, &*keys[1].e)
                .map_err(|_| Error::Unauthorized)?,
            &validation,
        )
        .map_err(|_| Error::Unauthorized)?;

        Ok(claims)
    }
}

/// Claims for Google ID tokens.
#[derive(Deserialize, Debug)]
struct Claims {
    email: String,
}

/// Google's public key used for decoding tokens.
#[derive(Deserialize)]
struct OuathKey {
    n: String,
    e: String,
}

/// Array of Google's public keys.
#[derive(Deserialize)]
struct OauthKeyList {
    // NOTE: Currently, Google always returns two keys. This is subject to change.
    keys: [OuathKey; 2],
}

impl OauthKeyList {
    /// Fetch Google's public keys.
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
            .await?
            .json::<Self>()
            .await?)
    }
}

pub struct Google;

impl OneTimeTokenAudience for Google {
    fn name() -> &'static str {
        "google"
    }

    fn from_str(s: &str) -> Result<(), Error> {
        match s {
            "google" => Ok(()),
            _ => Err(Error::Unauthorized),
        }
    }

    fn lifetime() -> Duration {
        Duration::seconds(60)
    }
}

/// The Google app's client ID.
static CLIENT_ID: Lazy<String> =
    Lazy::new(|| std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));

#[cfg(not(debug_assertions))]
// Use absolute namespace to silence errors about unused imports.
pub async fn login_page() -> axum::response::Response {
    axum::response::IntoResponse::into_response((
        axum::http::StatusCode::NOT_FOUND,
        "Not Found".to_string(),
    ))
}

#[cfg(debug_assertions)]
// Use absolute namespace to silence errors about unused imports.
pub async fn login_page() -> axum::response::Html<String> {
    axum::response::Html(format!(
        r##"
    <html>
        <body>
        <script src="https://accounts.google.com/gsi/client" async defer></script>
        <div
            id="g_id_onload"
            data-client_id="{}"
            data-login_uri="http://localhost:3000/google/login/client_individual?redirect_uri=http://localhost:3000/auth/keys"
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
        (*CLIENT_ID)
    ))
}
