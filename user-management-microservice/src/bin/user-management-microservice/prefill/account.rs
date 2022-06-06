use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{EmailWrapper, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

pub async fn individual_contractor_post_one(
    token: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<InsertOnePrefillIndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database
        .insert_one_client_prefill_individual_contractor_account_details(token.payload.ulid, body)
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorDetailsQueryForUser {
    email: EmailWrapper,
}

pub async fn individual_contractor_get_one(
    token: Token<UserAccessToken>,
    Query(query): Query<PrefillIndividualContractorDetailsQueryForUser>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorAccountDetails>> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    let result = database
        .select_one_client_prefill_individual_contractor_account_details(
            query.email,
            token.payload.ulid,
        )
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertOnePrefillIndividualContractorAccountDetails {
    pub email: EmailWrapper,
    pub client_ulid: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorAccountDetails {
    pub email: EmailWrapper,
    pub client_ulid: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
}

impl Database {
    pub async fn insert_one_client_prefill_individual_contractor_account_details(
        &self,
        client_ulid: Uuid,
        details: InsertOnePrefillIndividualContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_individual_contractors_account_details (
                email, client_ulid, first_name, last_name, dob, 
                dial_code, phone_number, country, city, address, 
                postal_code, tax_id, time_zone
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12, $13
            ) ON CONFLICT(email, client_ulid) DO UPDATE SET 
                first_name = $3, last_name = $4, dob = $5, dial_code = $6, phone_number = $7, 
                country = $8, city = $9, address = $10, postal_code = $11, tax_id = $12, 
                time_zone = $13";

        sqlx::query(query)
            .bind(details.email)
            .bind(client_ulid)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.country)
            .bind(details.city)
            .bind(details.address)
            .bind(details.postal_code)
            .bind(details.tax_id)
            .bind(details.time_zone)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_client_prefill_individual_contractor_account_details(
        &self,
        email: EmailWrapper,
        client_ulid: Uuid,
    ) -> GlobeliseResult<Option<PrefillIndividualContractorAccountDetails>> {
        let query = "
            SELECT
                email, client_ulid, first_name, last_name, dob, 
                dial_code, phone_number, country, city, address, 
                postal_code, tax_id, time_zone
            FROM
                prefilled_individual_contractors_account_details
            WHERE
                email = $1 AND
                client_ulid =$2";

        let result = sqlx::query_as(query)
            .bind(email)
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
