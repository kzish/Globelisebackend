use common_utils::error::{GlobeliseError, GlobeliseResult};
use email_address::EmailAddress;

use crate::onboard::{bank::BankDetails, individual::IndividualDetails};

use super::Database;

impl Database {
    pub async fn prefill_onboard_individual_contractors_account_details(
        &self,
        email: &EmailAddress,
        details: IndividualDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO  prefilled_onboard_individual_contractors
            (email, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($12, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(email) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            country = $6, city = $7, address = $8, postal_code = $9, tax_id = $10,
            time_zone = $11"
            .to_string();
        sqlx::query(&query)
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
            .bind(email.to_string())
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
            INSERT INTO  prefilled_onboard_individual_contractors_bank_details
            (email, bank_name, bank_account_name, bank_account_number) 
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(email) DO UPDATE SET 
            bank_name = $2, bank_account_name = $3, bank_account_number = $4"
            .to_string();
        sqlx::query(&query)
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
