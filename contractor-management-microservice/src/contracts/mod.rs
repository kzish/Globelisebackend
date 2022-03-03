use std::collections::HashMap;

use axum::{
    extract::{Extension, Query},
    Json,
};
use reqwest::Client;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use user_management_microservice_sdk::Role;

use crate::{
    auth::token::AccessToken, database::SharedDatabase,
    env::GLOBELISE_USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL, error::Error,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub contract_count: i64,
    pub created_at: String,
    pub email: String,
}

// Lists all the users plus some information about them.
pub async fn user_index(
    AccessToken(access_token): AccessToken,
    Query(query): Query<HashMap<String, String>>,
    Extension(shared_client): Extension<Client>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> Result<Json<Vec<UserIndex>>, Error> {
    let request = user_management_microservice_sdk::GetUserInfoRequest {
        page: query.get("page").map(|v| v.parse()).transpose()?,
        per_page: query.get("per_page").map(|v| v.parse()).transpose()?,
        search_text: query.get("search_text").map(|v| v.parse()).transpose()?,
        user_type: query.get("user_type").map(|v| v.parse()).transpose()?,
        user_role: query.get("user_role").map(|v| v.parse()).transpose()?,
    };

    let response = user_management_microservice_sdk::get_users_info(
        &shared_client,
        &*GLOBELISE_USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
        access_token,
        request,
    )
    .await
    .map_err(|_| {
        Error::Internal("Something wrong happened when trying to make request".to_string())
    })?;

    let mut result = Vec::with_capacity(response.len());

    let database = shared_database.lock().await;

    for v in response {
        let count = database
            .count_number_of_contracts(&v.ulid, &v.user_role)
            .await?;
        result.push(UserIndex {
            ulid: v.ulid,
            name: v.name,
            role: v.user_role,
            contract_count: count,
            created_at: v.created_at,
            email: v.email,
        })
    }
    Ok(Json(result))
}
