use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Currency, EmailWrapper, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{UserRole, UserType},
};
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::SharedDatabase};

use self::database::OnboardedUserIndex;

mod database;

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

/// Lists all the users plus some information about them.
pub async fn eor_admin_user_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetUserInfoRequest>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = shared_database.lock().await;
    let result = database
        .select_many_onboarded_users(
            query.page,
            query.per_page,
            query.search_text,
            query.user_type,
            query.user_role,
            None,
            None,
        )
        .await?;
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

pub async fn admin_get_many_contracts(
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
