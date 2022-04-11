use std::{collections::HashMap, marker::PhantomData};

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::ISSUER,
};
use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use user_management_microservice_sdk::user::UserType;

use crate::auth::{SharedDatabase, SharedState};

use super::KEYS;

/// Creates a one-time token.
pub fn create_one_time_token<T>(ulid: Ulid, user_type: UserType) -> GlobeliseResult<(String, i64)>
where
    T: OneTimeTokenAudience,
{
    let expiration = OffsetDateTime::now_utc()
        .unix_timestamp()
        .checked_add(T::lifetime().whole_seconds())
        .ok_or_else(|| {
            GlobeliseError::Internal(
                "Could not calculate one-time token expiration timestamp".into(),
            )
        })?;

    let claims = OneTimeToken::<T> {
        sub: ulid.to_string(),
        user_type: user_type.to_string(),
        aud: T::name().into(),
        iss: ISSUER.into(),
        exp: expiration as usize,
        one_time_audience: PhantomData,
    };

    Ok((
        encode(&Header::new(Algorithm::EdDSA), &claims, &KEYS.encoding)
            .map_err(|_| GlobeliseError::Internal("Failed to encode one-time token".into()))?,
        expiration,
    ))
}

/// Claims for one-time token
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeToken<T>
where
    T: OneTimeTokenAudience,
{
    pub sub: String,
    pub user_type: String,
    aud: String,
    iss: String,
    exp: usize,
    one_time_audience: PhantomData<T>,
}

impl<T> OneTimeToken<T>
where
    T: OneTimeTokenAudience,
{
    fn decode(input: &str) -> GlobeliseResult<Self> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_audience(&[T::name()]);
        validation.set_issuer(&[ISSUER]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } =
            decode::<OneTimeToken<T>>(input, &KEYS.decoding, &validation)
                .map_err(|_| GlobeliseError::Unauthorized("Failed to decode one-time token"))?;
        Ok(claims)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeTokenParam<T>(pub T);

impl<T> OneTimeTokenAudience for OneTimeTokenParam<T>
where
    T: OneTimeTokenAudience,
{
    fn name() -> &'static str {
        T::name()
    }

    fn lifetime() -> Duration {
        T::lifetime()
    }
}

#[async_trait]
impl<B, T> FromRequest<B> for OneTimeTokenParam<OneTimeToken<T>>
where
    B: Send,
    T: Send + OneTimeTokenAudience,
{
    type Rejection = GlobeliseError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Query(params) = Query::<HashMap<String, String>>::from_request(req)
            .await
            .map_err(|_| GlobeliseError::Unauthorized("No one-time token provided"))?;
        let Extension(database) = Extension::<SharedDatabase>::from_request(req)
            .await
            .map_err(|_| {
                GlobeliseError::Internal("Could not extract database from request".into())
            })?;
        let Extension(shared_state) =
            Extension::<SharedState>::from_request(req)
                .await
                .map_err(|_| {
                    GlobeliseError::Internal("Could not extract state store from request".into())
                })?;

        let token = params
            .get("token")
            .ok_or(GlobeliseError::Unauthorized("No one-time token provided"))?;

        let claims = OneTimeToken::<T>::decode(token)?;
        let ulid: Ulid = claims
            .sub
            .parse()
            .map_err(|_| GlobeliseError::Unauthorized("One-time token rejected: invalid ulid"))?;
        let user_type: UserType = claims
            .user_type
            .parse()
            .map_err(|_| GlobeliseError::Unauthorized("One-time token rejected: invalid role"))?;

        // Make sure the user actually exists.
        let database = database.lock().await;
        if database.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Unauthorized(
                "One-time token rejected: user does not exist",
            ));
        }

        // Do not authorize if the token has already been used.
        let mut shared_state = shared_state.lock().await;
        if shared_state
            .check_one_time_token_valid::<T>(ulid, token.as_bytes())
            .await?
        {
            Ok(OneTimeTokenParam(claims))
        } else {
            Err(GlobeliseError::Unauthorized(
                "One-time token rejected: invalid session",
            ))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeTokenBearer<T>(pub T);

impl<T> OneTimeTokenAudience for OneTimeTokenBearer<T>
where
    T: OneTimeTokenAudience,
{
    fn name() -> &'static str {
        T::name()
    }

    fn lifetime() -> Duration {
        T::lifetime()
    }
}

#[async_trait]
impl<B, T> FromRequest<B> for OneTimeTokenBearer<OneTimeToken<T>>
where
    B: Send,
    T: Send + OneTimeTokenAudience,
{
    type Rejection = GlobeliseError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| GlobeliseError::Unauthorized("No one-time token provided"))?;
        let Extension(database) = Extension::<SharedDatabase>::from_request(req)
            .await
            .map_err(|_| {
                GlobeliseError::Internal("Could not extract database from request".into())
            })?;
        let Extension(shared_state) =
            Extension::<SharedState>::from_request(req)
                .await
                .map_err(|_| {
                    GlobeliseError::Internal("Could not extract state store from request".into())
                })?;
        let claims = OneTimeToken::<T>::decode(bearer.token())?;
        let ulid: Ulid = claims
            .sub
            .parse()
            .map_err(|_| GlobeliseError::Unauthorized("One-time token rejected: invalid ulid"))?;
        let user_type: UserType = claims
            .user_type
            .parse()
            .map_err(|_| GlobeliseError::Unauthorized("One-time token rejected: invalid role"))?;

        // Make sure the user actually exists.
        let database = database.lock().await;
        if database.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Unauthorized(
                "One-time token rejected: user does not exist",
            ));
        }

        // Do not authorize if the token has already been used.
        let mut shared_state = shared_state.lock().await;
        if shared_state
            .check_one_time_token_valid::<T>(ulid, bearer.token().as_bytes())
            .await?
        {
            Ok(OneTimeTokenBearer(claims))
        } else {
            Err(GlobeliseError::Unauthorized(
                "One-time token rejected: invalid session",
            ))
        }
    }
}

pub trait OneTimeTokenAudience {
    fn name() -> &'static str;
    fn lifetime() -> Duration;
}
