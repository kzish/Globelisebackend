use sqlx::types::Uuid;

use crate::database::Database;
use crate::GlobeliseResult;

pub mod employment_information;
pub mod payroll_information;

impl Database {
    //check this pic is the owner of the entity
    pub async fn contractor_belongs_to_pic(
        &self,
        client_pic_ulid: Uuid,
        contractor_ulid: Uuid,
    ) -> GlobeliseResult<bool> {
        let response = sqlx::query(
            "SELECT
                *
            FROM
                client_contractor_pairs 
            WHERE client_ulid = $1
            AND contractor_ulid = $2",
        )
        .bind(client_pic_ulid)
        .bind(contractor_ulid)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(response)
    }
}
