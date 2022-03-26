use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use itertools::izip;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use user_management_microservice_sdk::{AccessToken as UserAccessToken, Role};

use crate::{common::PaginationQuery, database::SharedDatabase};

mod database;

pub async fn user_invoice_individual_index(
    claims: Token<UserAccessToken>,
    Query(mut query): Query<InvoiceIndividualIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceIndividualIndex>>> {
    let ulid = claims.payload.ulid;

    // Override the provided query with the ulid provided by the tokens.
    match query.role {
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
    Query(mut query): Query<InvoiceGroupIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceGroupIndex>>> {
    let ulid = claims.payload.ulid;

    // Override the provided query with the ulid provided by the tokens.
    match query.role {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceIndividualIndexQuery {
    pub invoice_group_ulid: Ulid,
    pub client_ulid: Option<Ulid>,
    pub contractor_ulid: Option<Ulid>,
    pub invoice_status: Option<String>,
    #[serde(flatten)]
    pub paginated_search: PaginationQuery,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceIndividualIndex {
    pub ulid: Ulid,
    pub invoice_group_ulid: Ulid,
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
    pub invoice_id: String,
    pub invoice_due: String,
    pub invoice_status: String,
    pub invoice_amount: i64,
}

impl InvoiceIndividualIndex {
    pub fn from_pg_row(row: PgRow) -> GlobeliseResult<Self> {
        Ok(InvoiceIndividualIndex {
            ulid: row.try_get::<String, _>("ulid")?.parse()?,
            invoice_group_ulid: row.try_get::<String, _>("invoice_group_ulid")?.parse()?,
            client_ulid: row.try_get::<String, _>("contractor_ulid")?.parse()?,
            contractor_ulid: row.try_get::<String, _>("contractor_ulid")?.parse()?,
            invoice_id: row.try_get::<String, _>("invoice_id")?,
            invoice_due: row
                .try_get::<sqlx::types::time::Date, _>("invoice_due")?
                .format("%Y-%m-%d"),
            invoice_status: row.try_get::<String, _>("invoice_status")?.parse()?,
            invoice_amount: row.try_get::<i64, _>("invoice_amount")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceGroupIndexQuery {
    pub invoice_group_ulid: Ulid,
    pub client_ulid: Option<Ulid>,
    pub contractor_ulid: Option<Ulid>,
    pub invoice_status: Option<String>,
    #[serde(flatten)]
    pub paginated_search: PaginationQuery,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InvoiceGroupIndex {
    Single(InvoiceIndividualIndex),
    Bulk(Vec<InvoiceIndividualIndex>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceGroupSingle {
    pub ulid: Ulid,
    pub invoice_group_ulid: Ulid,
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
    pub invoice_id: String,
    pub invoice_due: String,
    pub invoice_status: String,
    pub invoice_amount: i64,
}

impl InvoiceGroupIndex {
    pub fn from_pg_row(row: PgRow) -> GlobeliseResult<Self> {
        let vec_ulid = row
            .try_get::<Vec<String>, _>("ulid")?
            .into_iter()
            .map(|s| s.parse::<Ulid>())
            .collect::<Result<Vec<_>, _>>()?;
        let invoice_group_ulid = row
            .try_get::<String, _>("invoice_group_ulid")?
            .parse::<Ulid>()?;
        let vec_client_ulid = row
            .try_get::<Vec<String>, _>("ulid")?
            .into_iter()
            .map(|s| s.parse::<Ulid>())
            .collect::<Result<Vec<_>, _>>()?;
        let vec_contractor_ulid = row
            .try_get::<Vec<String>, _>("ulid")?
            .into_iter()
            .map(|s| s.parse::<Ulid>())
            .collect::<Result<Vec<_>, _>>()?;
        let vec_invoice_id = row.try_get::<Vec<String>, _>("ulid")?;
        let vec_invoice_due = row
            .try_get::<Vec<sqlx::types::time::Date>, _>("invoice_due")?
            .into_iter()
            .map(|s| s.format("%Y-%m-%d"));
        let vec_invoice_status = row.try_get::<Vec<String>, _>("invoice_status")?;
        let vec_invoice_amount = row.try_get::<Vec<i64>, _>("invoice_amount")?;

        let mut all = izip!(
            vec_ulid.into_iter(),
            vec_client_ulid.into_iter(),
            vec_contractor_ulid.into_iter(),
            vec_invoice_id.into_iter(),
            vec_invoice_due,
            vec_invoice_status.into_iter(),
            vec_invoice_amount.into_iter()
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
                    invoice_group_ulid,
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
