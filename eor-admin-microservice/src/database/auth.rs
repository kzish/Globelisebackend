use common_utils::error::{GlobeliseError, GlobeliseResult};
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::Row;

use crate::auth::admin::Admin;

use super::{ulid_from_sql_uuid, ulid_to_sql_uuid, Database};

impl Database {
    /// Creates and stores a new admin.
    pub async fn create_admin(&self, admin: Admin) -> GlobeliseResult<Ulid> {
        if !admin.has_authentication() {
            return Err(GlobeliseError::Unauthorized(
                "Refused to create admin: no authentication method provided",
            ));
        }

        // Avoid overwriting an existing admin.
        match self.admin_id(&admin.email).await {
            Ok(Some(_)) => return Err(GlobeliseError::UnavailableEmail),
            Ok(None) => (),
            Err(e) => return Err(e),
        }

        let ulid = Ulid::generate();

        sqlx::query(
            "INSERT INTO auth_eor_admins
             (ulid, email, password, is_google, is_outlook)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(ulid_to_sql_uuid(ulid))
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
        ulid: Ulid,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> GlobeliseResult<()> {
        sqlx::query("UPDATE auth_eor_admins SET password = $1 WHERE ulid = $2")
            .bind(new_password_hash)
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await?;

        Ok(())
    }

    /// Gets a admin's authentication information.
    pub async fn admin(&self, ulid: Ulid) -> GlobeliseResult<Option<Admin>> {
        sqlx::query(
            "SELECT email, password, is_google, is_outlook
                FROM auth_eor_admins
                WHERE ulid = $1",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .fetch_optional(&self.0)
        .await?
        .map(Admin::from_pg_row)
        .transpose()
    }

    /// Gets a admin's id.
    pub async fn admin_id(&self, email: &EmailAddress) -> GlobeliseResult<Option<Ulid>> {
        let m_row = sqlx::query("SELECT ulid FROM auth_eor_admins WHERE email = $1")
            .bind(email.as_ref())
            .fetch_optional(&self.0)
            .await?;

        if let Some(row) = m_row {
            Ok(Some(ulid_from_sql_uuid(row.get("ulid"))))
        } else {
            Ok(None)
        }
    }
}
