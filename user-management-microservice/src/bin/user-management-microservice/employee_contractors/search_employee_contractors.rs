use axum::{extract::Query, Extension, Json};
use common_utils::{
    calc_limit_and_offset,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::token::UserAccessToken;

use crate::database::{Database, SharedDatabase};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmployeeContractorQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Ulid,
    pub branch_ulid: Option<Ulid>,
    pub employee_contractor_name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmployeeContractorResponse {
    pub ulid: Ulid,
    pub name: String,
    pub job_title: String,
    pub branch_ulid: Ulid,
    pub sub_entity: String,
    //TODO add team
    // pub team: String,
    pub department_name: String,
    pub time_zone: String,
    pub classification: String,
    pub client_ulid: Ulid,
    pub contract_name: String,
    pub contract_status: String,
}

impl FromRow<'_, PgRow> for EmployeeContractorResponse {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(EmployeeContractorResponse {
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            client_ulid: ulid_from_sql_uuid(row.try_get("client_ulid")?),
            name: row.try_get("name")?,
            job_title: row.try_get("job_title")?,
            branch_ulid: ulid_from_sql_uuid(row.try_get("branch_ulid")?),
            sub_entity: row.try_get("sub_entity")?,
            //TODO add team
            // team: row.try_get("team")?,
            department_name: row.try_get("department_name")?,
            time_zone: row.try_get("time_zone")?,
            classification: row.try_get("classification")?,
            contract_name: row.try_get("contract_name")?,
            contract_status: row.try_get("contract_status")?,
        })
    }
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
            .bind(ulid_to_sql_uuid(query.client_ulid))
            .bind(query.employee_contractor_name)
            .bind(query.branch_ulid.map(ulid_to_sql_uuid))
            .fetch_all(&self.0)
            .await?;

        Ok(response)
    }
}
