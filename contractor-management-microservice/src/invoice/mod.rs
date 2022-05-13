use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::DateWrapper, error::GlobeliseResult, token::Token, ulid_from_sql_uuid,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use itertools::izip;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::Role};

use crate::database::SharedDatabase;

mod database;

pub async fn user_invoice_individual_index(
    claims: Token<UserAccessToken>,
    Path(role): Path<Role>,
    Query(mut query): Query<InvoiceIndividualIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceIndividualIndex>>> {
    let ulid = claims.payload.ulid;

    // Override the provided query with the ulid provided by the tokens.
    match role {
        Role::Client => query.client_ulid = Some(ulid),
        Role::Contractor => query.contractor_ulid = Some(ulid),
    };

    let database = database.lock().await;
    Ok(Json(database.invoice_individual_index(query).await?))
}

pub async fn eor_admin_invoice_individual_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<InvoiceIndividualIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceIndividualIndex>>> {
    let database = database.lock().await;
    Ok(Json(database.invoice_individual_index(query).await?))
}

pub async fn user_invoice_group_index(
    claims: Token<UserAccessToken>,
    Path(role): Path<Role>,
    Query(mut query): Query<InvoiceGroupIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceGroupIndex>>> {
    let ulid = claims.payload.ulid;

    // Override the provided query with the ulid provided by the tokens.
    match role {
        Role::Client => query.client_ulid = Some(ulid),
        Role::Contractor => query.contractor_ulid = Some(ulid),
    };

    let database = database.lock().await;
    Ok(Json(database.invoice_group_index(query).await?))
}

pub async fn eor_admin_invoice_group_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<InvoiceGroupIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceGroupIndex>>> {
    let database = database.lock().await;
    Ok(Json(database.invoice_group_index(query).await?))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceIndividualIndexQuery {
    pub invoice_group_ulid: Ulid,
    pub invoice_status: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Ulid>,
    pub client_ulid: Option<Ulid>,
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceIndividualIndex {
    ulid: Ulid,
    invoice_group_ulid: Ulid,
    client_ulid: Ulid,
    contractor_ulid: Ulid,
    invoice_id: i64,
    #[serde_as(as = "FromInto<DateWrapper>")]
    invoice_due: sqlx::types::time::Date,
    invoice_status: String,
    invoice_amount: sqlx::types::Decimal,
}

impl<'r> FromRow<'r, PgRow> for InvoiceIndividualIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let other_fields = InvoiceIndividualIndexSqlHelper::from_row(row)?;
        Ok(Self {
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            invoice_group_ulid: ulid_from_sql_uuid(row.try_get("invoice_group_ulid")?),
            client_ulid: ulid_from_sql_uuid(row.try_get("client_ulid")?),
            contractor_ulid: ulid_from_sql_uuid(row.try_get("contractor_ulid")?),
            invoice_id: other_fields.invoice_id,
            invoice_due: other_fields.invoice_due,
            invoice_status: other_fields.invoice_status,
            invoice_amount: other_fields.invoice_amount,
        })
    }
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceIndividualIndexSqlHelper {
    invoice_id: i64,
    #[serde_as(as = "FromInto<DateWrapper>")]
    invoice_due: sqlx::types::time::Date,
    invoice_status: String,
    invoice_amount: sqlx::types::Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceGroupIndexQuery {
    pub invoice_status: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Ulid>,
    pub client_ulid: Option<Ulid>,
}

#[derive(Debug, Serialize)]
pub enum InvoiceGroupIndex {
    Single(InvoiceIndividualIndex),
    Bulk(Vec<InvoiceIndividualIndex>),
}

impl<'r> FromRow<'r, PgRow> for InvoiceGroupIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let results = InvoiceGroupIndexInternal::from_row(row)?;

        let mut all = izip!(
            results.ulid.into_iter(),
            results.client_ulid.into_iter(),
            results.contractor_ulid.into_iter(),
            results.other_fields.invoice_id.into_iter(),
            results.other_fields.invoice_due,
            results.other_fields.invoice_status.into_iter(),
            results.other_fields.invoice_amount.into_iter()
        )
        .map(
            |(
                ulid,
                client_ulid,
                contractor_ulid,
                invoice_id,
                invoice_due,
                invoice_status,
                invoice_amount,
            )| {
                InvoiceIndividualIndex {
                    ulid,
                    invoice_group_ulid: results.invoice_group_ulid,
                    client_ulid,
                    contractor_ulid,
                    invoice_id,
                    invoice_due,
                    invoice_status,
                    invoice_amount,
                }
            },
        )
        .collect::<Vec<_>>();

        let length = all.len();

        Ok(if length == 1 {
            InvoiceGroupIndex::Single(all.pop().unwrap())
        } else {
            InvoiceGroupIndex::Bulk(all)
        })
    }
}

#[derive(Debug)]
struct InvoiceGroupIndexInternal {
    ulid: Vec<Ulid>,
    invoice_group_ulid: Ulid,
    client_ulid: Vec<Ulid>,
    contractor_ulid: Vec<Ulid>,
    other_fields: InvoiceGroupIndexInternalSqlHelper,
}

impl<'r> FromRow<'r, PgRow> for InvoiceGroupIndexInternal {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            ulid: row
                .try_get::<Vec<sqlx::types::Uuid>, _>("ulid")?
                .into_iter()
                .map(ulid_from_sql_uuid)
                .collect(),
            invoice_group_ulid: ulid_from_sql_uuid(row.try_get("invoice_group_ulid")?),
            client_ulid: row
                .try_get::<Vec<sqlx::types::Uuid>, _>("client_ulid")?
                .into_iter()
                .map(ulid_from_sql_uuid)
                .collect(),
            contractor_ulid: row
                .try_get::<Vec<sqlx::types::Uuid>, _>("contractor_ulid")?
                .into_iter()
                .map(ulid_from_sql_uuid)
                .collect(),
            other_fields: InvoiceGroupIndexInternalSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, FromRow)]
struct InvoiceGroupIndexInternalSqlHelper {
    invoice_id: Vec<i64>,
    invoice_due: Vec<sqlx::types::time::Date>,
    invoice_status: Vec<String>,
    invoice_amount: Vec<sqlx::types::Decimal>,
}
