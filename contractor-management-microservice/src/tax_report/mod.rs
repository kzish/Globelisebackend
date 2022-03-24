use axum::{
    extract::{ContentLengthLimit, Extension, Query},
    Json,
};
use common_utils::{
    custom_serde::{DateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{AccessToken as UserAccessToken, Role};

use crate::{
    common::{ulid_from_sql_uuid, ulid_to_sql_uuid},
    database::{Database, SharedDatabase},
};

impl Database {
    /// Indexes tax report.
    pub async fn tax_report_index(
        &self,
        ulid: Ulid,
        query: TaxReportIndexQuery,
    ) -> GlobeliseResult<Vec<TaxReportIndex>> {
        let client_ulid = match query.role {
            Role::Client => Some(ulid_to_sql_uuid(ulid)),
            Role::Contractor => None,
        };
        let contractor_ulid = match query.role {
            Role::Client => None,
            Role::Contractor => Some(ulid_to_sql_uuid(ulid)),
        };
        let index = sqlx::query_as(
            "
            SELECT
                ulid, client_name, contractor_name, contract_name, tax_interval,
                tax_name, begin_period, end_period, country
            FROM
                tax_reports_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (client_name ~* $3 OR contractor_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(query.search_text)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Create tax report
    pub async fn create_tax_report(&self, query: CreateTaxReportIndex) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO tax_reports
            (ulid, client_ulid, contractor_ulid, contract_ulid, tax_interval,
            tax_name, begin_period, end_period, country, tax_report_file)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(ulid_to_sql_uuid(Ulid::generate()))
        .bind(ulid_to_sql_uuid(query.client_ulid))
        .bind(ulid_to_sql_uuid(query.contractor_ulid))
        .bind(query.contract_ulid.map(ulid_to_sql_uuid))
        .bind(query.tax_interval)
        .bind(query.tax_name)
        .bind(query.begin_period)
        .bind(query.end_period)
        .bind(query.country)
        .bind(query.tax_report_file)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}

/// List the tax reports
pub async fn user_tax_report_index(
    claims: Token<UserAccessToken>,
    Query(query): Query<TaxReportIndexQuery>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<TaxReportIndex>>> {
    let ulid = claims.payload.ulid.parse::<Ulid>()?;
    let database = shared_database.lock().await;
    let result = database.tax_report_index(ulid, query).await?;
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
    let result = database.tax_report_index(ulid, query).await?;
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaxReportIndex {
    ulid: Ulid,
    #[serde(flatten)]
    other_fields: TaxReportIndexSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for TaxReportIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            other_fields: TaxReportIndexSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, FromRow, Serialize)]
struct TaxReportIndexSqlHelper {
    client_name: String,
    contractor_name: String,
    contract_name: Option<String>,
    tax_interval: TaxInterval,
    tax_name: String,
    begin_period: String,
    end_period: String,
    country: String,
}

#[derive(Debug, Deserialize)]
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
    pub contract_ulid: Option<Ulid>,
    pub tax_interval: TaxInterval,
    pub tax_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub begin_period: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub end_period: sqlx::types::time::Date,
    pub country: String,
    #[serde_as(as = "Base64")]
    pub tax_report_file: Vec<u8>,
}
