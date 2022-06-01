use common_utils::{
    custom_serde::EmailWrapper,
    error::{GlobeliseError, GlobeliseResult},
};
use sqlx::Row;
use uuid::Uuid;

use crate::auth::admin::Admin;

use super::Database;

impl Database {
    /// Creates and stores a new admin.
    pub async fn create_admin(&self, admin: Admin) -> GlobeliseResult<Uuid> {
        if !admin.has_authentication() {
            return Err(GlobeliseError::unauthorized(
                "Cannot create user without any authentication method provided",
            ));
        }

        // Avoid overwriting an existing admin.
        match self.admin_id(&admin.email).await {
            Ok(Some(_)) => return Err(GlobeliseError::UnavailableEmail),
            Ok(None) => (),
            Err(e) => return Err(e),
        }

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
        .bind(admin.email)
        .bind(admin.password)
        .bind(admin.is_google)
        .bind(admin.is_outlook)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }

    /// Updates a admin's password.
    pub async fn update_password(
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
    pub async fn admin(&self, ulid: Uuid) -> GlobeliseResult<Option<Admin>> {
        Ok(sqlx::query_as(
            "
            SELECT 
                email, password, is_google, is_outlook
            FROM 
                admin_users
            WHERE 
                ulid = $1",
        )
        .bind(ulid)
        .fetch_optional(&self.0)
        .await?)
    }

    /// Gets a admin's id.
    pub async fn admin_id(&self, email: &EmailWrapper) -> GlobeliseResult<Option<Uuid>> {
        let m_row = sqlx::query(
            "
            SELECT 
                ulid 
            FROM 
                admin_users 
            WHERE 
                email = $1",
        )
        .bind(email.as_ref())
        .fetch_optional(&self.0)
        .await?;

        if let Some(row) = m_row {
            Ok(Some(row.get("ulid")))
        } else {
            Ok(None)
        }
    }
}
