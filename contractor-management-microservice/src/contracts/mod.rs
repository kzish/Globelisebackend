use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Currency, EmailWrapper, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::{Token, TokenString},
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{UserRole, UserType},
    user_index::GetUserInfoRequest,
};
use uuid::Uuid;

use crate::{
    common::PaginatedQuery, database::SharedDatabase, env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
};

mod database;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetOneContractQuery {
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
}

/// Lists all the users plus some information about them.
pub async fn eor_admin_user_index(
    TokenString(access_token): TokenString,
    Query(request): Query<GetUserInfoRequest>,
    Extension(shared_client): Extension<Client>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let response = user_management_microservice_sdk::user_index::eor_admin_onboarded_users(
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
            .count_number_of_contracts(v.ulid, v.user_role)
            .await?;
        result.push(UserIndex {
            ulid: v.ulid,
            name: v.name,
            r#type: v.user_type,
            role: v.user_role,
            contract_count: count,
            created_at: v.created_at,
            email: v.email,
        })
    }
    Ok(Json(result))
}

pub async fn get_many_clients_for_contractors(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ClientsIndex>>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .select_many_clients_for_contractors(access_token.payload.ulid, query)
            .await?,
    ))
}

pub async fn get_many_contractors_for_clients(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorsIndex>>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .select_many_contractors_for_clients(access_token.payload.ulid, query)
            .await?,
    ))
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyContractsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
}

pub async fn contracts_index(
    claims: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndex>>> {
    let database = database.lock().await;
    let results = match role {
        UserRole::Client => {
            database
                .select_many_contracts(
                    query.page,
                    query.per_page,
                    query.query,
                    query.contractor_ulid,
                    Some(claims.payload.ulid),
                    query.branch_ulid,
                )
                .await?
        }
        UserRole::Contractor => {
            database
                .select_many_contracts(
                    query.page,
                    query.per_page,
                    query.query,
                    Some(claims.payload.ulid),
                    query.client_ulid,
                    query.branch_ulid,
                )
                .await?
        }
    };

    Ok(Json(results))
}

pub async fn admin_get_many_contract_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndex>>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .select_many_contracts(
                query.per_page,
                query.per_page,
                query.query,
                query.contractor_ulid,
                query.client_ulid,
                query.branch_ulid,
            )
            .await?,
    ))
}

pub async fn admin_get_one_contract_index(
    _: Token<AdminAccessToken>,
    Path(contract_ulid): Path<Uuid>,
    Query(query): Query<GetOneContractQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ContractsIndex>> {
    let database = database.lock().await;
    let result = database
        .select_one_contract(
            Some(contract_ulid),
            query.contractor_ulid,
            query.client_ulid,
            query.query,
            query.branch_ulid,
        )
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

pub async fn admin_post_one_contract(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostOneContract>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let ulid = database
        .insert_one_contract(
            body.client_ulid,
            body.contractor_ulid,
            body.branch_ulid,
            &body.contract_name,
            &body.contract_type,
            &body.job_title,
            &body.contract_status,
            body.contract_amount,
            body.currency,
            &body.seniority,
            &body.begin_at,
            &body.end_at,
        )
        .await?;

    Ok(ulid.to_string())
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserIndex {
    pub ulid: Uuid,
    pub name: String,
    pub r#type: UserType,
    pub role: UserRole,
    pub contract_count: i64,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
    pub email: EmailWrapper,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientsIndex {
    client_ulid: Uuid,
    client_name: String,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorsIndex {
    contractor_ulid: Uuid,
    contractor_name: String,
    contract_name: Option<String>,
    contract_status: Option<String>,
    job_title: Option<String>,
    seniority: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostOneContract {
    client_ulid: Uuid,
    contractor_ulid: Uuid,
    branch_ulid: Option<Uuid>,
    contract_name: String,
    contract_type: String,
    job_title: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    seniority: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndex {
    contract_ulid: Uuid,
    client_ulid: Uuid,
    branch_ulid: Option<Uuid>,
    client_name: Option<String>,
    contractor_ulid: Uuid,
    contractor_name: Option<String>,
    contract_name: String,
    contract_type: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
    job_title: String,
    seniority: String,
}
