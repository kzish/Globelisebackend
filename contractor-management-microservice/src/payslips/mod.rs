use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{DateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
    token::Token,
    ulid_from_sql_uuid,
};
use eor_admin_microservice_sdk::token::AccessToken as AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, FromInto, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::AccessToken as UserAccessToken, user::Role};

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
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PayslipsIndex {
    payslip_ulid: Ulid,
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

impl<'r> FromRow<'r, PgRow> for PayslipsIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let other_fields = PayslipsIndexSqlHelper::from_row(row)?;
        Ok(Self {
            payslip_ulid: ulid_from_sql_uuid(row.try_get("payslip_ulid")?),
            client_name: other_fields.client_name,
            contractor_name: other_fields.contractor_name,
            contract_name: other_fields.contract_name,
            payslip_title: other_fields.payslip_title,
            payment_date: other_fields.payment_date,
            begin_period: other_fields.begin_period,
            end_period: other_fields.end_period,
        })
    }
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
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
    #[serde(default)]
    pub contract_ulid: Option<Ulid>,
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
