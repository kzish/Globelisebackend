use std::sync::Arc;

use common_utils::custom_serde::EmailWrapper;
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
#[serde(rename_all = "kebab-case")]
pub struct IdToken(pub String);

impl IdToken {
    /// Decode and validate the token.
    pub async fn decode_and_validate(&self, google_client_id: &str) -> Result<Claims> {
        let openid_configuration = OpenIdConfiguration::new().await?;
        let oauth_key_list = OauthKeyList::new(&openid_configuration.jwks_uri).await?;
        let key = self.identify_key(&oauth_key_list.keys)?;

        if &key.kty != "RSA" {
            return Err(Error::NotSupported(key.kty.clone()));
        }

        let mut validation = Validation::new(key.alg);
        validation.set_audience(&[google_client_id]);
        validation.set_issuer(&["accounts.google.com"]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } = decode::<Claims>(
            &*self.0,
            &DecodingKey::from_rsa_components(&*key.n, &*key.e)
                .map_err(|e| Error::Decoding(format!("{}", e)))?,
            &validation,
        )
        .map_err(|e| Error::Decoding(format!("{}", e)))?;

        Ok(claims)
    }

    /// Pick the correct key used for the token.
    fn identify_key<'a>(&self, keys: &'a [OauthKey]) -> Result<&'a OauthKey> {
        let header = decode_header(&*self.0).map_err(|e| Error::Decoding(format!("{}", e)))?;
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
#[serde(rename_all = "kebab-case")]
pub struct Claims {
    pub email: EmailWrapper,
    pub aud: String,
    pub iss: String,
}

/// Google's public key used for decoding tokens.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct OauthKey {
    kty: String,
    alg: Algorithm,
    kid: String,
    n: String,
    e: String,
}

/// Array of Google's public keys.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct OauthKeyList {
    keys: Vec<OauthKey>,
}

impl OauthKeyList {
    /// Fetch Google's public keys.
    async fn new(jwks_uri: &str) -> Result<Self> {
        OAUTH_KEY_CLIENT
            .get(jwks_uri)
            .send()
            .await
            .map_err(|e| Error::FetchPublicKeys(format!("{}", e)))?
            .json::<Self>()
            .await
            .map_err(|e| Error::FetchPublicKeys(format!("{}", e)))
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

/// Array of Google's public keys.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct OpenIdConfiguration {
    #[serde(rename = "jwks_uri")]
    jwks_uri: String,
}

impl OpenIdConfiguration {
    /// Fetch Google's public keys.
    async fn new() -> Result<Self> {
        OPENID_CONFIGURATION
            .get("https://accounts.google.com/.well-known/openid-configuration")
            .send()
            .await
            .map_err(|e| Error::FetchPublicKeys(format!("{}", e)))?
            .json::<Self>()
            .await
            .map_err(|e| Error::FetchPublicKeys(format!("{}", e)))
    }
}

/// HTTP client for fetching and caching Google's public keys.
static OPENID_CONFIGURATION: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: Arc::new(MokaManager::default()),
            options: None,
        }))
        .build()
});
