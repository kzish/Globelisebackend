use uuid::Uuid;

use crate::{
    custom_serde::{UserRole, UserType},
    database::Database,
    error::GlobeliseResult,
};

pub mod bank;
pub mod entity;
pub mod individual;
pub mod payment;
pub mod pic;

impl Database {
    pub async fn get_is_user_fully_onboarded(
        &self,
        ulid: Uuid,
        user_type: UserType,
        user_role: UserRole,
    ) -> GlobeliseResult<bool> {
        let table_name = match (user_type, user_role) {
            (UserType::Individual, UserRole::Client) => "individual_client_fully_onboarded",
            (UserType::Individual, UserRole::Contractor) => "individual_contractor_fully_onboarded",
            (UserType::Entity, UserRole::Client) => "entity_client_fully_onboarded",
            (UserType::Entity, UserRole::Contractor) => "entity_contractor_fully_onboarded",
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
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?
            .is_some(); // This will also return false if there's an Err

        Ok(result)
    }
}
