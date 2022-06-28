use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{OffsetDateWrapper, UserRole, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, FromInto, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::SharedDatabase};

mod database;

pub async fn user_find_many_payslips(
    claims: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Query(query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<PayslipsIndex>>> {
    let database = shared_database.lock().await;

    let result = match role {
        UserRole::Client => {
            database
                .select_many_payslips(
                    query.page,
                    query.per_page,
                    query.query,
                    query.contractor_ulid,
                    Some(claims.payload.ulid),
                )
                .await?
        }
        UserRole::Contractor => {
            database
                .select_many_payslips(
                    query.page,
                    query.per_page,
                    query.query,
                    Some(claims.payload.ulid),
                    query.client_ulid,
                )
                .await?
        }
    };

    Ok(Json(result))
}

pub async fn user_get_one_payslip_index(
    claims: Token<UserAccessToken>,
    Path((user_role, payslip_ulid)): Path<(UserRole, Uuid)>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PayslipsIndex>> {
    let database = shared_database.lock().await;

    let result = match user_role {
        UserRole::Client => {
            database
                .select_one_payslip_index(payslip_ulid, Some(claims.payload.ulid), None)
                .await?
        }
        UserRole::Contractor => {
            database
                .select_one_payslip_index(payslip_ulid, None, Some(claims.payload.ulid))
                .await?
        }
    }
    .ok_or_else(|| GlobeliseError::not_found("Cannot find a payslip with that UUID"))?;

    Ok(Json(result))
}

pub async fn admin_get_many_payslip_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<PayslipsIndex>>> {
    let database = shared_database.lock().await;
    let result = database
        .select_many_payslips(
            query.page,
            query.per_page,
            query.query,
            query.contractor_ulid,
            query.client_ulid,
        )
        .await?;
    Ok(Json(result))
}

pub async fn admin_get_one_payslip_index(
    _: Token<AdminAccessToken>,
    Path(payslip_ulid): Path<Uuid>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PayslipsIndex>> {
    let database = shared_database.lock().await;

    let result = database
        .select_one_payslip_index(payslip_ulid, None, None)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find a payslip with that UUID"))?;

    Ok(Json(result))
}

pub async fn user_delete_one_payslip(
    claims: Token<UserAccessToken>,
    Path((user_role, payslip_ulid)): Path<(UserRole, Uuid)>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = shared_database.lock().await;

    match user_role {
        UserRole::Client => {
            database
                .delete_one_payslip(payslip_ulid, Some(claims.payload.ulid), None)
                .await?
        }
        UserRole::Contractor => {
            database
                .delete_one_payslip(payslip_ulid, None, Some(claims.payload.ulid))
                .await?
        }
    };

    Ok(())
}

pub async fn admin_delete_one_payslip(
    _: Token<AdminAccessToken>,
    Path(payslip_ulid): Path<Uuid>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = shared_database.lock().await;

    database
        .delete_one_payslip(payslip_ulid, None, None)
        .await?;

    Ok(())
}

pub async fn admin_post_one_payslip(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<CreatePayslipsIndex>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = shared_database.lock().await;
    let result = database
        .insert_one_payslip(
            body.client_ulid,
            body.contractor_ulid,
            body.contract_ulid,
            body.payslip_title,
            body.payment_date,
            body.begin_period,
            body.end_period,
            body.payslip_file,
        )
        .await?;
    Ok(result.to_string())
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PayslipsIndex {
    payslip_ulid: Uuid,
    client_name: String,
    contractor_name: String,
    #[serde(default)]
    contract_name: Option<String>,
    payslip_title: String,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    begin_period: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    end_period: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreatePayslipsIndex {
    pub client_ulid: Uuid,
    pub contractor_ulid: Uuid,
    #[serde(default)]
    pub contract_ulid: Option<Uuid>,
    pub payslip_title: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub begin_period: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub end_period: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "Base64")]
    pub payslip_file: Vec<u8>,
}
