use std::collections::HashMap;

use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{
    error::GlobeliseResult,
    token::{Token, TokenString},
};
use reqwest::Client;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use user_management_microservice_sdk::{AccessToken, Role};

use crate::{database::SharedDatabase, env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL};

/// Lists all the users plus some information about them.
pub async fn user_index(
    TokenString(access_token): TokenString,
    Query(query): Query<HashMap<String, String>>,
    Extension(shared_client): Extension<Client>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let request = user_management_microservice_sdk::GetUserInfoRequest {
        page: query.get("page").map(|v| v.parse()).transpose()?,
        per_page: query.get("per_page").map(|v| v.parse()).transpose()?,
        search_text: query.get("search_text").map(|v| v.parse()).transpose()?,
        user_type: query.get("user_type").map(|v| v.parse()).transpose()?,
        user_role: query.get("user_role").map(|v| v.parse()).transpose()?,
    };

    let response = user_management_microservice_sdk::get_users_info(
        &shared_client,
        &*USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
        access_token,
        request,
    )
    .await?;

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

pub async fn contractor_index(
    access_token: Token<AccessToken>,
    Query(query): Query<ContractorIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorIndex>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>().unwrap();
    let database = database.lock().await;
    Ok(Json(database.contractor_index(ulid, query).await?))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub contract_count: i64,
    pub created_at: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractorIndexQuery {
    #[serde(default = "ContractorIndexQuery::default_page")]
    pub page: i64,
    #[serde(default = "ContractorIndexQuery::default_per_page")]
    pub per_page: i64,
    pub search_text: Option<String>,
}

impl ContractorIndexQuery {
    fn default_page() -> i64 {
        1
    }

    fn default_per_page() -> i64 {
        25
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ContractorIndex {
    #[sqlx(rename = "contractor_name")]
    pub name: String,
    pub contract_name: String,
    pub contract_status: String,
    pub job_title: String,
    pub seniority: String,
}
