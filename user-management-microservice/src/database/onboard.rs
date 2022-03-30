use common_utils::error::{GlobeliseError, GlobeliseResult};
use rusty_ulid::Ulid;

use crate::{
    auth::user::{Role, UserType},
    onboard::{
        bank::BankDetails,
        entity::{EntityContractorDetails, EntityDetails, PicDetails},
        individual::{IndividualClientDetails, IndividualContractorDetails},
    },
};

use super::{ulid_to_sql_uuid, Database};

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
            .bind(
                details
                    .common_info
                    .profile_picture
                    .map(|b| b.as_ref().to_owned()),
            )
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
}
