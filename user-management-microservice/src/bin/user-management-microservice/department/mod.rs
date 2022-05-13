use axum::{
    extract::{ContentLengthLimit, Query},
    Extension, Json,
};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::{Currency, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostDepartmentRequest {
    department_name: String,
    branch_ulid: Ulid,
    country: String,
    classification: String,
    currency: Currency,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetDepartmentRequest {
    page: Option<u32>,
    per_page: Option<u32>,
    client_ulid: Option<Ulid>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetDepartmentResponse {
    ulid: Ulid,
    branch_name: String,
    department_name: String,
    country: String,
    classification: String,
    currency: Currency,
    total_member: i64,
    client_ulid: Ulid,
}

impl<'r> FromRow<'r, PgRow> for GetDepartmentResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            branch_name: row.try_get("branch_name")?,
            department_name: row.try_get("department_name")?,
            country: row.try_get("country")?,
            classification: row.try_get("classification")?,
            currency: row.try_get("currency")?,
            total_member: row.try_get("total_member")?,
            client_ulid: ulid_from_sql_uuid(row.try_get("client_ulid")?),
        })
    }
}

pub async fn eor_admin_post_department(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PostDepartmentRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let ulid = database.create_department(request).await?;

    Ok(ulid.to_string())
}

pub async fn eor_admin_get_departments(
    _: Token<AdminAccessToken>,
    Query(request): Query<GetDepartmentRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetDepartmentResponse>>> {
    let database = database.lock().await;

    let result = database.get_departments(request).await?;

    Ok(Json(result))
}

pub async fn user_post_department(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PostDepartmentRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, request.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let ulid = database.create_department(request).await?;

    Ok(ulid.to_string())
}

pub async fn user_get_departments(
    claims: Token<UserAccessToken>,
    Query(mut request): Query<GetDepartmentRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetDepartmentResponse>>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    request.client_ulid = Some(claims.payload.ulid);

    let result = database.get_departments(request).await?;

    Ok(Json(result))
}

impl Database {
    pub async fn create_department(&self, request: PostDepartmentRequest) -> GlobeliseResult<Ulid> {
        let ulid = Ulid::generate();

        let query = "
            INSERT INTO entity_client_branch_departments (
                ulid, branch_ulid, department_name, country, classification, currency
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(ulid_to_sql_uuid(request.branch_ulid))
            .bind(request.department_name)
            .bind(request.country)
            .bind(request.classification)
            .bind(request.currency)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(ulid)
    }

    pub async fn get_departments(
        &self,
        request: GetDepartmentRequest,
    ) -> GlobeliseResult<Vec<GetDepartmentResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let query = "
            SELECT
                ulid, branch_ulid, branch_name, department_name, country, classification, 
                currency, total_member, client_ulid
            FROM
                entity_client_branch_departments_index
            WHERE
                ($1 IS NULL OR client_ulid = $1)
            LIMIT 
                $2 
            OFFSET 
                $3";

        let result = sqlx::query_as(query)
            .bind(request.client_ulid.map(ulid_to_sql_uuid))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
