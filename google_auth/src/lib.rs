use std::sync::Arc;

use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaManager};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::Deserialize;

pub mod error;

pub use error::Error;
use error::Result;

/// Representation of Google's ID token.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdToken {
    credential: String,
}

impl IdToken {
    /// Decode and validate the token.
    pub async fn decode(&self, client_id: &str) -> Result<Claims> {
        let OauthKeyList { keys } = OauthKeyList::new().await?;
        let key = self.identify_key(&keys)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[client_id]);
        validation.set_issuer(&["https://accounts.google.com"]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } = decode::<Claims>(
            &*self.credential,
            &DecodingKey::from_rsa_components(&*key.n, &*key.e).map_err(|_| {
                Error::Decoding("could not create decoding key from Google public key".into())
            })?,
            &validation,
        )
        .map_err(|_| Error::Decoding("could not decode JWT".into()))?;

        Ok(claims)
    }

    /// Pick the correct key used for the token.
    fn identify_key<'a>(&self, keys: &'a [OauthKey]) -> Result<&'a OauthKey> {
        let header = decode_header(&*self.credential)
            .map_err(|_| Error::Decoding("could not decode JWT header".into()))?;
        let key_id = header.kid.ok_or(Error::MissingKeyId)?;

        for key in keys {
            if key.kid == key_id {
                return Ok(key);
            }
        }

        Err(Error::InvalidKeyId)
    }
}

/// Claims for Google ID tokens.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub email: String,
}

/// Google's public key used for decoding tokens.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OauthKey {
    kid: String,
    n: String,
    e: String,
}

/// Array of Google's public keys.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OauthKeyList {
    keys: Vec<OauthKey>,
}

impl OauthKeyList {
    /// Fetch Google's public keys.
    async fn new() -> Result<Self> {
        OAUTH_KEY_CLIENT
            .get("https://www.googleapis.com/oauth2/v3/certs")
            .send()
            .await
            .map_err(|_| Error::FetchPublicKeys)?
            .json::<Self>()
            .await
            .map_err(|_| Error::FetchPublicKeys)
    }
}

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
