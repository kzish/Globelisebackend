use common_utils::error::{GlobeliseError, GlobeliseResult};
use rusty_ulid::Ulid;

use crate::{
    auth::user::{Role, UserType},
    onboard::{
        bank::{BankDetails, PaymentDetails},
        entity::{EntityDetails, PicDetails},
        individual::IndividualDetails,
    },
};

use super::{ulid_to_sql_uuid, Database};

impl Database {
    pub async fn onboard_individual_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: IndividualDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table =
            UserType::Individual.db_onboard_details_prefix(role) + "_account_details";
        let query = format!(
            "
            INSERT INTO {target_table} 
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($13, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            country = $6, city = $7, address = $8, postal_code = $9, tax_id = $10,
            time_zone = $11, profile_picture = $12",
        );
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
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_entity_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: EntityDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = UserType::Entity.db_onboard_details_prefix(role) + "_account_details";
        sqlx::query(&format!(
            "
            INSERT INTO {target_table}
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo)
            VALUES ($11, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $1, country = $2, entity_type = $3, registration_number = $4,
            tax_id = $5, company_address = $6, city = $7, postal_code = $8, time_zone = $9,
            logo = $10"
        ))
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
        .bind(ulid_to_sql_uuid(ulid))
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
}
