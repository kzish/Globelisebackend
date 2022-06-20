use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    custom_serde::{EmailWrapper, OffsetDateWrapper},
    database::Database,
    error::GlobeliseResult,
};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorAccountDetails {
    pub email: EmailWrapper,
    pub client_ulid: Uuid,
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
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_client_prefill_individual_contractor_account_details(
        &self,
        client_ulid: Uuid,
        email: EmailWrapper,
        first_name: String,
        last_name: String,
        dob: sqlx::types::time::OffsetDateTime,
        dial_code: String,
        phone_number: String,
        country: String,
        city: String,
        address: String,
        postal_code: String,
        tax_id: Option<String>,
        time_zone: String,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_individual_contractor_account_details (
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

    pub async fn select_one_client_prefill_individual_contractor_account_details(
        &self,
        email: EmailWrapper,
        client_ulid: Uuid,
    ) -> GlobeliseResult<Option<PrefillIndividualContractorAccountDetails>> {
        let query = "
            SELECT
                *
            FROM
                prefilled_individual_contractor_account_details
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
