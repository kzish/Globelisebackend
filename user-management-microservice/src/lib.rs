#![allow(dead_code)]
#![allow(unused_variables)]

mod auth;
mod database;
mod env;
mod eor_admin;
mod error;
mod onboard;

use derive_builder::Builder;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};

pub use crate::{
    auth::user::{Role, UserType},
    eor_admin::UserIndex,
    error::Error,
};

#[derive(Default, Builder, Debug)]
pub struct GetUserInfoRequest {
    #[builder(setter(strip_option), default)]
    pub page: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub per_page: Option<u64>,
    #[builder(setter(strip_option), default)]
    pub search_text: Option<String>,
    #[builder(setter(strip_option), default)]
    pub user_type: Option<UserType>,
    #[builder(setter(strip_option), default)]
    pub user_role: Option<Role>,
}

pub async fn get_users_info(
    client: &Client,
    base_url: &str,
    access_token: String,
    request: GetUserInfoRequest,
) -> Result<Vec<UserIndex>, Error> {
    let mut query: Vec<(&'static str, String)> = vec![];
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
        .get(format!("{base_url}/eor-admin/users/index"))
        .headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                "dapr-app-id",
                HeaderValue::from_static("user-management-microservice"),
            );
            headers
        })
        .query(&query)
        .bearer_auth(access_token)
        .send()
        .await?;
    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        _ => Err(Error::Internal(response.status().to_string())),
    }
}
