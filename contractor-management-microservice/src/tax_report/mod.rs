use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Country, OffsetDateWrapper, UserRole, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
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

/// List the tax reports
pub async fn user_tax_report_index(
    claims: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Query(mut query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<TaxReportIndex>>> {
    let database = shared_database.lock().await;

    match role {
        UserRole::Client => query.client_ulid = Some(claims.payload.ulid),
        UserRole::Contractor => query.contractor_ulid = Some(claims.payload.ulid),
    };

    let result = database.tax_report_index(query).await?;
    Ok(Json(result))
}

/// List the tax reports
pub async fn eor_admin_tax_report_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<TaxReportIndex>>> {
    let database = shared_database.lock().await;
    let result = database.tax_report_index(query).await?;
    Ok(Json(result))
}

/// Create the tax reports
pub async fn eor_admin_create_tax_report(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<CreateTaxReportIndex>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = shared_database.lock().await;
    database.create_tax_report(request).await?;
    Ok(())
}

#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[sqlx(type_name = "interval_type")]
pub enum TaxInterval {
    Monthly,
    Yearly,
}

impl TaxInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaxInterval::Monthly => "monthly",
            TaxInterval::Yearly => "yearly",
        }
    }
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaxReportIndex {
    tax_report_ulid: Uuid,
    client_name: String,
    contractor_name: String,
    #[serde(default)]
    contract_name: Option<String>,
    tax_interval: TaxInterval,
    tax_name: String,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    pub begin_period: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    pub end_period: sqlx::types::time::OffsetDateTime,
    country: String,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
struct TaxReportIndexSqlHelper {
    client_name: String,
    contractor_name: String,
    #[serde(default)]
    contract_name: Option<String>,
    tax_interval: TaxInterval,
    tax_name: String,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    pub begin_period: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    pub end_period: sqlx::types::time::OffsetDateTime,
    country: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateTaxReportIndex {
    pub client_ulid: Uuid,
    pub contractor_ulid: Uuid,
    #[serde(default)]
    pub contract_ulid: Option<Uuid>,
    pub tax_interval: TaxInterval,
    pub tax_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub begin_period: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub end_period: sqlx::types::time::OffsetDateTime,
    pub country: Country,
    #[serde_as(as = "Base64")]
    pub tax_report_file: Vec<u8>,
}
