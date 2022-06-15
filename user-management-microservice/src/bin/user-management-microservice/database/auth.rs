use common_utils::{custom_serde::EmailWrapper, error::GlobeliseResult};
use user_management_microservice_sdk::user::UserType;
use uuid::Uuid;

use crate::auth::user::User;

use super::Database;

impl Database {
    /// Creates and stores a new user.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_user(
        &self,
        email: EmailWrapper,
        password: Option<String>,
        is_google: bool,
        is_outlook: bool,
        is_entity: bool,
        is_individual: bool,
        is_client: bool,
        is_contractor: bool,
    ) -> GlobeliseResult<Uuid> {
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
        .bind(email)
        .bind(password)
        .bind(is_google)
        .bind(is_outlook)
        .bind(is_entity)
        .bind(is_individual)
        .bind(is_client)
        .bind(is_contractor)
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

    pub async fn find_one_user(
        &self,
        ulid: Option<Uuid>,
        email: Option<&EmailWrapper>,
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
                ($1 IS NULL OR ulid = $1) AND
                ($2 IS NULL OR email = $2) AND
                ($3 IS NULL OR is_entity = $3) AND
                ($4 IS NULL OR is_individual = $4)",
        )
        .bind(ulid)
        .bind(email)
        .bind(user_type.map(|t| t == UserType::Entity))
        .bind(user_type.map(|t| t == UserType::Individual))
        .fetch_optional(&self.0)
        .await?;

        Ok(maybe_user)
    }
}
