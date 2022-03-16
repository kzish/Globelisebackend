use axum::{
    extract::{ContentLengthLimit, Extension, Query},
    Json,
};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use strum::{Display, EnumIter, EnumString};
use user_management_microservice_sdk::{AccessToken as UserAccessToken, Role};

use crate::database::SharedDatabase;

/// List the tax reports
pub async fn user_tax_report_index(
    claims: Token<UserAccessToken>,
    Query(query): Query<TaxReportIndexQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<TaxReportIndex>>> {
    let ulid = claims.payload.ulid.parse::<Ulid>()?;
    let database = shared_database.lock().await;
    let result = database.tax_report_index(&ulid, query).await?;
    Ok(Json(result))
}

/// List the tax reports
pub async fn eor_admin_tax_report_index(
    claims: Token<AdminAccessToken>,
    Query(query): Query<TaxReportIndexQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<TaxReportIndex>>> {
    let ulid = claims.payload.ulid.parse::<Ulid>()?;
    let database = shared_database.lock().await;
    let result = database.tax_report_index(&ulid, query).await?;
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TaxInterval {
    Monthly,
    Yearly,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaxReportIndex {
    client_ulid: Ulid,
    client_name: String,
    contractor_ulid: Ulid,
    contractor_name: String,
    tax_interval: TaxInterval,
}

impl TaxReportIndex {
    pub fn from_pg_row(row: PgRow) -> GlobeliseResult<Self> {
        Ok(Self {
            client_ulid: row.try_get::<String, _>("client_ulid")?.parse::<Ulid>()?,
            client_name: row.try_get::<String, _>("client_name")?,
            contractor_ulid: row
                .try_get::<String, _>("contractor_ulid")?
                .parse::<Ulid>()?,
            contractor_name: row.try_get::<String, _>("contractor_name")?,
            tax_interval: row
                .try_get::<String, _>("tax_interval")?
                .parse::<TaxInterval>()?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaxReportIndexQuery {
    #[serde(default = "TaxReportIndexQuery::default_page")]
    pub page: i64,
    #[serde(default = "TaxReportIndexQuery::default_per_page")]
    pub per_page: i64,
    pub search_text: Option<String>,
    // NOTE: The access token should have this information instead because
    // someone _could_ spoof if they have a similar ULID.
    pub role: Role,
}

impl TaxReportIndexQuery {
    fn default_page() -> i64 {
        1
    }

    fn default_per_page() -> i64 {
        25
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateTaxReportIndex {
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
    pub tax_interval: TaxInterval,
    pub tax_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub begin_period: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub end_period: sqlx::types::time::Date,
    pub country: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub tax_report_file: Option<ImageData>,
}
