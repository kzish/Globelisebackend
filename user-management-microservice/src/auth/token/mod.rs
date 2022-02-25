//! Functions and types for handling authorization tokens.

use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

use argon2::verify_encoded;
use axum::{
    async_trait,
    extract::{Extension, FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use email_address::EmailAddress;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::sync::Mutex;

use crate::{database::Database, error::Error};

use super::{user::UserType, SharedDatabase, SharedState};

pub mod one_time;

/// Creates an access token.
pub fn create_access_token(
    ulid: Ulid,
    email: EmailAddress,
    user_type: UserType,
) -> Result<String, Error> {
    let expiration = match OffsetDateTime::now_utc().checked_add(ACCESS_LIFETIME) {
        Some(datetime) => datetime.unix_timestamp(),
        None => {
            return Err(Error::Internal(
                "Could not calculate access token expiration timestamp".into(),
            ))
        }
    };

    let claims = AccessToken {
        sub: ulid.to_string(),
        email: email.into(),
        user_type: user_type.to_string(),
        iss: ISSSUER.into(),
        exp: expiration as usize,
    };
    encode(&Header::new(Algorithm::RS256), &claims, &KEYS.encoding)
        .map_err(|_| Error::Internal("Failed to encode access token".into()))
}

/// Creates a refresh token.
pub fn create_refresh_token(ulid: Ulid, user_type: UserType) -> Result<(String, i64), Error> {
    let expiration = match OffsetDateTime::now_utc().checked_add(REFRESH_LIFETIME) {
        Some(datetime) => datetime.unix_timestamp(),
        None => {
            return Err(Error::Internal(
                "Could not calculate refresh token expiration timestamp".into(),
            ))
        }
    };

    let claims = RefreshToken {
        sub: ulid.to_string(),
        user_type: user_type.to_string(),
        aud: "refresh_token".into(),
        iss: ISSSUER.into(),
        exp: expiration as usize,
    };

    Ok((
        encode(&Header::new(Algorithm::RS256), &claims, &KEYS.encoding)
            .map_err(|_| Error::Internal("Failed to encode refresh token".into()))?,
        expiration,
    ))
}

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub sub: String,
    pub email: String,
    pub user_type: String,
    iss: String,
    exp: usize,
}

impl AccessToken {
    async fn decode(input: &str, database: Arc<Mutex<Database>>) -> Result<Self, Error> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[ISSSUER]);
        validation.set_required_spec_claims(&["iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } = decode::<AccessToken>(input, &KEYS.decoding, &validation)
            .map_err(|_| Error::Unauthorized("Failed to decode access token"))?;

        let ulid: Ulid = claims
            .sub
            .parse()
            .map_err(|_| Error::Unauthorized("Access token rejected: invalid ulid"))?;
        let user_type: UserType = claims
            .user_type
            .parse()
            .map_err(|_| Error::Unauthorized("Access token rejected: invalid role"))?;

        // Make sure the user actually exists.
        let database = database.lock().await;
        if database.user(ulid, Some(user_type)).await?.is_none() {
            return Err(Error::Unauthorized(
                "Access token rejected: user does not exist",
            ));
        }

        Ok(claims)
    }
}

#[async_trait]
impl<B> FromRequest<B> for AccessToken
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(database) = Extension::<SharedDatabase>::from_request(req)
            .await
            .map_err(|_| Error::Internal("Could not extract database from request".into()))?;
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(AccessToken::decode(bearer.token(), database).await?)
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or(Error::Unauthorized(
                "Please provide access token in the query param",
            ))?;
            Ok(AccessToken::decode(token.as_str(), database).await?)
        } else {
            Err(Error::Unauthorized("No valid access token provided"))
        }
    }
}

/// Claims for refresh tokens.
#[derive(Deserialize, Serialize)]
pub struct RefreshToken {
    pub sub: String,
    pub user_type: String,
    aud: String,
    iss: String,
    exp: usize,
}

impl RefreshToken {
    fn decode(input: &str) -> Result<Self, Error> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["refresh_token"]);
        validation.set_issuer(&[ISSSUER]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } =
            decode::<RefreshToken>(input, &KEYS.decoding, &validation)
                .map_err(|_| Error::Unauthorized("Failed to decode refresh token"))?;
        Ok(claims)
    }
}

#[async_trait]
impl<B> FromRequest<B> for RefreshToken
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| Error::Unauthorized("No refresh token provided"))?;
        let Extension(database) = Extension::<SharedDatabase>::from_request(req)
            .await
            .map_err(|_| Error::Internal("Could not extract database from request".into()))?;
        let Extension(shared_state) = Extension::<SharedState>::from_request(req)
            .await
            .map_err(|_| Error::Internal("Could not extract state store from request".into()))?;
        let claims = RefreshToken::decode(bearer.token())?;
        let ulid: Ulid = claims
            .sub
            .parse()
            .map_err(|_| Error::Unauthorized("Refresh token rejected: invalid ulid"))?;
        let user_type: UserType = claims
            .user_type
            .parse()
            .map_err(|_| Error::Unauthorized("Refresh token rejected: invalid role"))?;

        // Make sure the user actually exists.
        let database = database.lock().await;
        if database.user(ulid, Some(user_type)).await?.is_none() {
            return Err(Error::Unauthorized(
                "Refresh token rejected: user does not exist",
            ));
        }

        // Do not authorize if the server has revoked the session,
        // even if the token is otherwise valid.
        let mut shared_state = shared_state.lock().await;
        let mut is_session_valid = false;
        let _ = shared_state.clear_expired_sessions(ulid).await;
        if let Some(sessions) = shared_state.sessions(ulid).await? {
            for (hash, _) in sessions.iter() {
                if let Ok(true) = verify_encoded(hash, bearer.token().as_bytes()) {
                    is_session_valid = true;
                    break;
                }
            }
        }

        if is_session_valid {
            Ok(claims)
        } else {
            Err(Error::Unauthorized(
                "Refresh token rejected: invalid session",
            ))
        }
    }
}

/// Stores the keys used for encoding and decoding tokens.
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates a new encoding/decoding key pair from an RSA key pair.
    ///
    /// The private key must be in PEM form, and the public key in JWK form.
    fn new(private_key: &[u8], public_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_rsa_pem(private_key)
                .expect("Could not create encoding key"),
            decoding: DecodingKey::from_rsa_pem(public_key).expect("Could not create decoding key"),
        }
    }
}

/// The public key used for decoding tokens.
pub static PUBLIC_KEY: Lazy<String> = Lazy::new(|| {
    let mut public_key = String::new();
    File::open("public.pem")
        .expect("Could not open public key")
        .read_to_string(&mut public_key)
        .expect("Could not read public key");
    public_key
});

/// The encoding/decoding key pair.
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let mut private_key: Vec<u8> = Vec::new();
    File::open("private.pem")
        .expect("Could not open private key")
        .read_to_end(&mut private_key)
        .expect("Could not read private key");
    Keys::new(&private_key, PUBLIC_KEY.as_bytes())
});

/// The issuer of tokens, used in the `iss` field of JWTs.
pub const ISSSUER: &str = "https://globelise.com";

/// Lifetime of access tokens.
const ACCESS_LIFETIME: Duration = Duration::hours(1);

/// Lifetime of refresh tokens.
const REFRESH_LIFETIME: Duration = Duration::hours(2);
