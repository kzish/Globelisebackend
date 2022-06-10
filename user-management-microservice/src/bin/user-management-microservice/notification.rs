use axum::{extract::Query, Extension, Json};
use common_utils::{
    calc_limit_and_offset,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{UserRole, UserType},
};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetNotificationRequest {
    user_role: UserRole,
    per_page: Option<u32>,
    page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PutNotificationRequest {
    ulid: Uuid,
    user_ulid: Uuid,
    user_role: UserRole,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Notification {
    ulid: Uuid,
    user_ulid: Uuid,
    message: String,
    read: bool,
}

pub async fn get_many(
    token: Token<UserAccessToken>,
    Query(body): Query<GetNotificationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<Notification>>> {
    let database = database.lock().await;

    if !token.payload.user_roles.contains(&body.user_role) {
        return Err(GlobeliseError::Forbidden);
    }

    let result = database
        .select_many_notifications(
            token.payload.user_type,
            body.user_role,
            token.payload.ulid,
            body.per_page,
            body.page,
        )
        .await?;

    Ok(Json(result))
}

pub async fn put_one(
    token: Token<UserAccessToken>,
    Query(body): Query<PutNotificationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !token.payload.user_roles.contains(&body.user_role) {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_one_notification_as_read(
            token.payload.user_type,
            body.user_role,
            token.payload.ulid,
            body.ulid,
        )
        .await?;

    Ok(())
}

impl Database {
    pub async fn update_one_notification_as_read(
        &self,
        user_type: UserType,
        user_role: UserRole,
        user_ulid: Uuid,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        let table = match (user_type, user_role) {
            (UserType::Individual, UserRole::Client) => "entity_client_notifications",
            (UserType::Individual, UserRole::Contractor) => "entity_contractor_notifications",
            (UserType::Entity, UserRole::Client) => "individual_client_notifications",
            (UserType::Entity, UserRole::Contractor) => "individual_contractor_notifications",
        };

        let query = format!(
            "
        UPDATE 
            {table} 
        SET 
            read = 't'
        WHERE 
            ulid = $1 AND
            user_ulid = $2",
        );

        sqlx::query(&query)
            .bind(ulid)
            .bind(user_ulid)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_many_notifications(
        &self,
        user_type: UserType,
        user_role: UserRole,
        user_ulid: Uuid,
        per_page: Option<u32>,
        page: Option<u32>,
    ) -> GlobeliseResult<Vec<Notification>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let table = match (user_type, user_role) {
            (UserType::Individual, UserRole::Client) => "entity_client_notifications",
            (UserType::Individual, UserRole::Contractor) => "entity_contractor_notifications",
            (UserType::Entity, UserRole::Client) => "individual_client_notifications",
            (UserType::Entity, UserRole::Contractor) => "individual_contractor_notifications",
        };

        let query = format!(
            "
        SELECT
            ulid, user_ulid, message, read
        FROM
            {table}
        WHERE
            user_ulid = $1
        LIMIT
            $2
        OFFSET
            $3"
        );

        let result = sqlx::query_as(&query)
            .bind(user_ulid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }
}
