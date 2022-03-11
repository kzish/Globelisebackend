//! Functions and types for handling authorization tokens.

use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::error::{GlobeliseError, GlobeliseResult};

/// The issuer of tokens, used in the `iss` field of JWTs.
pub const ISSUER: &str = "https://globelise.com";

/// Stores the keys used for encoding and decoding tokens.
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
        .map_err(|_| GlobeliseError::Internal("Failed to encode access token".into()))?;
    Ok((token, claims.exp))
}

/// Creates an access token.
pub fn create_one_time_token<P>(
    payload: P,
    encoding: &EncodingKey,
) -> Result<(String, i64), GlobeliseError>
where
    P: std::fmt::Debug + Serialize + DeserializeOwned + TokenLike + OneTimeTokenLike,
{
    let claims = Token::new(payload)?;
    let token = encode(&Header::new(Algorithm::EdDSA), &claims, encoding)
        .map_err(|_| GlobeliseError::Internal("Failed to encode access token".into()))?;
    Ok((token, claims.exp))
}

pub trait TokenLike {
    fn aud() -> &'static str;
    fn exp() -> Duration;
}

pub trait OneTimeTokenLike {}

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
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
        let exp = match OffsetDateTime::now_utc().checked_add(P::exp()) {
            Some(datetime) => datetime.unix_timestamp(),
            None => {
                return Err(GlobeliseError::Internal(
                    "Could not calculate access token expiration timestamp".into(),
                ))
            }
        };

        Ok(Token {
            payload,
            aud: P::aud().to_string(),
            iss: ISSUER.to_string(),
            exp,
        })
    }

    async fn decode<'e>(input: &'e str, decoding: DecodingKey) -> Result<Self, GlobeliseError>
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

        let TokenData { claims, .. } = decode::<Token<P>>(input, &decoding, &validation)
            .map_err(|_| GlobeliseError::Unauthorized("Failed to decode access token"))?;

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
        let Extension(decoding_key) =
            Extension::<DecodingKey>::from_request(req)
                .await
                .map_err(|_| {
                    GlobeliseError::Internal("Could not extract database from request".into())
                })?;
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(Token::decode(bearer.token(), decoding_key).await?)
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or(GlobeliseError::Unauthorized(
                "Please provide access token in the query param",
            ))?;
            Ok(Token::decode(token.as_str(), decoding_key).await?)
        } else {
            Err(GlobeliseError::Unauthorized(
                "No valid access token provided",
            ))
        }
    }
}
