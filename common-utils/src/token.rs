//! Functions and types for handling authorization tokens.

use std::{collections::HashMap, sync::Arc};

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use http_cache_reqwest::{Cache, CacheMode, HttpCache, MokaManager};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client as ReqwestClient, StatusCode,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::sync::Mutex;

use crate::{
    error::{GlobeliseError, GlobeliseResult},
    DaprAppId,
};

/// The issuer of tokens, used in the `iss` field of JWTs.
pub const ISSUER: &str = "https://globelise.com";

/// Stores the keys used for encoding and decoding tokens.
#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates a new encoding/decoding key pair from an RSA key pair.
    ///
    /// The private key must be in PEM form, and the public key in JWK form.
    pub fn new(private_key: &[u8], public_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_ed_pem(private_key).expect("Could not create encoding key"),
            decoding: DecodingKey::from_ed_pem(public_key).expect("Could not create decoding key"),
        }
    }
}

/// Creates an access token.
pub fn create_token<P>(payload: P, encoding: &EncodingKey) -> Result<(String, i64), GlobeliseError>
where
    P: std::fmt::Debug + Serialize + DeserializeOwned + TokenLike,
{
    let claims = Token::new(payload)?;
    let token = encode(&Header::new(Algorithm::EdDSA), &claims, encoding)
        .map_err(GlobeliseError::internal)?;
    Ok((token, claims.exp))
}

pub trait TokenLike {
    fn aud() -> &'static str;
    fn exp() -> Duration;
    fn dapr_app_id() -> DaprAppId;
}

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Token<P>
where
    P: TokenLike,
{
    pub payload: P,
    aud: String,
    iss: String,
    exp: i64,
}

impl<P> Token<P>
where
    P: TokenLike,
{
    pub fn new(payload: P) -> GlobeliseResult<Self> {
        let exp = OffsetDateTime::now_utc()
            .unix_timestamp()
            .checked_add(P::exp().whole_seconds())
            .ok_or_else(|| {
                GlobeliseError::Internal(
                    "Could not calculate access token expiration timestamp".into(),
                )
            })?;

        Ok(Token {
            payload,
            aud: P::aud().to_string(),
            iss: ISSUER.to_string(),
            exp,
        })
    }

    async fn decode<'e>(input: &'e str, decoding: &DecodingKey) -> Result<Self, GlobeliseError>
    where
        P: DeserializeOwned,
    {
        let validation = {
            let mut validation = Validation::new(Algorithm::EdDSA);
            validation.set_audience(&[P::aud()]);
            validation.set_issuer(&[ISSUER]);
            validation.set_required_spec_claims(&["aud", "iss", "exp"]);
            validation
        };

        let TokenData { claims, .. } = decode::<Token<P>>(input, decoding, &validation)
            .map_err(|e| GlobeliseError::unauthorized(e.to_string()))?;

        Ok(claims)
    }
}

#[async_trait]
impl<P, B> FromRequest<B> for Token<P>
where
    B: Send,
    P: std::fmt::Debug + Serialize + DeserializeOwned + TokenLike,
{
    type Rejection = GlobeliseError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(public_keys) = Extension::<SharedPublicKeys>::from_request(req).await?;
        let mut public_keys = public_keys.lock().await;
        let decoding_key = public_keys.get(P::dapr_app_id()).await?;
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(Token::decode(bearer.token(), decoding_key).await?)
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or_else(|| {
                GlobeliseError::unauthorized(
                    "Please provide access token in the query param or as auth bearer",
                )
            })?;
            Ok(Token::decode(token.as_str(), decoding_key).await?)
        } else {
            Err(GlobeliseError::unauthorized(
                "Please provide access token in the query param or as auth bearer",
            ))
        }
    }
}

/// HTTP client for public keys
static HTTP_CLIENT: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    ClientBuilder::new(ReqwestClient::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: Arc::new(MokaManager::default()),
            options: None,
        }))
        .build()
});

#[derive(Default)]
pub struct PublicKeys(std::collections::HashMap<DaprAppId, DecodingKey>);

pub type SharedPublicKeys = Arc<Mutex<PublicKeys>>;

impl PublicKeys {
    pub async fn get(&mut self, key: DaprAppId) -> GlobeliseResult<&DecodingKey> {
        let m_value = self.0.get(&key);
        if m_value.is_none() {
            let public_key_str = HTTP_CLIENT
                .get(&format!(
                    "{}/auth/public-key",
                    key.microservice_domain_url()?
                ))
                .headers({
                    let mut headers = HeaderMap::default();
                    headers.insert("dapr-app-id", HeaderValue::from_static(key.as_str()));
                    headers
                })
                .send()
                .await?
                .text()
                .await?;
            let decoding_key = DecodingKey::from_ed_pem(public_key_str.as_bytes())?;
            self.0.insert(key, decoding_key);
        }
        Ok(self.0.get(&key).unwrap())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthBearer(pub String);

#[async_trait]
impl<B> FromRequest<B> for AuthBearer
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> std::result::Result<Self, Self::Rejection> {
        // Get authorisation header
        let authorisation = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or((StatusCode::BAD_REQUEST, "`Authorization` header is missing"))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "`Authorization` header contains invalid characters",
                )
            })?;

        // Check that its a well-formed bearer and return
        let split = authorisation.split_once(' ');
        match split {
            Some((name, contents)) if name == "Bearer" => Ok(Self(contents.to_string())),
            _ => Err((
                StatusCode::BAD_REQUEST,
                "`Authorization` header must be a bearer token",
            )),
        }
    }
}
