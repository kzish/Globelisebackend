use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::Row;
use strum::IntoEnumIterator;

use crate::auth::user::{User, UserType};

use super::Database;

impl Database {
    /// Creates and stores a new user.
    pub async fn create_user(&self, user: User, user_type: UserType) -> GlobeliseResult<Ulid> {
        if !user.has_authentication() {
            return Err(GlobeliseError::Unauthorized(
                "Refused to create user: no authentication method provided",
            ));
        }

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
        let types_to_check = match user_type {
            Some(t) => vec![t],
            None => UserType::iter().collect(),
        };

        for t in types_to_check {
            let user = sqlx::query(&format!(
                "SELECT email, password, is_google, is_outlook
                FROM {}
                WHERE ulid = $1",
                t.db_auth_name()
            ))
            .bind(ulid_to_sql_uuid(ulid))
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

            if let Some(user) = user {
                return Ok(Some((
                    User {
                        email: user.get::<String, _>("email").parse().map_err(|_| {
                            GlobeliseError::Internal("Invalid email address from database".into())
                        })?,
                        password_hash: user.get("password"),
                        google: user.get("is_google"),
                        outlook: user.get("is_outlook"),
                    },
                    t,
                )));
            }
        }
        Ok(None)
    }

    /// Gets a user's id and user type.
    pub async fn user_id(&self, email: &EmailAddress) -> GlobeliseResult<Option<(Ulid, UserType)>> {
        for t in UserType::iter() {
            let id = sqlx::query(&format!(
                "SELECT ulid FROM {} WHERE email = $1",
                t.db_auth_name()
            ))
            .bind(email.as_ref())
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

            if let Some(id) = id {
                return Ok(Some((ulid_from_sql_uuid(id.get("ulid")), t)));
            }
        }
        Ok(None)
    }
}
