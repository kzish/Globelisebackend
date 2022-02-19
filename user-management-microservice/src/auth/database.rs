use std::{sync::Arc, time::Duration};

use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use strum::IntoEnumIterator;
use tokio::sync::Mutex;

use super::{
    onboarding::IndividualDetails,
    user::{Role, User},
    Error,
};

pub type SharedDatabase = Arc<Mutex<Database>>;

/// Convenience wrapper around PostgreSQL.
pub struct Database(Pool<Postgres>);

impl Database {
    /// Connects to PostgreSQL.
    pub async fn new() -> Self {
        let connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_timeout(Duration::from_secs(3))
            .connect(&connection_str)
            .await
            .expect("Cannot connect to database");

        Self(pool)
    }

    /// Creates and stores a new user.
    pub async fn create_user(&self, user: User, role: Role) -> Result<Ulid, Error> {
        if !user.has_authentication() {
            return Err(Error::BadRequest);
        }

        // Avoid overwriting an existing user.
        if self.user_id(&user.email, role).await?.is_some() {
            return Err(Error::BadRequest);
        }

        let ulid = Ulid::generate();

        sqlx::query(&format!(
            "INSERT INTO {} (ulid, email, password, is_google, is_outlook)
                VALUES ($1, $2, $3, $4, $5)",
            Self::user_table_name(role)
        ))
        .bind(ulid_to_sql_uuid(ulid))
        .bind(user.email.as_ref())
        .bind(user.password_hash)
        .bind(user.google)
        .bind(user.outlook)
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(ulid)
    }

    /// Updates a user's password.
    pub async fn update_password(
        &self,
        ulid: Ulid,
        role: Role,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> Result<(), Error> {
        sqlx::query(&format!(
            "UPDATE {} SET password = $1 WHERE ulid = $2",
            Self::user_table_name(role)
        ))
        .bind(new_password_hash)
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    /// Gets a user's information.
    ///
    /// If `role` is specified, this function only searches that role's table.
    /// Otherwise, it searches all user tables.
    pub async fn user(
        &self,
        ulid: Ulid,
        role: Option<Role>,
    ) -> Result<Option<(User, Role)>, Error> {
        let roles_to_check = match role {
            Some(role) => vec![role],
            None => Role::iter().collect(),
        };

        for r in roles_to_check {
            let user = sqlx::query(&format!(
                "SELECT email, password, is_google, is_outlook
                FROM {}
                WHERE ulid = $1",
                Self::user_table_name(r)
            ))
            .bind(ulid_to_sql_uuid(ulid))
            .fetch_optional(&self.0)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

            if let Some(user) = user {
                return Ok(Some((
                    User {
                        email: user
                            .get::<String, _>("email")
                            .parse()
                            .map_err(|_| Error::Conversion("email parse error".into()))?,
                        password_hash: user.get("password"),
                        google: user.get("is_google"),
                        outlook: user.get("is_outlook"),
                    },
                    r,
                )));
            }
        }
        Ok(None)
    }

    /// Gets a user's id.
    pub async fn user_id(&self, email: &EmailAddress, role: Role) -> Result<Option<Ulid>, Error> {
        let roles_to_check = match role {
            Role::ClientIndividual | Role::ClientEntity => {
                vec![Role::ClientIndividual, Role::ClientEntity]
            }
            Role::ContractorIndividual | Role::ContractorEntity => {
                vec![Role::ContractorIndividual, Role::ContractorEntity]
            }
            Role::EorAdmin => vec![Role::EorAdmin],
        };

        for r in roles_to_check {
            let id = sqlx::query(&format!(
                "SELECT ulid FROM {} WHERE email = $1",
                Self::user_table_name(r)
            ))
            .bind(email.as_ref())
            .fetch_optional(&self.0)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

            if let Some(id) = id {
                if r == role {
                    return Ok(Some(ulid_from_sql_uuid(id.get("ulid"))));
                } else {
                    return Err(Error::BadRequest);
                }
            }
        }
        Ok(None)
    }

    pub async fn onboard_individual_details(
        &self,
        ulid: Ulid,
        role: Option<Role>,
        details: IndividualDetails,
    ) -> Result<(), Error> {
        eprintln!("{details:?}");
        Ok(())
    }

    fn user_table_name(role: Role) -> &'static str {
        match role {
            Role::ClientIndividual => "client_individuals",
            Role::ClientEntity => "client_entities",
            Role::ContractorIndividual => "contractor_individuals",
            Role::ContractorEntity => "contractor_entities",
            Role::EorAdmin => "eor_admins",
        }
    }
}

fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
