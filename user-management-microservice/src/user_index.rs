use common_utils::{
    custom_serde::OffsetDateWrapper,
    error::{GlobeliseError, GlobeliseResult},
    ulid_from_sql_uuid, DaprAppId,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};

use super::user::{Role, UserType};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetUserInfoRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<Role>,
}

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardedUserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub user_role: Role,
    pub user_type: UserType,
    pub email: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}

impl<'r> FromRow<'r, PgRow> for OnboardedUserIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let name = row.try_get("name")?;
        let role_str: String = row.try_get("user_role")?;
        let user_role =
            Role::try_from(role_str.as_str()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let type_str: String = row.try_get("user_type")?;
        let user_type =
            UserType::try_from(type_str.as_str()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let email = row.try_get("email")?;
        let created_at = row.try_get("created_at")?;
        Ok(OnboardedUserIndex {
            ulid,
            name,
            user_role,
            user_type,
            email,
            created_at,
        })
    }
}

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserIndex {
    pub ulid: Ulid,
    pub email: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}

impl<'r> FromRow<'r, PgRow> for UserIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let email = row.try_get("email")?;
        let created_at = row.try_get("created_at")?;
        Ok(UserIndex {
            ulid,
            email,
            created_at,
        })
    }
}
