use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{Country, EmailWrapper, ImageData, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientAccountDetails {
    pub email: EmailWrapper,
    pub company_name: String,
    pub country: Country,
    pub entity_type: String,
    #[serde(default)]
    pub registration_number: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

pub async fn entity_client_post_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_prefilled_entity_client_account_details(
            body.email,
            body.company_name,
            body.country,
            body.entity_type,
            body.registration_number,
            body.tax_id,
            body.company_address,
            body.city,
            body.postal_code,
            body.time_zone,
            body.logo,
        )
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientGetOneQuery {
    email: EmailWrapper,
}

pub async fn entity_client_get_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<EntityClientGetOneQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillEntityClientAccountDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefilled_entity_client_account_details(query.email)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefilledIndividualContractorAccountDetails {
    pub email: EmailWrapper,
    pub client_ulid: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: Country,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
}

pub async fn individual_contractor_post_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefilledIndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_prefilled_individual_contractor_account_details(
            body.email,
            body.client_ulid,
            body.first_name,
            body.last_name,
            body.dob,
            body.dial_code,
            body.phone_number,
            body.country,
            body.city,
            body.address,
            body.postal_code,
            body.tax_id,
            body.time_zone,
        )
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorGetOneQuery {
    email: EmailWrapper,
    client_ulid: Uuid,
}

pub async fn individual_contractor_get_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<IndividualContractorGetOneQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefilledIndividualContractorAccountDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefilled_individual_contractor_account_details(query.email, query.client_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

impl Database {
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_prefilled_individual_contractor_account_details(
        &self,
        email: EmailWrapper,
        client_ulid: Uuid,
        first_name: String,
        last_name: String,
        dob: sqlx::types::time::OffsetDateTime,
        dial_code: String,
        phone_number: String,
        country: Country,
        city: String,
        address: String,
        postal_code: String,
        tax_id: Option<String>,
        time_zone: String,
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
                first_name = $3, last_name = $4, dob = $5, 
                dial_code = $6, phone_number = $7, country = $8, city = $9, address = $10, 
                postal_code = $11, tax_id = $12, time_zone = $13";

        sqlx::query(query)
            .bind(email)
            .bind(client_ulid)
            .bind(first_name)
            .bind(last_name)
            .bind(dob)
            .bind(dial_code)
            .bind(phone_number)
            .bind(country)
            .bind(city)
            .bind(address)
            .bind(postal_code)
            .bind(tax_id)
            .bind(time_zone)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_prefilled_individual_contractor_account_details(
        &self,
        email: EmailWrapper,
        client_ulid: Uuid,
    ) -> GlobeliseResult<Option<PrefilledIndividualContractorAccountDetails>> {
        let query = "
            SELECT
                *
            FROM
                prefilled_individual_contractors_account_details
            WHERE
                email = $1 AND
                client_ulid = $2";

        let result = sqlx::query_as(query)
            .bind(email)
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_prefilled_entity_client_account_details(
        &self,
        email: EmailWrapper,
        company_name: String,
        country: Country,
        entity_type: String,
        registration_number: Option<String>,
        tax_id: Option<String>,
        company_address: String,
        city: String,
        postal_code: String,
        time_zone: String,
        logo: Option<ImageData>,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_account_details (
                email, company_name, country, entity_type, registration_number, 
                tax_id, company_address, city, postal_code, time_zone, 
                logo
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10, 
                $11
            ) ON CONFLICT(email) DO UPDATE SET 
                company_name = $2, country = $3, entity_type = $4, registration_number = $5,
                tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
                logo = $11";

        sqlx::query(query)
            .bind(email)
            .bind(company_name)
            .bind(country)
            .bind(entity_type)
            .bind(registration_number)
            .bind(tax_id)
            .bind(company_address)
            .bind(city)
            .bind(postal_code)
            .bind(time_zone)
            .bind(logo)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_prefilled_entity_client_account_details(
        &self,
        email: EmailWrapper,
    ) -> GlobeliseResult<Option<PrefillEntityClientAccountDetails>> {
        let query = "
            SELECT 
                email, company_name, country, entity_type, registration_number, 
                tax_id, company_address, city, postal_code, time_zone,
                logo
            FROM
                prefilled_entity_clients_account_details
            WHERE
                email = $1";

        let result = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
