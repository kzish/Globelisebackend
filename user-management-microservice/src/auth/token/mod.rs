//! Functions and types for handling authorization tokens.

use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

use axum::{
    async_trait,
    extract::{FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use common_utils::token::{TokenLike, ISSUER};
use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaManager};
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::{env::EOR_ADMIN_MICROSERVICE_DOMAIN_URL, error::Error};

pub mod one_time;

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub ulid: String,
    pub email: String,
    pub user_type: String,
}

impl TokenLike for AccessToken {
    fn aud() -> &'static str {
        "access_token"
    }

    fn exp() -> Duration {
        Duration::minutes(60)
    }
}

/// Claims for refresh tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    pub ulid: String,
    pub user_type: String,
}

impl TokenLike for RefreshToken {
    fn aud() -> &'static str {
        "refresh_token"
    }

    fn exp() -> Duration {
        Duration::minutes(120)
    }
}

/// Stores the keys used for encoding and decoding tokens.
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates a new encoding/decoding key pair from an Ed25519 key pair.
    ///
    /// The keys must be in PEM form.
    fn new(private_key: &[u8], public_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_ed_pem(private_key).expect("Could not create encoding key"),
            decoding: DecodingKey::from_ed_pem(public_key).expect("Could not create decoding key"),
        }
    }
}

/// The public key used for decoding tokens.
pub static PUBLIC_KEY: Lazy<String> = Lazy::new(|| {
    let mut public_key = String::new();
    File::open("user-management-microservice/public.pem")
        .expect("Could not open public key")
        .read_to_string(&mut public_key)
        .expect("Could not read public key");
    public_key
});

/// The encoding/decoding key pair.
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let mut private_key: Vec<u8> = Vec::new();
    File::open("user-management-microservice/private.pem")
        .expect("Could not open private key")
        .read_to_end(&mut private_key)
        .expect("Could not read private key");
    Keys::new(&private_key, PUBLIC_KEY.as_bytes())
});

/// HTTP Cache EOR admin microservice public key
static EOR_ADMIN_PUBLIC_KEY: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    ClientBuilder::new(ReqwestClient::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: Arc::new(MokaManager::default()),
            options: None,
        }))
        .build()
});

struct PublicKey(DecodingKey);

impl PublicKey {
    /// Fetch Google's public keys.
    async fn new() -> Result<Self, Error> {
        let key = EOR_ADMIN_PUBLIC_KEY
            .get(&format!(
                "{}/auth/public-key",
                &*EOR_ADMIN_MICROSERVICE_DOMAIN_URL
            ))
            .headers({
                let mut headers = HeaderMap::default();
                headers.insert(
                    "dapr-app-id",
                    HeaderValue::from_static("eor-admin-microservice"),
                );
                headers
            })
            .send()
            .await?
            .text()
            .await?;
        Ok(PublicKey(DecodingKey::from_ed_pem(key.as_bytes())?))
    }
}

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdminAccessToken {
    pub sub: String,
    pub email: String,
    iss: String,
    exp: usize,
}

impl AdminAccessToken {
    async fn decode(input: &str) -> Result<Self, Error> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&[ISSUER]);
        validation.set_required_spec_claims(&["iss", "exp"]);
        let validation = validation;

        let public_key = PublicKey::new().await?;

        let TokenData { claims, .. } =
            decode::<AdminAccessToken>(input, &public_key.0, &validation)
                .map_err(|_| Error::Unauthorized("Failed to decode access token"))?;

        // TODO: Check that the access token still points to a valid admin?

        Ok(claims)
    }
}

#[async_trait]
impl<B> FromRequest<B> for AdminAccessToken
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // TODO: Add retry the decoding with a new public key if it fails.
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(AdminAccessToken::decode(bearer.token()).await?)
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or(Error::Unauthorized(
                "Please provide access token in the query param",
            ))?;
            Ok(AdminAccessToken::decode(token.as_str()).await?)
        } else {
            Err(Error::Unauthorized("No valid access token provided"))
        }
    }
}
