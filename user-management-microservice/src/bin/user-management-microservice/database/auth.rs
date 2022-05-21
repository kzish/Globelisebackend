use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::Row;
use user_management_microservice_sdk::user::UserType;

use crate::auth::user::User;

use super::Database;

impl Database {
    /// Creates and stores a new user.
    pub async fn create_user(&self, user: User, user_type: UserType) -> GlobeliseResult<Ulid> {
        // Avoid overwriting an existing user.
        if self.user_id(&user.email).await?.is_some() {
            return Err(GlobeliseError::UnavailableEmail);
        }

        let ulid = Ulid::generate();

        sqlx::query(&format!(
            "INSERT INTO {} (ulid, email, password, is_google, is_outlook)
            VALUES ($1, $2, $3, $4, $5)",
            user_type.db_auth_name()
        ))
        .bind(ulid_to_sql_uuid(ulid))
        .bind(user.email.as_ref())
        .bind(user.password_hash)
        .bind(user.google)
        .bind(user.outlook)
        .execute(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(ulid)
    }

    /// Updates a user's password.
    pub async fn update_password(
        &self,
        ulid: Ulid,
        user_type: UserType,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> GlobeliseResult<()> {
        sqlx::query(&format!(
            "UPDATE {} SET password = $1 WHERE ulid = $2",
            user_type.db_auth_name()
        ))
        .bind(new_password_hash)
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Gets a user's authentication information.
    ///
    /// If `user_type` is specified, this function only searches that type's table.
    /// Otherwise, it searches all user tables.
    pub async fn user(
        &self,
        ulid: Ulid,
        user_type: Option<UserType>,
    ) -> GlobeliseResult<Option<(User, UserType)>> {
        if let Some(row) = sqlx::query(
            "
            SELECT 
                ulid, email, password, is_google, is_outlook, is_client,
                is_contractor, user_type
            FROM 
                users_index
            WHERE 
                ulid = $1 AND
                ($2 IS NULL OR user_type = $2)",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .bind(user_type.map(|v| v.as_str()))
        .fetch_optional(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?
        {
            Ok(Some((
                User {
                    email: row.try_get::<String, _>("email")?.parse().map_err(|_| {
                        GlobeliseError::internal("Invalid email address from database")
                    })?,
                    password_hash: row.try_get("password")?,
                    google: row.try_get("is_google")?,
                    outlook: row.try_get("is_outlook")?,
                },
                row.try_get::<String, _>("user_type")?
                    .parse()
                    .map_err(|_| GlobeliseError::internal("Invalid user_type from database"))?,
            )))
        } else {
            Ok(None)
        }
    }

    /// Gets a user's id and user type.
    pub async fn user_id(&self, email: &EmailAddress) -> GlobeliseResult<Option<(Ulid, UserType)>> {
        if let Some(row) = sqlx::query(
            "
                SELECT 
                    email, ulid, user_type
                FROM 
                    users_index
                WHERE 
                    email = $1",
        )
        .bind(email.as_ref())
        .fetch_optional(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?
        {
            Ok(Some((
                ulid_from_sql_uuid(row.get("ulid")),
                row.try_get::<String, _>("user_type")?
                    .parse()
                    .map_err(|_| GlobeliseError::internal("Invalid user_type from database"))?,
            )))
        } else {
            Ok(None)
        }
    }
}
