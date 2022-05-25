use crate::database::Database;
use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{EmailWrapper, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use uuid::Uuid;

use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::SharedDatabase;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertOnePrefillIndividualContractorBankDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Uuid,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorBankDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Uuid,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub updated_at: sqlx::types::time::OffsetDateTime,
}

impl FromRow<'_, PgRow> for PrefillIndividualContractorBankDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row
                .try_get::<'_, String, &'static str>("email")?
                .parse()
                .unwrap(),
            client_ulid: row.try_get("client_ulid")?,
            bank_name: row.try_get("bank_name")?,
            bank_account_name: row.try_get("bank_account_name")?,
            bank_account_number: row.try_get("bank_account_number")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

pub async fn individual_contractor_post_one(
    token: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<InsertOnePrefillIndividualContractorBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database
        .insert_one_client_prefill_individual_contractor_bank_details(token.payload.ulid, body)
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorBankDetailsQuery {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    email: EmailAddress,
}

pub async fn individual_contractor_get_one(
    token: Token<UserAccessToken>,
    Query(query): Query<PrefillIndividualContractorBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorBankDetails>> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    let result = database
        .select_one_client_prefill_individual_contractor_bank_details(
            token.payload.ulid,
            query.email,
        )
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

impl Database {
    pub async fn insert_one_client_prefill_individual_contractor_bank_details(
        &self,
        client_ulid: Uuid,
        details: InsertOnePrefillIndividualContractorBankDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_individual_contractors_bank_details (
                email, client_ulid, bank_name, bank_account_name, bank_account_number
            ) VALUES (
                $1, $2, $3, $4, $5
            ) ON CONFLICT(email, client_ulid) DO UPDATE SET 
                bank_name = $2, bank_account_name = $3, bank_account_number = $4";

        sqlx::query(query)
            .bind(details.email.to_string())
            .bind(client_ulid)
            .bind(details.bank_name)
            .bind(details.bank_account_name)
            .bind(details.bank_account_number)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn select_one_client_prefill_individual_contractor_bank_details(
        &self,
        client_ulid: Uuid,
        email: EmailAddress,
    ) -> GlobeliseResult<Option<PrefillIndividualContractorBankDetails>> {
        let query = "
            SELECT
                email, client_ulid, bank_name, bank_account_name, bank_account_number
            FROM
                prefilled_individual_contractors_bank_details
            WHERE
                email = $1 AND
                client_ulid = $2";

        let result = sqlx::query_as(query)
            .bind(email.to_string())
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
