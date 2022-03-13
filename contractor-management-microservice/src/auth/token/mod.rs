//! Functions and types for handling authorization tokens.

use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{HeaderMap, HeaderValue},
};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::ISSUER,
    DaprAppId,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use reqwest::Client;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use user_management_microservice_sdk::UserType;

use crate::env::GLOBELISE_USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken(pub String);

#[async_trait]
impl<B> FromRequest<B> for AccessToken
where
    B: Send,
{
    type Rejection = GlobeliseError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(AccessToken(bearer.token().to_owned()))
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or(GlobeliseError::Unauthorized(
                "Please provide access token in the query param",
            ))?;
            Ok(AccessToken(token.to_owned()))
        } else {
            Err(GlobeliseError::Unauthorized(
                "No valid access token provided",
            ))
        }
    }
}

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub email: String,
    pub user_type: String,
    iss: String,
    exp: usize,
}

impl AccessTokenClaims {
    async fn decode(input: &str) -> GlobeliseResult<Self> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&[ISSUER]);
        validation.set_required_spec_claims(&["iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } =
            decode::<AccessTokenClaims>(input, &UserManagementKey::new().await?.0, &validation)
                .map_err(|_| GlobeliseError::Unauthorized("Failed to decode access token"))?;

        claims
            .sub
            .parse::<Ulid>()
            .map_err(|_| GlobeliseError::Unauthorized("Access token rejected: invalid ulid"))?;
        claims.user_type.parse::<UserType>().map_err(|_| {
            GlobeliseError::Unauthorized("Access token rejected: invalid user type")
        })?;

        Ok(claims)
    }
}

#[async_trait]
impl<B> FromRequest<B> for AccessTokenClaims
where
    B: Send,
{
    type Rejection = GlobeliseError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(AccessTokenClaims::decode(bearer.token()).await?)
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or(GlobeliseError::Unauthorized(
                "Please provide access token in the query param",
            ))?;
            Ok(AccessTokenClaims::decode(token.as_str()).await?)
        } else {
            Err(GlobeliseError::Unauthorized(
                "No valid access token provided",
            ))
        }
    }
}

struct UserManagementKey(DecodingKey);

impl UserManagementKey {
    async fn new() -> GlobeliseResult<Self> {
        let key = USER_MANAGEMENT_KEY_CLIENT
            .get(&format!(
                "{}/auth/public-key",
                &*GLOBELISE_USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL
            ))
            .headers({
                let mut headers = HeaderMap::default();
                headers.insert(
                    "dapr-app-id",
                    HeaderValue::from_static(DaprAppId::UserManagementMicroservice.as_str()),
                );
                headers
            })
            .send()
            .await?
            .text()
            .await?;
        Ok(Self(DecodingKey::from_ed_pem(key.as_bytes())?))
    }
}

static USER_MANAGEMENT_KEY_CLIENT: Lazy<Client> = Lazy::new(Client::new);
