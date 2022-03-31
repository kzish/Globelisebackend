use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    DaprAppId,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};
use serde::{Deserialize, Serialize};

pub mod token;
pub mod user;
pub mod user_index;

use user::{Role, UserType};
use user_index::UserIndex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetUserInfoRequest {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<Role>,
}

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
