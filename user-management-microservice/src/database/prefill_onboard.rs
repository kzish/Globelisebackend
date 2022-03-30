use common_utils::error::{GlobeliseError, GlobeliseResult};
use email_address::EmailAddress;

use crate::onboard::{bank::BankDetails, individual::IndividualContractorDetails};

use super::Database;

impl Database {
    pub async fn prefill_onboard_individual_contractors_account_details(
        &self,
        email: &EmailAddress,
        details: IndividualContractorDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO  prefilled_individual_contractors_account_details
            (email, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(email) DO UPDATE SET 
            first_name = $2, last_name = $3, dob = $4, dial_code = $5, phone_number = $6,
            country = $7, city = $8, address = $9, postal_code = $10, tax_id = $11,
            time_zone = $12";
        sqlx::query(query)
            .bind(email.to_string())
            .bind(details.common_info.first_name)
            .bind(details.common_info.last_name)
            .bind(details.common_info.dob)
            .bind(details.common_info.dial_code)
            .bind(details.common_info.phone_number)
            .bind(details.common_info.country)
            .bind(details.common_info.city)
            .bind(details.common_info.address)
            .bind(details.common_info.postal_code)
            .bind(details.common_info.tax_id)
            .bind(details.common_info.time_zone)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn prefill_onboard_individual_contractors_bank_details(
        &self,
        email: &EmailAddress,
        details: BankDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO  prefilled_individual_contractors_bank_details
            (email, bank_name, bank_account_name, bank_account_number) 
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(email) DO UPDATE SET 
            bank_name = $2, bank_account_name = $3, bank_account_number = $4";
        sqlx::query(query)
            .bind(email.to_string())
            .bind(details.bank_name)
            .bind(details.account_name)
            .bind(details.account_number)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}
