use common_utils::error::GlobeliseResult;
use uuid::Uuid;

use crate::onboard::individual::IndividualDetails;

use super::Database;

impl Database {
    pub async fn onboard_admin_details(
        &self,
        ulid: Uuid,
        details: IndividualDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO onboard_eor_admins 
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($13, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            country = $6, city = $7, address = $8, postal_code = $9, tax_id = $10,
            time_zone = $11, profile_picture = $12"
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
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(ulid)
            .execute(&self.0)
            .await?;

        Ok(())
    }
}
