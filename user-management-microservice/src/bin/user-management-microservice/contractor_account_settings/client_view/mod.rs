use crate::database::Database;
use common_utils::error::GlobeliseResult;
use uuid::Uuid;

pub mod entity;
pub mod individual;

impl Database {
    pub async fn is_client_contractor_pair(
        &self,
        client_ulid: Uuid,
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
        .bind(client_ulid)
        .bind(contractor_ulid)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(response)
    }
}
