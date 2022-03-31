use common_utils::{
    custom_serde::DateWrapper,
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

pub async fn get_users_info(
    client: &Client,
    base_url: &str,
    access_token: String,
    request: GetUserInfoRequest,
) -> GlobeliseResult<Vec<UserIndex>> {
    let mut query: Vec<(&'static str, String)> = vec![];
    // TODO: Turn this into a derive_macro
    if let Some(page) = request.page {
        query.push(("page", page.to_string()))
    }
    if let Some(per_page) = request.per_page {
        query.push(("per_page", per_page.to_string()))
    }
    if let Some(search_text) = request.search_text {
        query.push(("search_text", search_text))
    }
    if let Some(user_type) = request.user_type {
        query.push(("user_type", user_type.to_string()))
    }
    if let Some(user_role) = request.user_role {
        query.push(("user_role", user_role.to_string()))
    }
    let response = client
        .get(format!("{base_url}/eor-admin/users"))
        .headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                "dapr-app-id",
                HeaderValue::from_static(DaprAppId::UserManagementMicroservice.as_str()),
            );
            headers
        })
        .query(&query)
        .bearer_auth(access_token)
        .send()
        .await?;
    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(GlobeliseError::Unauthorized(
            "Not authorised to make the request",
        )),
        _ => Err(GlobeliseError::Internal(response.status().to_string())),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetUserInfoRequest {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<Role>,
}

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub user_role: Role,
    pub user_type: UserType,
    pub email: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub created_at: sqlx::types::time::Date,
}

impl<'r> FromRow<'r, PgRow> for UserIndex {
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
        Ok(UserIndex {
            ulid,
            name,
            user_role,
            user_type,
            email,
            created_at,
        })
    }
}
