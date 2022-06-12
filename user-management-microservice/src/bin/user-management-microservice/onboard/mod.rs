use axum::{extract::Path, Extension, Json};
use common_utils::{error::GlobeliseResult, token::Token};
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{UserRole, UserType},
};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

pub mod bank;
pub mod entity;
pub mod individual;
pub mod payment;
pub mod pic;

pub async fn fully_onboarded(
    claims: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<bool>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .get_is_user_fully_onboarded(claims.payload.ulid, claims.payload.user_type, role)
            .await?,
    ))
}

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
