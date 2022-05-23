use common_utils::error::{GlobeliseError, GlobeliseResult};
use email_address::EmailAddress;
use sqlx::Row;
use uuid::Uuid;

use crate::auth::admin::Admin;

use super::Database;

impl Database {
    /// Creates and stores a new admin.
    pub async fn create_admin(&self, admin: Admin) -> GlobeliseResult<Uuid> {
        if !admin.has_authentication() {
            return Err(GlobeliseError::unauthorized(
                "Refused to create admin: no authentication method provided",
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
            "INSERT INTO auth_eor_admins
             (ulid, email, password, is_google, is_outlook)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(ulid)
        .bind(admin.email.as_ref())
        .bind(admin.password_hash)
        .bind(admin.google)
        .bind(admin.outlook)
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
        sqlx::query("UPDATE auth_eor_admins SET password = $1 WHERE ulid = $2")
            .bind(new_password_hash)
            .bind(ulid)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    /// Gets a admin's authentication information.
    pub async fn admin(&self, ulid: Uuid) -> GlobeliseResult<Option<Admin>> {
        Ok(sqlx::query_as(
            "SELECT email, password, is_google, is_outlook
                FROM auth_eor_admins
                WHERE ulid = $1",
        )
        .bind(ulid)
        .fetch_optional(&self.0)
        .await?)
    }

    /// Gets a admin's id.
    pub async fn admin_id(&self, email: &EmailAddress) -> GlobeliseResult<Option<Uuid>> {
        let m_row = sqlx::query("SELECT ulid FROM auth_eor_admins WHERE email = $1")
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
