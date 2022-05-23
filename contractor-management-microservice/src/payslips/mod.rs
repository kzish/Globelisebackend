use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{DateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, FromInto, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{token::UserAccessToken, user::Role};
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::SharedDatabase};

mod database;

/// List the payslips.
pub async fn user_payslips_index(
    claims: Token<UserAccessToken>,
    Path(role): Path<Role>,
    Query(mut query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<PayslipsIndex>>> {
    let database = shared_database.lock().await;

    match role {
        Role::Client => query.client_ulid = Some(claims.payload.ulid),
        Role::Contractor => query.contractor_ulid = Some(claims.payload.ulid),
    };

    let result = database.payslips_index(query).await?;
    Ok(Json(result))
}

/// List the payslips.
pub async fn eor_admin_payslips_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<PayslipsIndex>>> {
    let database = shared_database.lock().await;
    let result = database.payslips_index(query).await?;
    Ok(Json(result))
}

/// Create a payslip.
pub async fn eor_admin_create_payslip(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<CreatePayslipsIndex>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = shared_database.lock().await;
    database.create_payslip(request).await?;
    Ok(())
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
    #[serde_as(as = "FromInto<DateWrapper>")]
    payment_date: sqlx::types::time::Date,
    #[serde_as(as = "FromInto<DateWrapper>")]
    begin_period: sqlx::types::time::Date,
    #[serde_as(as = "FromInto<DateWrapper>")]
    end_period: sqlx::types::time::Date,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct PayslipsIndexSqlHelper {
    client_name: String,
    contractor_name: String,
    #[serde(default)]
    contract_name: Option<String>,
    payslip_title: String,
    #[serde_as(as = "FromInto<DateWrapper>")]
    payment_date: sqlx::types::time::Date,
    #[serde_as(as = "FromInto<DateWrapper>")]
    begin_period: sqlx::types::time::Date,
    #[serde_as(as = "FromInto<DateWrapper>")]
    end_period: sqlx::types::time::Date,
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
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub begin_period: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub end_period: sqlx::types::time::Date,
    #[serde_as(as = "Base64")]
    pub payslip_file: Vec<u8>,
}
