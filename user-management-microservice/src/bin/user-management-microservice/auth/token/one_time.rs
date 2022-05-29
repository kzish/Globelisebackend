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
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use user_management_microservice_sdk::user::UserType;
use uuid::Uuid;

use crate::auth::{SharedDatabase, SharedState};

use super::KEYS;

/// Creates a one-time token.
pub fn create_one_time_token<T>(ulid: Uuid, user_type: UserType) -> GlobeliseResult<(String, i64)>
where
    T: OneTimeTokenAudience,
{
    let expiration = OffsetDateTime::now_utc()
        .unix_timestamp()
        .checked_add(T::lifetime().whole_seconds())
        .ok_or_else(|| {
            GlobeliseError::internal("Could not calculate one-time token expiration timestamp")
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
            .map_err(GlobeliseError::internal)?,
        expiration,
    ))
}

/// Claims for one-time token
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
                .map_err(GlobeliseError::unauthorized)?;
        Ok(claims)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
            .map_err(GlobeliseError::internal)?;
        let Extension(database) = Extension::<SharedDatabase>::from_request(req)
            .await
            .map_err(GlobeliseError::internal)?;
        let Extension(shared_state) = Extension::<SharedState>::from_request(req)
            .await
            .map_err(GlobeliseError::internal)?;

        let token = params
            .get("token")
            .ok_or_else(|| GlobeliseError::unauthorized("No one-time token provided"))?;

        let claims = OneTimeToken::<T>::decode(token)?;
        let ulid: Uuid = claims.sub.parse().map_err(GlobeliseError::unauthorized)?;
        let user_type: UserType = claims
            .user_type
            .parse()
            .map_err(GlobeliseError::unauthorized)?;

        // Make sure the user actually exists.
        let database = database.lock().await;
        if database
            .find_one_user(ulid, Some(user_type))
            .await?
            .is_none()
        {
            return Err(GlobeliseError::unauthorized(
                "Cannot create one time token because the user does not exist",
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
            Err(GlobeliseError::unauthorized(
                "One-time token rejected: invalid session",
            ))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
                .map_err(GlobeliseError::internal)?;
        let Extension(database) = Extension::<SharedDatabase>::from_request(req)
            .await
            .map_err(GlobeliseError::internal)?;
        let Extension(shared_state) = Extension::<SharedState>::from_request(req)
            .await
            .map_err(GlobeliseError::internal)?;
        let claims = OneTimeToken::<T>::decode(bearer.token())?;
        let ulid: Uuid = claims.sub.parse().map_err(GlobeliseError::internal)?;
        let user_type: UserType = claims.user_type.parse().map_err(GlobeliseError::internal)?;

        // Make sure the user actually exists.
        let database = database.lock().await;
        if database
            .find_one_user(ulid, Some(user_type))
            .await?
            .is_none()
        {
            return Err(GlobeliseError::unauthorized(
                "Cannot create one time token because the user does not exist",
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
            Err(GlobeliseError::unauthorized(
                "One-time token rejected: invalid session",
            ))
        }
    }
}

pub trait OneTimeTokenAudience {
    fn name() -> &'static str;
    fn lifetime() -> Duration;
}
