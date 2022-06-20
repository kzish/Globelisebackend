use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Currency, OffsetDateWrapper, UserRole, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{user::OnboardedUserIndex, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::SharedDatabase};

mod database;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
}

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
pub async fn admin_get_many_user_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    println!("query:{:#?}", query);
    let database = shared_database.lock().await;
    let result = database
        .select_many_onboarded_user_index(
            query.page,
            query.per_page,
            query.query,
            query.user_type,
            query.user_role,
            None,
            None,
        )
        .await?;
    Ok(Json(result))
}

pub async fn user_get_many_clients_for_contractors(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ClientContractorPair>>> {
    let database = database.lock().await;
    let result = database
        .select_many_client_contractor_pairs(
            query.page,
            query.per_page,
            query.query,
            query.contractor_ulid,
            Some(access_token.payload.ulid),
        )
        .await?;
    Ok(Json(result))
}

pub async fn user_get_many_contractors_for_clients(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ClientContractorPair>>> {
    let database = database.lock().await;
    let result = database
        .select_many_client_contractor_pairs(
            query.page,
            query.per_page,
            query.query,
            Some(access_token.payload.ulid),
            query.client_ulid,
        )
        .await?;
    Ok(Json(result))
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

pub async fn user_get_many_contract_index(
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

pub async fn user_get_one_contract_index(
    claims: Token<UserAccessToken>,
    Path((user_role, contract_ulid)): Path<(UserRole, Uuid)>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ContractsIndex>> {
    let database = database.lock().await;
    let result = match user_role {
        UserRole::Client => database
            .select_one_contract(
                Some(contract_ulid),
                query.contractor_ulid,
                Some(claims.payload.ulid),
                query.query,
                query.branch_ulid,
            )
            .await?
            .ok_or(GlobeliseError::NotFound)?,
        UserRole::Contractor => database
            .select_one_contract(
                Some(contract_ulid),
                Some(claims.payload.ulid),
                query.client_ulid,
                query.query,
                query.branch_ulid,
            )
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    };

    Ok(Json(result))
}

pub async fn user_sign_one_contract(
    claims: Token<UserAccessToken>,
    Path((user_role, contract_ulid)): Path<(UserRole, Uuid)>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if user_role != UserRole::Contractor
        || !claims.payload.user_roles.contains(&UserRole::Contractor)
    {
        return Err(GlobeliseError::unauthorized(
            "Only contractors can sign a contract",
        ));
    }

    database
        .sign_one_contract(contract_ulid, claims.payload.ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(())
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
            &body.contract_status,
            &body.contract_type,
            &body.job_title,
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
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPair {
    client_ulid: Uuid,
    client_name: String,
    contractor_ulid: Uuid,
    contractor_name: String,
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
    contract_status: String,
    contract_type: String,
    job_title: String,
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
