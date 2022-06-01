use common_utils::{
    custom_serde::{EmailWrapper, OffsetDateWrapper},
    error::{GlobeliseError, GlobeliseResult},
    DaprAppId,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use super::user::{UserRole, UserType};

pub async fn eor_admin_onboarded_users(
    client: &Client,
    base_url: &str,
    access_token: String,
    request: GetUserInfoRequest,
) -> GlobeliseResult<Vec<OnboardedUserIndex>> {
    let response = client
        .get(format!("{base_url}/eor-admin/onboarded-users"))
        .headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                "dapr-app-id",
                HeaderValue::from_static(DaprAppId::UserManagementMicroservice.as_str()),
            );
            headers
        })
        .query(&request)
        .bearer_auth(access_token)
        .send()
        .await?;
    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(GlobeliseError::unauthorized(
            "Not authorised to make the request",
        )),
        _ => Err(GlobeliseError::Internal(response.status().to_string())),
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetUserInfoRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
}

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardedUserIndex {
    pub ulid: Uuid,
    pub name: String,
    pub user_role: UserRole,
    pub user_type: UserType,
    pub email: EmailWrapper,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserIndex {
    pub ulid: Uuid,
    pub email: EmailWrapper,
    pub user_type: UserType,
    pub is_google: bool,
    pub is_outlook: bool,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}
