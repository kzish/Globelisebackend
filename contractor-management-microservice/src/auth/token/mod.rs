//! Functions and types for handling authorization tokens.

use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequest, Query, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken(pub String);

#[async_trait]
impl<B> FromRequest<B> for AccessToken
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await
        {
            Ok(AccessToken(bearer.token().to_owned()))
        } else if let Ok(Query(param)) = Query::<HashMap<String, String>>::from_request(req).await {
            let token = param.get("token").ok_or(Error::Unauthorized(
                "Please provide access token in the query param",
            ))?;
            Ok(AccessToken(token.to_owned()))
        } else {
            Err(Error::Unauthorized("No valid access token provided"))
        }
    }
}
