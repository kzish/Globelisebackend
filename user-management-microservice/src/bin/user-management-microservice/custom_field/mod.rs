use axum::{
    extract::{ContentLengthLimit, Query},
    Extension, Json,
};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::AccessToken as UserAccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldDetailType {
    Personal,
    Employment,
    Bank,
    Payroll,
}

impl FieldDetailType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldDetailType::Personal => "PERSONAL",
            FieldDetailType::Employment => "EMPLOYMENT",
            FieldDetailType::Bank => "BANK",
            FieldDetailType::Payroll => "PAYROLL",
        }
    }

    pub fn from_str(input: &str) -> Option<FieldDetailType> {
        match input {
            "PERSONAL" => Some(FieldDetailType::Personal),
            "EMPLOYMENT" => Some(FieldDetailType::Employment),
            "BANK" => Some(FieldDetailType::Bank),
            "PAYROLL" => Some(FieldDetailType::Payroll),
            _ => None,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostCustomFieldRequest {
    field_name: String,
    field_detail_type: FieldDetailType,
    // Should be an enum in the future
    field_format: String,
    option_1: String,
    option_2: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetCustomFieldRequest {
    page: Option<u32>,
    per_page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetCustomFieldResponse {
    ulid: Ulid,
    client_ulid: Ulid,
    field_name: String,
    field_detail_type: FieldDetailType,
    // Should be an enum in the future
    field_format: String,
    option_1: String,
    option_2: String,
}

impl<'r> FromRow<'r, PgRow> for GetCustomFieldResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let field_detail_type = FieldDetailType::from_str(row.try_get("field_detail_type")?)
            .expect("There is something wrong with the tabl `user_detail_types`");
        Ok(Self {
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            client_ulid: ulid_from_sql_uuid(row.try_get("client_ulid")?),
            field_name: row.try_get("field_name")?,
            field_detail_type,
            field_format: row.try_get("field_format")?,
            option_1: row.try_get("field_option_1")?,
            option_2: row.try_get("field_option_2")?,
        })
    }
}

pub async fn user_post_custom_field(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PostCustomFieldRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let field_type_ulid = database
        .get_ulid_custom_field_type(request.field_detail_type)
        .await?;

    let ulid = database
        .create_custom_field(claims.payload.ulid, field_type_ulid, request)
        .await?;

    Ok(ulid.to_string())
}

pub async fn user_get_custom_fields(
    claims: Token<UserAccessToken>,
    Query(request): Query<GetCustomFieldRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetCustomFieldResponse>>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let result = database
        .get_custom_fields(claims.payload.ulid, request)
        .await?;

    Ok(Json(result))
}

impl Database {
    pub async fn create_custom_field(
        &self,
        client_ulid: Ulid,
        field_type_ulid: Ulid,
        request: PostCustomFieldRequest,
    ) -> GlobeliseResult<Ulid> {
        let ulid = Ulid::generate();

        let query = "
            INSERT INTO entity_client_custom_fields (
                ulid, client_ulid, field_name, field_detail_type_ulid, field_format,
                field_option_1, field_option_2
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7
            )";

        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(ulid_to_sql_uuid(client_ulid))
            .bind(request.field_name)
            .bind(ulid_to_sql_uuid(field_type_ulid))
            .bind(request.field_format)
            .bind(request.option_1)
            .bind(request.option_2)
            .execute(&self.0)
            .await?;

        Ok(ulid)
    }

    pub async fn get_custom_fields(
        &self,
        client_ulid: Ulid,
        request: GetCustomFieldRequest,
    ) -> GlobeliseResult<Vec<GetCustomFieldResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let query = "
            SELECT
                ulid, client_ulid, field_name, field_detail_type, field_format,
                field_option_1, field_option_2
            FROM
                entity_client_custom_fields_index
            WHERE
                client_ulid = $1
            LIMIT 
                $2 
            OFFSET 
                $3";

        let result = sqlx::query_as(query)
            .bind(ulid_to_sql_uuid(client_ulid))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn get_ulid_custom_field_type(
        &self,
        field_type: FieldDetailType,
    ) -> GlobeliseResult<Ulid> {
        let query = "
            SELECT
                ulid
            FROM
                user_detail_types
            WHERE
                detail_type = $1
            LIMIT 
                1";

        let result = sqlx::query(query)
            .bind(field_type.as_str())
            .fetch_one(&self.0)
            .await
            .map(|v| v.try_get("ulid").map(ulid_from_sql_uuid))??;

        Ok(result)
    }
}
