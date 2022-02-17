//! Functions and types for handling authorization tokens.

use std::{fs::File, io::Read};

use argon2::verify_encoded;
use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
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

use super::{error::Error, user::Role, SharedState};

pub mod one_time;

/// Creates an access token.
pub fn create_access_token(ulid: Ulid, email: EmailAddress, role: Role) -> Result<String, Error> {
    let expiration = match OffsetDateTime::now_utc().checked_add(ACCESS_LIFETIME) {
        Some(datetime) => datetime.unix_timestamp(),
        None => return Err(Error::Internal),
    };

    let claims = AccessToken {
        sub: ulid.to_string(),
        email: email.into(),
        role: role.to_string(),
        iss: ISSSUER.into(),
        exp: expiration as usize,
    };
    encode(&Header::new(Algorithm::RS256), &claims, &KEYS.encoding).map_err(|_| Error::Internal)
}

/// Creates a refresh token.
pub fn create_refresh_token(ulid: Ulid, role: Role) -> Result<(String, i64), Error> {
    let expiration = match OffsetDateTime::now_utc().checked_add(REFRESH_LIFETIME) {
        Some(datetime) => datetime.unix_timestamp(),
        None => return Err(Error::Internal),
    };

    let claims = RefreshToken {
        sub: ulid.to_string(),
        role: role.to_string(),
        aud: "refresh_token".into(),
        iss: ISSSUER.into(),
        exp: expiration as usize,
    };

    Ok((
        encode(&Header::new(Algorithm::RS256), &claims, &KEYS.encoding)
            .map_err(|_| Error::Internal)?,
        expiration,
    ))
}

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    sub: String,
    pub email: String,
    role: String,
    iss: String,
    exp: usize,
}

/// Claims for refresh tokens.
#[derive(Deserialize, Serialize)]
pub struct RefreshToken {
    pub sub: String,
    pub role: String,
    aud: String,
    iss: String,
    exp: usize,
}

impl RefreshToken {
    pub fn decode(input: &str) -> Result<RefreshToken, Error> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["refresh_token"]);
        validation.set_issuer(&[ISSSUER]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } = decode::<RefreshToken>(input, &KEYS.decoding, &validation)
            .map_err(|_| Error::Unauthorized)?;
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
                .map_err(|_| Error::Unauthorized)?;
        let Extension(shared_state) = Extension::<SharedState>::from_request(req)
            .await
            .map_err(|_| Error::Internal)?;
        let claims = RefreshToken::decode(bearer.token())?;
        let ulid: Ulid = claims.sub.parse().map_err(|_| Error::Unauthorized)?;
        let role: Role = claims.role.parse().map_err(|_| Error::Unauthorized)?;

        // Make sure the user actually exists.
        let mut shared_state = shared_state.lock().await;
        if shared_state.user(ulid, role).await?.is_none() {
            return Err(Error::Unauthorized);
        }

        // Do not authorize if the server has revoked the session,
        // even if the token is otherwise valid.
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
            Err(Error::Unauthorized)
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
