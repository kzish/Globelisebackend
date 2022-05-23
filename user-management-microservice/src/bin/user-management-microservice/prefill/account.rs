use crate::database::Database;
use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{DateWrapper, EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};

use email_address::EmailAddress;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::SharedDatabase;

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
    email: EmailAddress,
}

pub async fn individual_contractor_get_one(
    token: Token<UserAccessToken>,
    Query(query): Query<PrefillIndividualContractorDetailsQueryForUser>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Option<PrefillIndividualContractorAccountDetails>>> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    let result = database
        .select_one_client_prefill_individual_contractor_account_details(
            query.email,
            token.payload.ulid,
        )
        .await?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertOnePrefillIndividualContractorAccountDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Option<Ulid>,
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
    pub client_ulid: Option<Ulid>,
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
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub created_at: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub updated_at: sqlx::types::time::Date,
}

impl FromRow<'_, PgRow> for PrefillIndividualContractorAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row
                .try_get::<'_, String, &'static str>("email")?
                .parse()
                .unwrap(),
            client_ulid: row
                .try_get::<'_, Option<String>, &'static str>("client_ulid")?
                .map(|v| v.parse().unwrap()),
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

impl Database {
    pub async fn insert_one_client_prefill_individual_contractor_account_details(
        &self,
        client_ulid: Ulid,
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
            .bind(ulid_to_sql_uuid(client_ulid))
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

    pub async fn select_one_client_prefill_individual_contractor_account_details(
        &self,
        email: EmailAddress,
        client_ulid: Ulid,
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
            .bind(email.to_string())
            .bind(ulid_to_sql_uuid(client_ulid))
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}