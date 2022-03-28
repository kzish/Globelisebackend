//! Endpoint for handling Google authentication.

use std::sync::Arc;

use axum::extract::{Extension, Form};
use common_utils::error::{GlobeliseError, GlobeliseResult};
use email_address::EmailAddress;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaManager};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::Deserialize;

use super::{admin::Admin, SharedDatabase, SharedState};

/// Sign up as an admin through Google sign-in.
pub async fn signup(
    Form(id_token): Form<IdToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<String> {
    let OauthKeyList { keys } = OauthKeyList::new()
        .await
        .map_err(|_| GlobeliseError::Internal("Could not get Google's public keys".into()))?;

    let claims = id_token.decode(&keys)?;
    let email: EmailAddress = claims.email.parse().unwrap(); // Google emails should be valid.

    let admin = Admin {
        email,
        password_hash: None,
        google: true,
        outlook: false,
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
    let OauthKeyList { keys } = OauthKeyList::new()
        .await
        .map_err(|_| GlobeliseError::Internal("Could not get Google's public keys".into()))?;

    let claims = id_token.decode(&keys)?;
    let email: EmailAddress = claims.email.parse().unwrap(); // Google emails should be valid.

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;
    if let Some(ulid) = database.admin_id(&email).await? {
        if let Some(Admin { google: true, .. }) = database.admin(ulid).await? {
            let refresh_token = shared_state.open_session(&database, ulid).await?;

            Ok(refresh_token)
        } else {
            // TODO: Implement linking with an existing account.
            Err(GlobeliseError::Unauthorized(
                "Linking Google with existing account is not implemented",
            ))
        }
    } else {
        Err(GlobeliseError::Unauthorized("Google login failed"))
    }
}

/// Representation of Google's ID token.
#[derive(Debug, Deserialize)]
pub struct IdToken {
    credential: String,
}

impl IdToken {
    /// Decode and validate the token.
    fn decode(&self, keys: &[OauthKey]) -> GlobeliseResult<Claims> {
        let key = self.identify_key(keys)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[(*CLIENT_ID).clone()]);
        validation.set_issuer(&["https://accounts.google.com"]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } = decode::<Claims>(
            &*self.credential,
            &DecodingKey::from_rsa_components(&*key.n, &*key.e).map_err(|_| {
                GlobeliseError::Internal("Could not create decoding key for Google ID token".into())
            })?,
            &validation,
        )
        .map_err(|_| GlobeliseError::Unauthorized("Failed to decode Google ID token"))?;

        Ok(claims)
    }

    /// Pick the correct key used for the token.
    fn identify_key<'a>(&self, keys: &'a [OauthKey]) -> GlobeliseResult<&'a OauthKey> {
        let header = decode_header(&*self.credential)
            .map_err(|_| GlobeliseError::Unauthorized("Failed to decode Google ID token header"))?;
        let key_id = header.kid.ok_or(GlobeliseError::Unauthorized(
            "Google ID token header has no key id",
        ))?;

        for key in keys {
            if key.kid == key_id {
                return Ok(key);
            }
        }

        Err(GlobeliseError::Unauthorized(
            "Could not identify correct key for Google ID token",
        ))
    }
}

/// Claims for Google ID tokens.
#[derive(Debug, Deserialize)]
struct Claims {
    email: String,
}

/// Google's public key used for decoding tokens.
#[derive(Deserialize)]
struct OauthKey {
    kid: String,
    n: String,
    e: String,
}

/// Array of Google's public keys.
#[derive(Deserialize)]
struct OauthKeyList {
    keys: Vec<OauthKey>,
}

impl OauthKeyList {
    /// Fetch Google's public keys.
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(OAUTH_KEY_CLIENT
            .get("https://www.googleapis.com/oauth2/v3/certs")
            .send()
            .await?
            .json::<Self>()
            .await?)
    }
}

/// The Google app's client ID.
static CLIENT_ID: Lazy<String> =
    Lazy::new(|| std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));

/// HTTP client for fetching and caching Google's public keys.
static OAUTH_KEY_CLIENT: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: Arc::new(MokaManager::default()),
            options: None,
        }))
        .build()
});
