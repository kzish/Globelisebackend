use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use user_management_microservice_sdk::user::{Role, UserType};

use crate::onboard::{
    bank::{BankDetails, PaymentDetails},
    entity::{EntityContractorDetails, EntityDetails, PicDetails},
    individual::{IndividualClientDetails, IndividualContractorDetails},
};

use super::Database;

impl Database {
    pub async fn onboard_individual_client_details(
        &self,
        ulid: Ulid,
        details: IndividualClientDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO individual_clients_account_details
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $2, last_name = $3, dob = $4, dial_code = $5, phone_number = $6,
            country = $7, city = $8, address = $9, postal_code = $10, tax_id = $11,
            time_zone = $12, profile_picture = $13";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
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
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_individual_contractor_details(
        &self,
        ulid: Ulid,
        details: IndividualContractorDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO individual_contractors_account_details
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture, cv) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $2, last_name = $3, dob = $4, dial_code = $5, phone_number = $6,
            country = $7, city = $8, address = $9, postal_code = $10, tax_id = $11,
            time_zone = $12, profile_picture = $13, cv = $14";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
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
            .bind(details.cv)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_entity_client_details(
        &self,
        ulid: Ulid,
        details: EntityDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO entity_clients_account_details
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $2, country = $3, entity_type = $4, registration_number = $5,
            tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
            logo = $11";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
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

    pub async fn onboard_entity_contractor_details(
        &self,
        ulid: Ulid,
        details: EntityContractorDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO entity_contractors_account_details
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo, company_profile)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $2, country = $3, entity_type = $4, registration_number = $5,
            tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
            logo = $11, company_profile = $12";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
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
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_pic_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: PicDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = UserType::Entity.db_onboard_details_prefix(role) + "_pic_details";
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, first_name, last_name, dob, dial_code, phone_number, profile_picture)
            VALUES ($7, $1, $2, $3, $4, $5, $6)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            profile_picture = $6",
        );

        sqlx::query(&query)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_bank_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        details: BankDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = user_type.db_onboard_details_prefix(Role::Contractor) + "_bank_details";
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, bank_name, bank_account_name, bank_account_number)
            VALUES ($4, $1, $2, $3)
            ON CONFLICT(ulid) DO UPDATE SET 
            bank_name = $1, bank_account_name = $2, bank_account_number = $3",
        );
        sqlx::query(&query)
            .bind(details.bank_name)
            .bind(details.account_name)
            .bind(details.account_number)
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_payment_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        details: PaymentDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = user_type.db_onboard_details_prefix(Role::Client) + "_payment_details";
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, currency, payment_date, cutoff_date)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(ulid) DO UPDATE SET 
            currency = $2, payment_date = $3, cutoff_date = $4",
        );
        sqlx::query(&query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(details.currency)
            .bind(details.payment_date)
            .bind(details.cutoff_date)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_is_user_fully_onboarded(
        &self,
        ulid: Ulid,
        user_type: UserType,
        user_role: Role,
    ) -> GlobeliseResult<bool> {
        let table_name = match (user_type, user_role) {
            (UserType::Individual, Role::Client) => "individual_clients_fully_onboarded",
            (UserType::Individual, Role::Contractor) => "individual_contractors_fully_onboarded",
            (UserType::Entity, Role::Client) => "entity_clients_fully_onboarded",
            (UserType::Entity, Role::Contractor) => "entity_contractors_fully_onboarded",
        };
        let query = format!(
            "
            SELECT 
                1
            FROM
                {table_name}
            WHERE
                ulid = $1",
        );
        let result = sqlx::query(&query)
            .bind(ulid_to_sql_uuid(ulid))
            .fetch_optional(&self.0)
            .await?
            .is_some(); // This will also return false if there's an Err

        Ok(result)
    }
}
