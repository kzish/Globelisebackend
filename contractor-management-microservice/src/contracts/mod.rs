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
    common::{ulid_to_sql_uuid, PaginationQuery},
    database::{Database, SharedDatabase},
    env::USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
};

impl Database {
    /// Counts the number of contracts.
    pub async fn count_number_of_contracts(
        &self,
        ulid: &Ulid,
        role: &Role,
    ) -> GlobeliseResult<i64> {
        let client_ulid = match role {
            Role::Client => Some(ulid_to_sql_uuid(*ulid)),
            Role::Contractor => None,
        };
        let contractor_ulid = match role {
            Role::Client => None,
            Role::Contractor => Some(ulid_to_sql_uuid(*ulid)),
        };

        let result = sqlx::query_scalar(
            "SELECT
                COUNT(*)
            FROM
                contracts
            WHERE
                ($1 IS NULL OR (client_ulid = $1)) AND
                ($2 IS NULL OR (contractor_ulid = $2))",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }

    /// Indexes contracts working for a client.
    pub async fn contractor_index(
        &self,
        client_ulid: Ulid,
        query: PaginationQuery,
    ) -> GlobeliseResult<Vec<ContractorIndex>> {
        let index = sqlx::query_as(
            "SELECT
                contractor_name, contract_name, contract_status,
                job_title, seniority
            FROM
                contractor_index
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR (contractor_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.search_text)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contract_for_contractor_index(
        &self,
        contractor_ulid: Ulid,
        query: PaginationQuery,
    ) -> GlobeliseResult<Vec<ContractForContractorIndex>> {
        let index = sqlx::query_as(
            "
            SELECT
                contractor_ulid, contract_name, job_title, seniority,
                client_name, contract_status, contract_amount, end_at
            FROM
                contract_index_for_contractor
            WHERE
                contractor_ulid = $1 AND
                ($2 IS NULL OR (contract_name ~* $2 OR client_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(query.search_text)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contract_for_client_index(
        &self,
        client_ulid: Ulid,
        query: PaginationQuery,
    ) -> GlobeliseResult<Vec<ContractForClientIndex>> {
        let index = sqlx::query_as(
            "
            SELECT
                client_ulid, contract_name, job_title, seniority,
                contractor_name, contract_status, contract_amount, end_at
            FROM
                contract_index_for_client
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR (contract_name ~* $2 OR client_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.search_text)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract for EOR admin purposes
    pub async fn eor_admin_contract_index(
        &self,
        query: PaginationQuery,
    ) -> GlobeliseResult<Vec<ContractForClientIndex>> {
        let index = sqlx::query_as(
            "
            SELECT
                contract_name, job_title, seniority,
                client_name, contract_status, contract_amount, end_at
            FROM
                contract_index_for_eor_admin
            WHERE
                ($1 IS NULL OR (contract_name ~* $1 OR client_name ~* $1))
            LIMIT $2 OFFSET $3",
        )
        .bind(query.search_text)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }
}

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
