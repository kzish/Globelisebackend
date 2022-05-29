use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use common_utils::{custom_serde::OffsetDateWrapper, error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use itertools::izip;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::Role};
use uuid::Uuid;

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

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceIndividualIndexQuery {
    pub invoice_group_ulid: Uuid,
    pub invoice_status: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceIndividualIndex {
    ulid: Uuid,
    invoice_group_ulid: Uuid,
    client_ulid: Uuid,
    contractor_ulid: Uuid,
    invoice_id: i64,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    invoice_due: sqlx::types::time::OffsetDateTime,
    invoice_status: String,
    invoice_amount: sqlx::types::Decimal,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceIndividualIndexSqlHelper {
    invoice_id: i64,
    #[serde_as(as = "FromInto<OffsetDateWrapper>")]
    invoice_due: sqlx::types::time::OffsetDateTime,
    invoice_status: String,
    invoice_amount: sqlx::types::Decimal,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvoiceGroupIndexQuery {
    pub invoice_status: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
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
    ulid: Vec<Uuid>,
    invoice_group_ulid: Uuid,
    client_ulid: Vec<Uuid>,
    contractor_ulid: Vec<Uuid>,
    other_fields: InvoiceGroupIndexInternalSqlHelper,
}

impl FromRow<'_, PgRow> for InvoiceGroupIndexInternal {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            ulid: row.try_get::<Vec<Uuid>, _>("ulid")?.into_iter().collect(),
            invoice_group_ulid: row.try_get("invoice_group_ulid")?,
            client_ulid: row
                .try_get::<Vec<Uuid>, _>("client_ulid")?
                .into_iter()
                .collect(),
            contractor_ulid: row
                .try_get::<Vec<Uuid>, _>("contractor_ulid")?
                .into_iter()
                .collect(),
            other_fields: InvoiceGroupIndexInternalSqlHelper::from_row(row)?,
        })
    }
}

#[derive(Debug, FromRow)]
struct InvoiceGroupIndexInternalSqlHelper {
    invoice_id: Vec<i64>,
    invoice_due: Vec<sqlx::types::time::OffsetDateTime>,
    invoice_status: Vec<String>,
    invoice_amount: Vec<sqlx::types::Decimal>,
}
