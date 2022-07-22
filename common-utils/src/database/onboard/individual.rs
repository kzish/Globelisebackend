use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    custom_serde::{Country, EmailWrapper, ImageData, OffsetDateWrapper},
    database::Database,
    error::GlobeliseResult,
};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientAccountDetails {
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
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}

impl Database {
    pub async fn insert_one_onboard_individual_client_account_details(
        &self,
        ulid: Uuid,
        details: IndividualClientAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO individual_client_account_details (
                ulid, first_name, last_name, dob, dial_code, 
                phone_number, country, city, address, postal_code, 
                tax_id, time_zone, profile_picture
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12, $13
            ) ON CONFLICT(ulid) DO UPDATE SET 
                first_name = $2, last_name = $3, dob = $4, dial_code = $5, 
                phone_number = $6, country = $7, city = $8, address = $9, postal_code = $10, 
                tax_id = $11, time_zone = $12, profile_picture = $13";
        sqlx::query(query)
            .bind(ulid)
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
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_onboard_individual_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<IndividualClientAccountDetails>> {
        let query = "
            SELECT 
                *
            FROM
                individual_client_account_details
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
pub struct IndividualContractorAccountDetails {
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
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub cv: Option<Vec<u8>>,
    pub gender: String,
    pub marital_status: String,
    pub nationality: Option<String>,
    pub email_address: Option<EmailWrapper>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expiry_date: Option<String>,
    pub work_permit: Option<String>,
    pub added_related_pay_item_id: Option<Uuid>,
    pub total_dependants: Option<i64>,
}

impl Database {
    pub async fn insert_one_onboard_individual_contractor_account_details(
        &self,
        ulid: Uuid,
        details: &IndividualContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO individual_contractor_account_details (
                ulid, first_name, last_name, dob, dial_code, 
                phone_number, country, city, address, postal_code, 
                tax_id, time_zone, profile_picture, cv, gender,
                marital_status, nationality, email_address, national_id, passport_number,
                passport_expiry_date, work_permit, added_related_pay_item_id, total_dependants
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22, $23, $24       
            ) ON CONFLICT(ulid) DO UPDATE SET 
                first_name = $2, last_name = $3, dob = $4, dial_code = $5, 
                phone_number = $6, country = $7, city = $8, address = $9, postal_code = $10, 
                tax_id = $11, time_zone = $12, profile_picture = $13, cv = $14, gender = $15,
                marital_status = $16, nationality = $17, email_address = $18, national_id = $19, passport_number = $20,
                passport_expiry_date = $21, work_permit = $22, added_related_pay_item_id = $23, total_dependants = $24";

        sqlx::query(query)
            .bind(ulid)
            .bind(&details.first_name)
            .bind(&details.last_name)
            .bind(details.dob)
            .bind(&details.dial_code)
            .bind(&details.phone_number)
            .bind(&details.country)
            .bind(&details.city)
            .bind(&details.address)
            .bind(&details.postal_code)
            .bind(&details.tax_id)
            .bind(&details.time_zone)
            .bind(details.profile_picture.as_ref().map(|b| b.to_owned()))
            .bind(&details.cv)
            .bind(&details.gender)
            .bind(&details.marital_status)
            .bind(&details.nationality)
            .bind(&details.email_address)
            .bind(&details.national_id)
            .bind(&details.passport_number)
            .bind(&details.passport_expiry_date)
            .bind(&details.work_permit)
            .bind(details.added_related_pay_item_id)
            .bind(details.total_dependants)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_onboard_individual_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<IndividualContractorAccountDetails>> {
        let query = "
            SELECT
                *
            FROM
                individual_contractor_account_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
