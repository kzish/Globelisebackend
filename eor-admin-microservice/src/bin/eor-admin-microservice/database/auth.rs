use common_utils::{custom_serde::EmailWrapper, error::GlobeliseResult};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::Database;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Admin {
    pub ulid: Uuid,
    pub email: EmailWrapper,
    pub password: String,
    pub is_google: bool,
    pub is_outlook: bool,
}

impl Database {
    /// Creates and stores a new admin.
    pub async fn insert_one_admin(
        &self,
        email: EmailWrapper,
        password: String,
        is_google: bool,
        is_outlook: bool,
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "
            INSERT INTO admin_users (
                ulid, email, password, is_google, is_outlook
            ) VALUES (
                $1, $2, $3, $4, $5
            )",
        )
        .bind(ulid)
        .bind(email)
        .bind(password)
        .bind(is_google)
        .bind(is_outlook)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }

    /// Updates a admin's password.
    pub async fn update_one_admin_password(
        &self,
        ulid: Uuid,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE 
                admin_users 
            SET 
                password = $1 
            WHERE 
                ulid = $2",
        )
        .bind(new_password_hash)
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    /// Gets a admin's authentication information.
    pub async fn find_one_admin(
        &self,
        ulid: Option<Uuid>,
        email: Option<&EmailWrapper>,
    ) -> GlobeliseResult<Option<Admin>> {
        let query = "
            SELECT
                *
            FROM 
                admin_users
            WHERE 
                ($1 IS NULL OR ulid = $1) AND
                ($2 IS NULL OR email = $2)";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .bind(email)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
