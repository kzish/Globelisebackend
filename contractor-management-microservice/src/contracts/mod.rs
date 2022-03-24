use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{
    error::GlobeliseResult,
    token::{Token, TokenString},
};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use reqwest::Client;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use user_management_microservice_sdk::{AccessToken as UserAccessToken, GetUserInfoRequest, Role};

use crate::{
    common::PaginationQuery, database::SharedDatabase, env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
};

/// Lists all the users plus some information about them.
pub async fn user_index(
    TokenString(access_token): TokenString,
    Query(request): Query<GetUserInfoRequest>,
    Extension(shared_client): Extension<Client>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
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
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorIndex>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>()?;
    let database = database.lock().await;
    Ok(Json(database.contractor_index(ulid, query).await?))
}

pub async fn contract_for_contractor_index(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractForContractorIndex>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>()?;
    let database = database.lock().await;
    Ok(Json(
        database.contract_for_contractor_index(ulid, query).await?,
    ))
}

pub async fn contract_for_client_index(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractForClientIndex>>> {
    let ulid = access_token.payload.ulid.parse::<Ulid>()?;
    let database = database.lock().await;
    Ok(Json(database.contract_for_client_index(ulid, query).await?))
}

pub async fn eor_admin_contract_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginationQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractForClientIndex>>> {
    let database = database.lock().await;
    Ok(Json(database.eor_admin_contract_index(query).await?))
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ContractorIndex {
    #[sqlx(rename = "contractor_name")]
    pub name: String,
    pub contract_name: String,
    pub contract_status: String,
    pub job_title: String,
    pub seniority: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ContractForContractorIndex {
    pub contractor_ulid: String,
    pub contract_name: String,
    pub job_title: String,
    pub seniority: String,
    pub client_name: String,
    pub contract_status: String,
    pub contract_amount: String,
    pub end_at: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ContractForClientIndex {
    pub client_ulid: String,
    pub contract_name: String,
    pub job_title: String,
    pub seniority: String,
    pub contractor_name: String,
    pub contract_status: String,
    pub contract_amount: String,
    pub end_at: String,
}
