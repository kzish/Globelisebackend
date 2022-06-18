use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Country, OffsetDateWrapper, UserRole, FORM_DATA_LENGTH_LIMIT},
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

pub async fn user_get_many_tax_report_index(
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

    let result = database
        .select_many_tax_reports(
            query.page,
            query.per_page,
            query.query,
            query.contractor_ulid,
            query.client_ulid,
        )
        .await?;
    Ok(Json(result))
}

pub async fn admin_get_many_tax_report_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<TaxReportIndex>>> {
    let database = shared_database.lock().await;
    let result = database
        .select_many_tax_reports(
            query.page,
            query.per_page,
            query.query,
            query.contractor_ulid,
            query.client_ulid,
        )
        .await?;
    Ok(Json(result))
}

pub async fn admin_get_one_tax_report_index(
    _: Token<AdminAccessToken>,
    Path(tax_report_ulid): Path<Uuid>,
    Query(query): Query<PaginatedQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<TaxReportIndex>> {
    let database = shared_database.lock().await;
    let result = database
        .select_one_tax_report(
            Some(tax_report_ulid),
            query.contractor_ulid,
            query.client_ulid,
            query.query,
        )
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

pub async fn admin_post_one_tax_report(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<CreateTaxReportIndex>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = shared_database.lock().await;
    database.insert_one_tax_report(request).await?;
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
    country: Country,
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
