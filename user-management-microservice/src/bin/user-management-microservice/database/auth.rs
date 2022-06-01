use common_utils::{
    custom_serde::EmailWrapper,
    error::{GlobeliseError, GlobeliseResult},
};
use sqlx::Row;
use user_management_microservice_sdk::user::UserType;
use uuid::Uuid;

use crate::auth::user::User;

use super::Database;

impl Database {
    /// Creates and stores a new user.
    pub async fn create_user(&self, user: User) -> GlobeliseResult<Uuid> {
        // Avoid overwriting an existing user.
        if self.user_id(&user.email).await?.is_some() {
            return Err(GlobeliseError::UnavailableEmail);
        }

        let ulid = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO users (
                ulid, email, password, is_google, is_outlook,
                is_entity, is_individual, is_client, is_contractor
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9
            )",
        )
        .bind(ulid)
        .bind(user.email.as_ref())
        .bind(user.password)
        .bind(user.is_google)
        .bind(user.is_outlook)
        .bind(user.is_entity)
        .bind(user.is_individual)
        .bind(user.is_client)
        .bind(user.is_contractor)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }

    /// Updates a user's password.
    pub async fn update_user_password_hash(
        &self,
        ulid: Uuid,
        user_type: UserType,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE 
                users 
            SET 
                password = $1 
            WHERE 
                ulid = $2 AND
                is_entity = $2 AND
                is_individual = $3",
        )
        .bind(new_password_hash)
        .bind(ulid)
        .bind(user_type == UserType::Entity)
        .bind(user_type == UserType::Individual)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    /// Gets a user's authentication information.
    ///
    /// If `user_type` is specified, this function only searches that type's table.
    /// Otherwise, it searches all user tables.
    pub async fn find_one_user(
        &self,
        ulid: Uuid,
        user_type: Option<UserType>,
    ) -> GlobeliseResult<Option<User>> {
        let maybe_user = sqlx::query_as(
            "
            SELECT 
                ulid, email, password, is_google, is_outlook, 
                is_entity, is_individual, is_client, is_contractor
            FROM 
                users
            WHERE 
                ulid = $1 AND
                ($2 IS NULL OR is_entity = $2) AND
                ($3 IS NULL OR is_individual = $3)",
        )
        .bind(ulid)
        .bind(user_type.map(|t| t == UserType::Entity))
        .bind(user_type.map(|t| t == UserType::Individual))
        .fetch_optional(&self.0)
        .await?;

        Ok(maybe_user)
    }

    /// Gets a user's id and user type.
    pub async fn user_id(&self, email: &EmailWrapper) -> GlobeliseResult<Option<(Uuid, UserType)>> {
        if let Some(row) = sqlx::query(
            "
                SELECT 
                    ulid, email, user_type
                FROM 
                    users_index
                WHERE 
                    email = $1",
        )
        .bind(email.as_ref())
        .fetch_optional(&self.0)
        .await?
        {
            Ok(Some((row.try_get("ulid")?, row.try_get("user_type")?)))
        } else {
            Ok(None)
        }
    }
}
