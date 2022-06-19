use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    custom_serde::{Country, ImageData},
    database::Database,
    error::GlobeliseResult,
};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientAccountDetails {
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

impl Database {
    pub async fn insert_one_onboard_entity_client_account_details(
        &self,
        ulid: Uuid,
        details: EntityClientAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO entity_client_account_details (
                ulid, company_name, country, entity_type, registration_number, 
                tax_id, company_address, city, postal_code, time_zone, 
                logo
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11
            ) ON CONFLICT(ulid) DO UPDATE SET 
                company_name = $2, country = $3, entity_type = $4, registration_number = $5,
                tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
                logo = $11";

        sqlx::query(query)
            .bind(ulid)
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
            .await?;

        Ok(())
    }

    pub async fn select_one_onboard_entity_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<EntityClientAccountDetails>> {
        let query = "
            SELECT
                *
            FROM
                entity_client_account_details 
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorAccountDetails {
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
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

impl Database {
    pub async fn insert_one_onboard_entity_contractor_account_details(
        &self,
        ulid: Uuid,
        details: EntityContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO entity_contractor_account_details (
                ulid, company_name, country, entity_type, registration_number, 
                tax_id, company_address, city, postal_code, time_zone, 
                logo, company_profile
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12
            ) ON CONFLICT(ulid) DO UPDATE SET 
                company_name = $2, country = $3, entity_type = $4, registration_number = $5,
                tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
                logo = $11, company_profile = $12";
        sqlx::query(query)
            .bind(ulid)
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
            .bind(details.company_profile)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_onboard_entity_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<EntityContractorAccountDetails>> {
        let query = "
            SELECT
                *
            FROM
                entity_contractor_account_details
            WHERE
                ulid = $1";
        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
