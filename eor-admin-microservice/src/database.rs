use std::{sync::Arc, time::Duration};

use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use tokio::sync::Mutex;

use crate::auth::admin::Admin;

use crate::error::Error;
use crate::onboard::individual::IndividualDetails;

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

    /// Creates and stores a new admin.
    pub async fn create_admin(&self, admin: Admin) -> Result<Ulid, Error> {
        if !admin.has_authentication() {
            return Err(Error::Unauthorized(
                "Refused to create admin: no authentication method provided",
            ));
        }

        // Avoid overwriting an existing admin.
        match self.admin_id(&admin.email).await {
            Ok(Some(_)) => return Err(Error::UnavailableEmail),
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
    ) -> Result<(), Error> {
        sqlx::query("UPDATE auth_eor_admins SET password = $1 WHERE ulid = $2")
            .bind(new_password_hash)
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await?;

        Ok(())
    }

    /// Gets a admin's authentication information.
    pub async fn admin(&self, ulid: Ulid) -> Result<Option<Admin>, Error> {
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
    pub async fn admin_id(&self, email: &EmailAddress) -> Result<Option<Ulid>, Error> {
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

impl Database {
    pub async fn onboard_admin_details(
        &self,
        ulid: Ulid,
        details: IndividualDetails,
    ) -> Result<(), Error> {
        let query = "
            INSERT INTO onboard_eor_admins 
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($13, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            country = $6, city = $7, address = $8, postal_code = $9, tax_id = $10,
            time_zone = $11, profile_picture = $12"
            .to_string();
        sqlx::query(&query)
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
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(())
    }
}

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
