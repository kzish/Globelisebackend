use axum::{
    extract::{ContentLengthLimit, Path, Query},
    Extension, Json,
};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    error::{GlobeliseError, GlobeliseResult},
    impl_enum_asfrom_str,
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

impl_enum_asfrom_str!(
    FieldDetailFormat,
    SHORT_TEXT,
    LONG_TEXT,
    NUMBER,
    ACCOUNTING_NUMBER,
    DD_MM_YY,
    SINGLE_SELECT,
    MULTIPLE_SELECT
);

impl_enum_asfrom_str!(FieldDetailType, PERSONAL, EMPLOYMENT, BANK, PAYROLL);

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetCustomFieldResponse {
    ulid: Uuid,
    client_ulid: Uuid,
    field_name: String,
    field_type: FieldDetailType,
    field_format: FieldDetailFormat,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostCustomFieldRequestForClient {
    field_name: String,
    field_type: FieldDetailType,
    field_format: FieldDetailFormat,
}

pub async fn user_post_custom_field(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostCustomFieldRequestForClient>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let ulid = database
        .create_custom_field(
            claims.payload.ulid,
            body.field_name,
            body.field_type,
            body.field_format,
        )
        .await?;

    Ok(ulid.to_string())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetCustomFieldRequest {
    page: Option<u32>,
    per_page: Option<u32>,
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
        .get_custom_fields(Some(claims.payload.ulid), request.page, request.per_page)
        .await?;

    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostCustomFieldRequestForAdmin {
    client_ulid: Uuid,
    field_name: String,
    field_type: FieldDetailType,
    field_format: FieldDetailFormat,
}

pub async fn admin_post_custom_field(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostCustomFieldRequestForAdmin>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let ulid = database
        .create_custom_field(
            body.client_ulid,
            body.field_name,
            body.field_type,
            body.field_format,
        )
        .await?;

    Ok(ulid.to_string())
}

pub async fn admin_get_custom_field_by_ulid(
    _: Token<AdminAccessToken>,
    Path(ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<GetCustomFieldResponse>> {
    let database = database.lock().await;
    let result = database
        .get_custom_field_by_ulid(ulid, None)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetCustomFieldRequestForAdmin {
    client_ulid: Option<Uuid>,
    page: Option<u32>,
    per_page: Option<u32>,
}

pub async fn admin_get_custom_fields(
    _: Token<AdminAccessToken>,
    Query(request): Query<GetCustomFieldRequestForAdmin>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetCustomFieldResponse>>> {
    let database = database.lock().await;

    let result = database
        .get_custom_fields(request.client_ulid, request.page, request.per_page)
        .await?;

    Ok(Json(result))
}

impl Database {
    pub async fn create_custom_field(
        &self,
        client_ulid: Uuid,
        field_name: String,
        field_type: FieldDetailType,
        field_format: FieldDetailFormat,
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        let query = "
            INSERT INTO entity_client_custom_fields (
                ulid, client_ulid, field_name, field_type, field_format
            ) VALUES (
                $1, $2, $3, $4, $5
            )";

        sqlx::query(query)
            .bind(ulid)
            .bind(client_ulid)
            .bind(field_name)
            .bind(field_type)
            .bind(field_format)
            .execute(&self.0)
            .await?;

        Ok(ulid)
    }

    pub async fn get_custom_field_by_ulid(
        &self,
        ulid: Uuid,
        client_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Option<GetCustomFieldResponse>> {
        let query = "
            SELECT
                ulid, client_ulid, field_name, field_type, field_format
            FROM
                entity_client_custom_fields
            WHERE
                ulid = $1 AND
                ($2 IS NULL OR client_ulid = $2)";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn get_custom_fields(
        &self,
        client_ulid: Option<Uuid>,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<GetCustomFieldResponse>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let query = "
            SELECT
                ulid, client_ulid, field_name, field_type, field_format
            FROM
                entity_client_custom_fields
            WHERE
                $1 IS NULL OR (client_ulid = $1)
            LIMIT 
                $2 
            OFFSET 
                $3";

        let result = sqlx::query_as(query)
            .bind(client_ulid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }
}
