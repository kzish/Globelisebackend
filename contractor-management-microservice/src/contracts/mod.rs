use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Currency, DateWrapper},
    error::GlobeliseResult,
    token::{Token, TokenString},
    ulid_from_sql_uuid,
};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use reqwest::Client;
use rusty_ulid::Ulid;
use serde::Serialize;
use serde_with::{serde_as, FromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{AccessToken as UserAccessToken, GetUserInfoRequest, Role};

use crate::{
    common::PaginatedQuery, database::SharedDatabase, env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
};

mod database;

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
            .count_number_of_contracts(v.ulid, v.user_role)
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

pub async fn contractors_index(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorsIndex>>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .contractors_index(access_token.payload.ulid, query)
            .await?,
    ))
}

pub async fn contracts_index(
    access_token: Token<UserAccessToken>,
    Path(role): Path<Role>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ContractsIndex>> {
    let database = database.lock().await;
    let results = match role {
        Role::Client => ContractsIndex::Client(
            database
                .contracts_index_for_client(access_token.payload.ulid, query)
                .await?,
        ),
        Role::Contractor => ContractsIndex::Contractor(
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

#[serde_as]
#[derive(Debug, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub contract_count: i64,
    #[serde_as(as = "FromInto<DateWrapper>")]
    pub created_at: sqlx::types::time::Date,
    pub email: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorsIndex {
    contractor_ulid: Ulid,
    #[serde(flatten)]
    other_fields: ContractorsIndexSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractorsIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            contractor_ulid: ulid_from_sql_uuid(row.try_get("contractor_ulid")?),
            other_fields: ContractorsIndexSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ContractorsIndexSqlHelper {
    contractor_name: String,
    contract_name: String,
    contract_status: String,
    job_title: String,
    seniority: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ContractsIndex {
    Client(Vec<ContractsIndexForClient>),
    Contractor(Vec<ContractsIndexForContractor>),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForClient {
    contractor_name: String,
    contract_ulid: Ulid,
    #[serde(flatten)]
    common_info: ContractsIndexCommonInfoSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractsIndexForClient {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            contractor_name: row.try_get("contractor_name")?,
            contract_ulid: ulid_from_sql_uuid(row.try_get("contract_ulid")?),
            common_info: ContractsIndexCommonInfoSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForContractor {
    client_name: String,
    contract_ulid: Ulid,
    #[serde(flatten)]
    common_info: ContractsIndexCommonInfoSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractsIndexForContractor {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            client_name: row.try_get("client_name")?,
            contract_ulid: ulid_from_sql_uuid(row.try_get("contract_ulid")?),
            common_info: ContractsIndexCommonInfoSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexForEorAdmin {
    client_name: String,
    contractor_name: String,
    contract_ulid: Ulid,
    #[serde(flatten)]
    common_info: ContractsIndexCommonInfoSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for ContractsIndexForEorAdmin {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            client_name: row.try_get("client_name")?,
            contractor_name: row.try_get("contractor_name")?,
            contract_ulid: ulid_from_sql_uuid(row.try_get("contract_ulid")?),
            common_info: ContractsIndexCommonInfoSqlHelper::from_row(row)?,
        })
    }
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
    #[serde_as(as = "FromInto<DateWrapper>")]
    begin_at: sqlx::types::time::Date,
    #[serde_as(as = "FromInto<DateWrapper>")]
    end_at: sqlx::types::time::Date,
}
