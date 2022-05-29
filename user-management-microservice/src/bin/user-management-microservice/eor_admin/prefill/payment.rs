use crate::database::Database;
use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{Currency, EmailWrapper, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};

use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::database::SharedDatabase;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertOnePrefillEntityClientPaymentDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientPaymentDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub updated_at: sqlx::types::time::OffsetDateTime,
}

impl FromRow<'_, PgRow> for PrefillEntityClientPaymentDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row
                .try_get::<'_, String, &'static str>("email")?
                .parse()
                .unwrap(),
            currency: row.try_get("currency")?,
            payment_date: row.try_get("payment_date")?,
            cutoff_date: row.try_get("cutoff_date")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

pub async fn entity_client_post_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<InsertOnePrefillEntityClientPaymentDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_prefill_entity_client_payment_details(body)
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientPaymentDetailsQueryForAdmin {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    email: EmailAddress,
}

pub async fn entity_client_get_one(
    _: Token<AdminAccessToken>,
    Query(query): Query<PrefillEntityClientPaymentDetailsQueryForAdmin>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillEntityClientPaymentDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefill_entity_client_payment_details(query.email)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

impl Database {
    pub async fn insert_one_prefill_entity_client_payment_details(
        &self,
        details: InsertOnePrefillEntityClientPaymentDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_payment_details
            (email, currency, payment_date, cutoff_date)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(email) DO UPDATE SET 
            currency = $2, payment_date = $3, cutoff_date = $4";

        sqlx::query(query)
            .bind(details.email.as_ref())
            .bind(details.currency)
            .bind(details.payment_date)
            .bind(details.cutoff_date)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn select_one_prefill_entity_client_payment_details(
        &self,
        email: EmailAddress,
    ) -> GlobeliseResult<Option<PrefillEntityClientPaymentDetails>> {
        let query = "
            SELECT
                email, currency, payment_date, cutoff_date
            FROM
                prefilled_entity_clients_payment_details
            WHERE
                email = $1";

        let result = sqlx::query_as(query)
            .bind(email.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
