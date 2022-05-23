use axum::{extract::Path, Extension, Json};
use common_utils::{error::GlobeliseResult, token::Token};
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{Role, UserType},
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
    Path(role): Path<Role>,
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
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?
            .is_some(); // This will also return false if there's an Err

        Ok(result)
    }
}
