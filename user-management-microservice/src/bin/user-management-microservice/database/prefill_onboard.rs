use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    ulid_to_sql_uuid,
};

use crate::onboard::prefill::{PrefillBankDetails, PrefillIndividualDetails};

use super::Database;
use crate::onboard::entity::PrefillAuthEntities;
use crate::onboard::entity::PrefillEntityClientDetails;
use crate::onboard::entity::PrefilledPicDetails;
use crate::onboard::payment::PrefilledPaymentDetails;
use rusty_ulid::Ulid;

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
            (email, client_ulid, bank_name, bank_account_name, bank_account_number) 
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(email) DO UPDATE SET 
            bank_name = $3, bank_account_name = $4, bank_account_number = $5";
        sqlx::query(query)
            .bind(details.email.to_string())
            .bind(ulid_to_sql_uuid(details.client_ulid))
            .bind(details.bank_name)
            .bind(details.account_name)
            .bind(details.account_number)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn prefill_onboard_entity_client_details(
        &self,
        details: PrefillEntityClientDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_account_details
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $2, country = $3, entity_type = $4, registration_number = $5,
            tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
            logo = $11";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(details.client_ulid))
            .bind(details.common_info.company_name)
            .bind(details.common_info.country)
            .bind(details.common_info.entity_type)
            .bind(details.common_info.registration_number)
            .bind(details.common_info.tax_id)
            .bind(details.common_info.company_address)
            .bind(details.common_info.city)
            .bind(details.common_info.postal_code)
            .bind(details.common_info.time_zone)
            .bind(details.common_info.logo.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn prefill_onboard_auth_entity(
        &self,
        ulid: Ulid,
        details: PrefillAuthEntities,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO auth_entities
            (ulid, email, is_google, is_outlook)
            VALUES ($1, $2, false, false)
            ";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(details.email)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn prefill_onboard_entity_clients_bank_details(
        &self,
        details: PrefillBankDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_bank_details
            (ulid, bank_name, bank_account_name, bank_account_number)
            VALUES ($4, $1, $2, $3)
            ON CONFLICT(ulid) DO UPDATE SET 
            bank_name = $1, bank_account_name = $2, bank_account_number = $3";

        sqlx::query(query)
            .bind(details.bank_name)
            .bind(details.account_name)
            .bind(details.account_number)
            .bind(ulid_to_sql_uuid(details.client_ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn prefill_onboard_entity_client_payment_details(
        &self,
        details: PrefilledPaymentDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_payment_details
            (ulid, currency, payment_date, cutoff_date)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(ulid) DO UPDATE SET 
            currency = $2, payment_date = $3, cutoff_date = $4";

        sqlx::query(query)
            .bind(ulid_to_sql_uuid(details.client_ulid))
            .bind(details.currency)
            .bind(details.payment_date)
            .bind(details.cutoff_date)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn prefill_onboard_entity_client_pic_details(
        &self,
        details: PrefilledPicDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_pic_details
            (ulid, first_name, last_name, dob, dial_code, phone_number, profile_picture)
            VALUES ($7, $1, $2, $3, $4, $5, $6)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            profile_picture = $6";

        sqlx::query(query)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(ulid_to_sql_uuid(details.client_ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}
