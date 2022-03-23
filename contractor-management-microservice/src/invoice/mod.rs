use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use user_management_microservice_sdk::{AccessToken as UserAccessToken, UserType};

use crate::{
    common::ulid_to_sql_uuid,
    database::{Database, SharedDatabase},
};

impl Database {
    /// Index individual invoice
    pub async fn invoice_individual_index(
        &self,
        query: InvoiceIndividualIndexQuery,
    ) -> GlobeliseResult<Vec<InvoiceIndividualIndexForUser>> {
        let index = sqlx::query(
            "
                SELECT
                    ulid, invoice_group_ulid, contract_ulid, invoice_id,
                    invoice_due, invoice_status, invoice_amount
                FROM
                    invoice_individual_index
                WHERE
                    invoice_group_ulid = $1 AND
                    ($2 IS NULL OR (invoice_name ~* $2)) AND
                    ($3 IS NULL OR (contractor_ulid ~* $3)) AND
                    ($4 IS NULL OR (client_ulid ~* $4)) AND
                LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(query.invoice_group_ulid))
        .bind(query.search_text)
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?
        .into_iter()
        .map(InvoiceIndividualIndexForUser::from_pg_row)
        .collect::<GlobeliseResult<Vec<InvoiceIndividualIndexForUser>>>()?;

        Ok(index)
    }
}

pub async fn user_invoice_individual_index(
    claims: Token<UserAccessToken>,
    Query(query): Query<InvoiceIndividualIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceIndividualIndexForUser>>> {
    let ulid = claims.payload.ulid.parse::<Ulid>()?;
    let user_type = claims.payload.user_type.parse::<UserType>()?;
    if user_type == UserType::Individual && query.client_ulid == Some(ulid) {
        return Err(GlobeliseError::Unauthorized(
            "Contractor is not authorized to query other contractor's invoices",
        ));
    } else if user_type == UserType::Entity && query.contractor_ulid == Some(ulid) {
        return Err(GlobeliseError::Unauthorized(
            "Client is not authorized to query other client's invoices",
        ));
    } else {
        let database = database.lock().await;
        Ok(Json(database.invoice_individual_index(query).await?))
    }
}

pub async fn eor_admin_invoice_individual_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<InvoiceIndividualIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<InvoiceIndividualIndexForUser>>> {
    let database = database.lock().await;
    Ok(Json(database.invoice_individual_index(query).await?))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceIndividualIndexQuery {
    pub invoice_group_ulid: Ulid,
    pub client_ulid: Option<Ulid>,
    pub contractor_ulid: Option<Ulid>,
    pub invoice_status: Option<String>,
    pub search_text: Option<String>,
    page: i64,
    per_page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceIndividualIndexForUser {
    pub ulid: Ulid,
    pub invoice_group_ulid: Ulid,
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
    pub invoice_id: String,
    pub invoice_due: String,
    pub invoice_status: String,
    pub invoice_amount: u64,
}

impl InvoiceIndividualIndexForUser {
    pub fn from_pg_row(row: PgRow) -> GlobeliseResult<Self> {
        Ok(InvoiceIndividualIndexForUser {
            ulid: row.try_get::<String, _>("ulid")?.parse()?,
            invoice_group_ulid: row.try_get::<String, _>("invoice_group_ulid")?.parse()?,
            client_ulid: row.try_get::<String, _>("contractor_ulid")?.parse()?,
            contractor_ulid: row.try_get::<String, _>("contractor_ulid")?.parse()?,
            invoice_id: row.try_get::<String, _>("invoice_id")?,
            invoice_due: row
                .try_get::<sqlx::types::time::Date, _>("invoice_due")?
                .format("%Y-%m-%d"),
            invoice_status: row.try_get::<String, _>("invoice_status")?.parse()?,
            invoice_amount: row.try_get::<i64, _>("invoice_amount")?.try_into()?,
        })
    }
}
