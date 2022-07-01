use crate::{
    calc_limit_and_offset,
    custom_serde::{OffsetDateWrapper, UserRole, UserType},
    error::GlobeliseResult,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use super::Database;

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NotificationIndex {
    notification_ulid: Uuid,
    user_ulid: Uuid,
    message: String,
    read: bool,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}

impl Database {
    pub async fn create_one_user_notification(&self, message: String) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "
        INSERT INTO user_notification (
            ulid, message
        ) VALUES (
            $1, $2
        )",
        )
        .bind(ulid)
        .bind(message)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }

    pub async fn create_one_user_see_notification_for_group_of_users(
        &self,
        notification_ulid: Uuid,
        user_type: Option<UserType>,
        user_role: Option<UserRole>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO user_see_notification (
            user_ulid, notification_ulid
        ) SELECT
            ulid AS user_ulid, $1 AS notification_ulid
        FROM 
            onboarded_user_index 
        WHERE
            ($2 IS NULL OR user_type = $2) AND
            ($3 IS NULL OR user_role = $3)
        ON CONFLICT (user_ulid, notification_ulid) DO NOTHING",
        )
        .bind(notification_ulid)
        .bind(user_type)
        .bind(user_role)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn create_one_user_see_notification_for_specific_users(
        &self,
        user_ulid: &[Uuid],
        notification_ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO user_see_notification (
            user_ulid, notification_ulid
        ) 
        SELECT
            *
        FROM UNNEST (
            $1, $2
        ) RETURNING
            user_ulid, notification_ulid",
        )
        .bind(user_ulid)
        .bind(
            std::iter::repeat(notification_ulid)
                .take(user_ulid.len())
                .collect::<Vec<_>>(),
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_one_notification_as_read(
        &self,
        user_ulid: Uuid,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        UPDATE 
            user_see_notification
        SET 
            read = 't'
        WHERE 
            notification_ulid = $1 AND
            user_ulid = $2",
        )
        .bind(ulid)
        .bind(user_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn select_many_user_notifications(
        &self,
        user_ulid: Option<Uuid>,
        read: Option<bool>,
        query: Option<String>,
        per_page: Option<u32>,
        page: Option<u32>,
    ) -> GlobeliseResult<Vec<NotificationIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
        SELECT
            *
        FROM
            user_notification_index
        WHERE
            ($1 IS NULL OR user_ulid = $1) AND
            ($2 IS NULL OR read = $2) AND
            ($3 IS NULL OR message ~* $3)
        LIMIT
            $4
        OFFSET
            $5",
        )
        .bind(user_ulid)
        .bind(read)
        .bind(query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn select_many_admin_notifications(
        &self,
        admin_ulid: Option<Uuid>,
        read: Option<bool>,
        query: Option<String>,
        per_page: Option<u32>,
        page: Option<u32>,
    ) -> GlobeliseResult<Vec<NotificationIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
        SELECT
            *
        FROM
            admin_notification_index
        WHERE
            ($1 IS NULL OR admin_ulid = $1) AND
            ($2 IS NULL OR read = $2) AND
            ($3 IS NULL OR query ~* $3)
        LIMIT
            $4
        OFFSET
            $5",
        )
        .bind(admin_ulid)
        .bind(read)
        .bind(query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }
}
