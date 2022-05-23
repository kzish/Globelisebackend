use axum::{extract::Query, Extension, Json};
use common_utils::{
    calc_limit_and_offset,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmployeeContractorQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Uuid,

    pub branch_ulid: Option<Uuid>,
    pub employee_contractor_name: Option<String>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmployeeContractorResponse {
    pub ulid: Uuid,
    pub name: String,
    pub job_title: String,
    pub branch_ulid: Uuid,
    pub sub_entity: String,
    //TODO add team
    // pub team: String,
    pub department_name: String,
    pub time_zone: String,
    pub classification: String,
    pub client_ulid: Uuid,
    pub contract_name: String,
    pub contract_status: String,
}

pub async fn get_employee_contractors(
    claims: Token<UserAccessToken>,
    Query(query): Query<EmployeeContractorQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<EmployeeContractorResponse>>> {
    if claims.payload.ulid != query.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let response = database.get_employee_contractors(query).await?;

    Ok(Json(response))
}

impl Database {
    pub async fn get_employee_contractors(
        &self,
        query: EmployeeContractorQuery,
    ) -> GlobeliseResult<Vec<EmployeeContractorResponse>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let sql_query = "SELECT * FROM
                            search_employee_contractors
                        WHERE
                            client_ulid = $3
                        AND ($4 IS NULL OR name LIKE '%$4%')
                        AND ($5 IS NULL OR branch_ulid = $5)
                        LIMIT $1 OFFSET $2
                        ";

        let response = sqlx::query_as(sql_query)
            .bind(limit)
            .bind(offset)
            .bind(query.client_ulid)
            .bind(query.employee_contractor_name)
            .bind(query.branch_ulid)
            .fetch_all(&self.0)
            .await?;

        Ok(response)
    }
}
