//! Endpoint for handling Google authentication.

use axum::{
    extract::{Extension, Path, TypedHeader},
    headers::Cookie,
    Json,
};
use email_address::EmailAddress;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use rusty_ulid::Ulid;
use serde::Deserialize;

use super::{
    error::Error,
    user::{AuthMethod, Role, User},
    LoginResponse, SharedState,
};

/// Sign up as a Google user.
pub async fn create_account(
    id_token: String,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(role): Path<Role>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Json<LoginResponse>, Error> {
    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::BadRequest);
    }

    let id_token: IdToken =
        serde_urlencoded::from_str(&*id_token).map_err(|_| Error::Unauthorized)?;
    id_token.check_crsf_token(cookie)?;

    let OauthKeyList { keys } = OauthKeyList::new()
        .await
        .map_err(|_| Error::GooglePublicKeys)?;

    let claims = id_token.decode(&keys)?;
    let email: EmailAddress = claims.email.parse().unwrap(); // Google emails should be valid.
    let ulid = Ulid::generate();

    let mut shared_state = shared_state.lock().await;
    shared_state
        .create_user(ulid, email, AuthMethod::Google, role)
        .await?;
    let refresh_token = shared_state.open_session(ulid, role).await?;
    Ok(Json(LoginResponse { refresh_token }))
}

/// Log in as a Google user.
pub async fn login(
    id_token: String,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(role): Path<Role>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Json<LoginResponse>, Error> {
    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::BadRequest);
    }

    let id_token: IdToken =
        serde_urlencoded::from_str(&*id_token).map_err(|_| Error::Unauthorized)?;
    id_token.check_crsf_token(cookie)?;

    let OauthKeyList { keys } = OauthKeyList::new()
        .await
        .map_err(|_| Error::GooglePublicKeys)?;

    let claims = id_token.decode(&keys)?;
    let email: EmailAddress = claims.email.parse().unwrap(); // Google emails should be valid.

    let mut shared_state = shared_state.lock().await;
    if let Some(ulid) = shared_state.user_id(email, role).await? {
        if let Some(User {
            auth_method: AuthMethod::Google,
            ..
        }) = shared_state.user(ulid, role).await?
        {
            let refresh_token = shared_state.open_session(ulid, role).await?;
            return Ok(Json(LoginResponse { refresh_token }));
        }
        // TODO: Implement linking with an existing account.
    }

    Err(Error::Unauthorized)
}

/// Representation of Google's ID token.
#[derive(Deserialize)]
struct IdToken {
    credential: String,
    g_csrf_token: String,
}

impl IdToken {
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
#[derive(Deserialize)]
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

/// The Google app's client ID.
static CLIENT_ID: Lazy<String> =
    Lazy::new(|| std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));
