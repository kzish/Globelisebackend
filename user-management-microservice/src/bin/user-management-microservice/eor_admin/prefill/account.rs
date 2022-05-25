use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{
        DateWrapper, EmailWrapper, ImageData, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientAccountDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Option<Uuid>,
    pub company_name: String,
    pub country: String,
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
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

impl FromRow<'_, PgRow> for PrefillEntityClientAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row
                .try_get::<'_, String, &'static str>("email")?
                .parse()
                .unwrap(),
            client_ulid: row
                .try_get::<'_, Option<String>, &'static str>("email")?
                .map(|v| v.parse().unwrap()),
            company_name: row.try_get("company_name")?,
            country: row.try_get("country")?,
            entity_type: row.try_get("entity_type")?,
            registration_number: row.try_get("registration_number")?,
            tax_id: row.try_get("tax_id")?,
            company_address: row.try_get("company_address")?,
            city: row.try_get("city")?,
            postal_code: row.try_get("postal_code")?,
            time_zone: row.try_get("time_zone")?,
            logo: row
                .try_get::<'_, Option<Vec<u8>>, &'static str>("logo")?
                .map(ImageData),
            company_profile: row.try_get("company_profile")?,
        })
    }
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
        .insert_one_prefill_entity_client_account_details(body)
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientGetOneQuery {
    email: EmailAddress,
}

pub async fn entity_client_get_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<EntityClientGetOneQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillEntityClientAccountDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefill_entity_client_account_details(query.email)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertOnePrefillIndividualContractorAccountDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorAccountDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub updated_at: sqlx::types::time::OffsetDateTime,
}

impl FromRow<'_, PgRow> for PrefillIndividualContractorAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row
                .try_get::<'_, String, &'static str>("email")?
                .parse()
                .unwrap(),
            client_ulid: row.try_get("client_ulid")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            dob: row.try_get("dob")?,
            dial_code: row.try_get("dial_code")?,
            phone_number: row.try_get("phone_number")?,
            country: row.try_get("country")?,
            city: row.try_get("city")?,
            address: row.try_get("address")?,
            postal_code: row.try_get("postal_code")?,
            tax_id: row.try_get("tax_id")?,
            time_zone: row.try_get("time_zone")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

pub async fn individual_contractor_post_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<InsertOnePrefillIndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_prefill_individual_contractor_account_details(body)
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorGetOneQuery {
    email: EmailAddress,
    client_ulid: Option<Uuid>,
}

pub async fn individual_contractor_get_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<IndividualContractorGetOneQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorAccountDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefill_individual_contractor_account_details(query.email, query.client_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

impl Database {
    pub async fn insert_one_prefill_individual_contractor_account_details(
        &self,
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
            .bind(details.email.to_string())
            .bind(details.client_ulid)
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
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn select_one_prefill_individual_contractor_account_details(
        &self,
        email: EmailAddress,
        client_ulid: Option<Uuid>,
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
                ($2 IS NULL OR client_ulid = $2)";

        let result = sqlx::query_as(query)
            .bind(email.to_string())
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }

    pub async fn insert_one_prefill_entity_client_account_details(
        &self,
        details: PrefillEntityClientAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_account_details
            (email, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(email) DO UPDATE SET 
            company_name = $2, country = $3, entity_type = $4, registration_number = $5,
            tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
            logo = $11";

        sqlx::query(query)
            .bind(details.email.to_string())
            .bind(details.company_name)
            .bind(details.country)
            .bind(details.entity_type)
            .bind(details.registration_number)
            .bind(details.tax_id)
            .bind(details.company_address)
            .bind(details.city)
            .bind(details.postal_code)
            .bind(details.time_zone)
            .bind(details.logo.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn select_one_prefill_entity_client_account_details(
        &self,
        email: EmailAddress,
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
            .bind(email.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
