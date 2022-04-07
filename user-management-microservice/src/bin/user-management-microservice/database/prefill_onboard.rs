use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    ulid_to_sql_uuid,
};

use crate::onboard::prefill::{PrefillBankDetails, PrefillIndividualDetails};

use super::Database;

impl Database {
    pub async fn prefill_onboard_individual_contractors_account_details(
        &self,
        details: PrefillIndividualDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO  prefilled_individual_contractors_account_details
            (email, client_ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT(email) DO UPDATE SET 
            first_name = $3, last_name = $4, dob = $5, dial_code = $6, phone_number = $7,
            country = $8, city = $9, address = $10, postal_code = $11, tax_id = $12,
            time_zone = $13";
        sqlx::query(query)
            .bind(details.email.to_string())
            .bind(ulid_to_sql_uuid(details.client_ulid))
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

    pub async fn prefill_onboard_individual_contractors_bank_details(
        &self,
        details: PrefillBankDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO  prefilled_individual_contractors_bank_details
            (email client_ulid, bank_name, bank_account_name, bank_account_number) 
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(email) DO UPDATE SET 
            bank_name = $3, bank_account_name = $4, bank_account_number = $5";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(details.client_ulid))
            .bind(details.email.to_string())
            .bind(details.bank_name)
            .bind(details.account_name)
            .bind(details.account_number)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}