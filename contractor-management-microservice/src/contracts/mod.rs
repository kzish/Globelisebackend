use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Currency, EmailWrapper, OffsetDateWrapper},
    error::GlobeliseResult,
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
pub struct GetContractsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
}

pub async fn contracts_index(
    access_token: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Query(query): Query<GetContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ContractsIndex>> {
    let database = database.lock().await;
    let results = match role {
        UserRole::Client => ContractsIndex::Client(
            database
                .contracts_index_for_client(access_token.payload.ulid, query)
                .await?,
        ),
        UserRole::Contractor => ContractsIndex::Contractor(
            database
                .contracts_index_for_contractor(access_token.payload.ulid, query)
                .await?,
        ),
    };

    Ok(Json(results))
}

pub async fn eor_admin_contracts_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndexForEorAdmin>>> {
    let database = database.lock().await;
    Ok(Json(database.eor_admin_contracts_index(query).await?))
}

pub async fn eor_admin_create_contract(
    _: Token<AdminAccessToken>,
    Json(body): Json<CreateContractRequestForEorAdmin>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let ulid = database.create_contract(body.clone()).await?;

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

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ContractorsIndexSqlHelper {
    contractor_name: String,
    contract_name: Option<String>,
    contract_status: Option<String>,
    job_title: Option<String>,
    seniority: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ContractsIndex {
    Client(Vec<ContractsIndexForClient>),
    Contractor(Vec<ContractsIndexForContractor>),
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForClient {
    contractor_name: String,
    contract_ulid: Uuid,
    branch_ulid: Option<Uuid>,
    contract_name: String,
    contract_type: String,
    job_title: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForContractor {
    client_name: String,
    contract_ulid: Uuid,
    branch_ulid: Option<Uuid>,
    contract_name: String,
    contract_type: String,
    job_title: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateContractRequestForEorAdmin {
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
pub struct ContractsIndexForEorAdmin {
    client_name: String,
    contractor_name: String,
    contract_ulid: Uuid,
    contract_name: String,
    contract_type: String,
    job_title: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ContractsIndexCommonInfoSqlHelper {
    contract_name: String,
    contract_type: String,
    job_title: String,
    contract_status: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}
