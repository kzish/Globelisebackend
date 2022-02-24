use std::{str::FromStr, sync::Arc, time::Duration};

use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use strum::IntoEnumIterator;
use tokio::sync::Mutex;

use crate::{
    auth::user::{Role, User},
    info::UserIndex,
    onboard::{
        bank::BankDetails,
        entity::{EntityDetails, PicDetails},
        individual::IndividualDetails,
    },
};

use crate::error::Error;

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
            return Err(Error::Unauthorized(
                "Refused to create user: no authentication method provided",
            ));
        }

        // Avoid overwriting an existing user.
        match self.user_id(&user.email, role).await {
            Ok(Some(_)) | Err(Error::WrongUserType) => return Err(Error::UnavailableEmail),
            Ok(None) => (),
            Err(e) => return Err(e),
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
                        email: user.get::<String, _>("email").parse().map_err(|_| {
                            Error::Internal("Invalid email address from database".into())
                        })?,
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

    /// Gets a user's information.
    ///
    /// If `role` is specified, this function only searches that role's table.
    /// Otherwise, it searches all user tables.
    pub async fn user_index(&self) -> Result<Vec<UserIndex>, Error> {
        let result = sqlx::query!(
            r##"
            WITH client_entities_result AS (
                SELECT
                    ulid,
                    email,
                    company_name AS name,
                    dob
                FROM
                    client_entities
            ),
            client_individuals_result AS (
                SELECT
                    ulid,
                    email,
                    CONCAT(first_name, ' ', last_name) AS name,
                    dob
                FROM
                    client_individuals
            ),
            contractor_entities_result AS (
                SELECT
                    ulid,
                    email,
                    company_name AS name,
                    dob
                FROM
                    contractor_entities
            ),
            contractor_individuals_result AS (
                SELECT
                    ulid,
                    email,
                    CONCAT(first_name, ' ', last_name) AS name,
                    dob
                FROM
                    contractor_individuals
            ),
            result AS (
                SELECT
                    *,
                    'client-entity' AS role
                FROM
                    client_entities_result
                UNION
                SELECT
                    *,
                    'client-individual' AS role
                FROM
                    client_individuals_result
                UNION
                SELECT
                    *,
                    'contractor-entity' AS role
                FROM
                    contractor_entities_result
                UNION
                SELECT
                    *,
                    'contractor-individual' AS role
                FROM
                    contractor_individuals_result
            )
            SELECT
                ulid,
                name,
                email,
                role,
                dob
            FROM
                result"##,
        )
        .fetch_all(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?
        .into_iter()
        .map(|r| -> Result<UserIndex, Error> {
            // NOTE: The unwraps below should be safe because all of these values
            // are required in the database
            let role = Role::from_str(r.role.unwrap().as_ref())?;
            Ok(UserIndex {
                ulid: ulid_from_sql_uuid(r.ulid.unwrap()),
                name: r.name.unwrap(),
                role,
                email: r.email.unwrap(),
                dob: r.dob.map(|v| v.to_string()),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
        Ok(result)
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
                    return Err(Error::WrongUserType);
                }
            }
        }
        Ok(None)
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

impl Database {
    pub async fn onboard_individual_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: IndividualDetails,
    ) -> Result<(), Error> {
        if !matches!(
            role,
            Role::ClientIndividual | Role::ContractorIndividual | Role::EorAdmin
        ) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(role)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        sqlx::query(&format!(
            "UPDATE {}
            SET first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            country = $6, city = $7, address = $8, postal_code = $9, tax_id = $10,
            time_zone = $11, profile_picture = $12
            WHERE ulid = $13",
            Self::user_table_name(role)
        ))
        .bind(details.first_name)
        .bind(details.last_name)
        .bind(details.dob)
        .bind(details.dial_code)
        .bind(details.phone_number)
        .bind(details.country)
        .bind(details.city)
        .bind(details.address)
        .bind(details.postal_code)
        .bind(details.tax_id)
        .bind(details.time_zone)
        .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_entity_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: EntityDetails,
    ) -> Result<(), Error> {
        if !matches!(role, Role::ClientEntity | Role::ContractorEntity) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(role)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        sqlx::query(&format!(
            "UPDATE {}
            SET company_name = $1, country = $2, entity_type = $3, registration_number = $4,
            tax_id = $5, company_address = $6, city = $7, postal_code = $8, time_zone = $9,
            logo = $10
            WHERE ulid = $11",
            Self::user_table_name(role)
        ))
        .bind(details.company_name)
        .bind(details.country)
        .bind(details.entity_type)
        .bind(details.registration_number)
        .bind(details.tax_id)
        .bind(details.company_address)
        .bind(details.city)
        .bind(details.postal_code)
        .bind(details.time_zone)
        .bind(details.logo.map(|b| b.as_ref().to_owned()))
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_pic_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: PicDetails,
    ) -> Result<(), Error> {
        if !matches!(role, Role::ClientEntity | Role::ContractorEntity) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(role)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        sqlx::query(&format!(
            "UPDATE {}
            SET first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            profile_picture = $6
            WHERE ulid = $7",
            Self::user_table_name(role)
        ))
        .bind(details.first_name)
        .bind(details.last_name)
        .bind(details.dob)
        .bind(details.dial_code)
        .bind(details.phone_number)
        .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_bank_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: BankDetails,
    ) -> Result<(), Error> {
        if !matches!(role, Role::ContractorIndividual | Role::ContractorEntity) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(role)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        sqlx::query(&format!(
            "UPDATE {}
            SET bank_name = $1, bank_account_name = $2, bank_account_number = $3
            WHERE ulid = $4",
            Self::user_table_name(role)
        ))
        .bind(details.bank_name)
        .bind(details.account_name)
        .bind(details.account_number)
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }
}

fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
