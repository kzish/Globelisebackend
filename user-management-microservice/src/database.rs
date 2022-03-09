use std::{sync::Arc, time::Duration};

use email_address::EmailAddress;
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use strum::IntoEnumIterator;
use tokio::sync::Mutex;

use crate::{
    auth::user::{Role, User, UserType},
    eor_admin::UserIndex,
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
    pub async fn create_user(&self, user: User, user_type: UserType) -> Result<Ulid, Error> {
        if !user.has_authentication() {
            return Err(Error::Unauthorized(
                "Refused to create user: no authentication method provided",
            ));
        }

        // Avoid overwriting an existing user.
        match self.user_id(&user.email, user_type).await {
            Ok(Some(_)) | Err(Error::WrongUserType) => return Err(Error::UnavailableEmail),
            Ok(None) => (),
            Err(e) => return Err(e),
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
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(ulid)
    }

    /// Updates a user's password.
    pub async fn update_password(
        &self,
        ulid: Ulid,
        user_type: UserType,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> Result<(), Error> {
        sqlx::query(&format!(
            "UPDATE {} SET password = $1 WHERE ulid = $2",
            user_type.db_auth_name()
        ))
        .bind(new_password_hash)
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

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
    ) -> Result<Option<(User, UserType)>, Error> {
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
                    t,
                )));
            }
        }
        Ok(None)
    }

    /// Index users (client and contractors)
    ///
    /// Currently, the search functionality only works on the name.
    /// For entities, this is the company's name.
    /// For individuals, this is a concat of their first and last name.
    pub async fn user_index(
        &self,
        m_page: Option<i64>,
        m_per_page: Option<i64>,
        m_search_text: Option<String>,
        m_user_type: Option<UserType>,
        m_user_role: Option<Role>,
    ) -> Result<Vec<UserIndex>, Error> {
        let page = m_page.unwrap_or(0);
        let per_page = m_per_page.unwrap_or(25);
        let query = create_eor_admin_user_index_query(
            page,
            per_page,
            m_search_text,
            m_user_type,
            m_user_role,
        );
        let result = sqlx::query(&query)
            .fetch_all(&self.0)
            .await
            .map_err(|e| Error::Database(e.to_string()))?
            .into_iter()
            .map(UserIndex::from_pg_row)
            .collect::<Result<Vec<UserIndex>, _>>()?;
        Ok(result)
    }

    /// Gets a user's id.
    pub async fn user_id(
        &self,
        email: &EmailAddress,
        user_type: UserType,
    ) -> Result<Option<Ulid>, Error> {
        let mut ulid = None;

        for t in UserType::iter() {
            let id = sqlx::query(&format!(
                "SELECT ulid FROM {} WHERE email = $1",
                t.db_auth_name()
            ))
            .bind(email.as_ref())
            .fetch_optional(&self.0)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

            if let Some(id) = id {
                if t == user_type {
                    ulid = Some(ulid_from_sql_uuid(id.get("ulid")));
                } else {
                    return Err(Error::WrongUserType);
                }
            }
        }
        Ok(ulid)
    }
}

impl Database {
    pub async fn onboard_individual_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        role: Role,
        details: IndividualDetails,
    ) -> Result<(), Error> {
        if !matches!(user_type, UserType::Individual | UserType::EorAdmin) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        let target_table = user_type.db_onboard_name(role);
        let query = format!(
            "
            INSERT INTO {target_table} 
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($13, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            country = $6, city = $7, address = $8, postal_code = $9, tax_id = $10,
            time_zone = $11, profile_picture = $12",
        );
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
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn onboard_entity_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        role: Role,
        details: EntityDetails,
    ) -> Result<(), Error> {
        if !matches!(user_type, UserType::Entity) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        sqlx::query(&format!(
            "
            INSERT INTO {}
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo)
            VALUES ($11, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $1, country = $2, entity_type = $3, registration_number = $4,
            tax_id = $5, company_address = $6, city = $7, postal_code = $8, time_zone = $9,
            logo = $10",
            user_type.db_onboard_name(role)
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
        user_type: UserType,
        role: Role,
        details: PicDetails,
    ) -> Result<(), Error> {
        if !matches!(user_type, UserType::Entity) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        let target_table = user_type.db_onboard_name(role);
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, first_name, last_name, dob, dial_code, phone_number, profile_picture)
            VALUES ($7, $1, $2, $3, $4, $5, $6)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            profile_picture = $6",
        );

        sqlx::query(&query)
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
        user_type: UserType,
        details: BankDetails,
    ) -> Result<(), Error> {
        if !matches!(user_type, UserType::Individual | UserType::Entity) {
            return Err(Error::Forbidden);
        }
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(Error::Forbidden);
        }

        let target_table = user_type.db_onboard_name(Role::Contractor);
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, bank_name, bank_account_name, bank_account_number)
            VALUES ($4, $1, $2, $3)
            ON CONFLICT(ulid) DO UPDATE SET 
            bank_name = $1, bank_account_name = $2, bank_account_number = $3",
        );
        sqlx::query(&query)
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

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}

fn create_eor_admin_user_index_query(
    page: i64,
    per_page: i64,
    m_search_text: Option<String>,
    m_user_type: Option<UserType>,
    m_user_role: Option<Role>,
) -> String {
    let mut where_clauses_iter = vec![];
    if let Some(search_text) = m_search_text {
        where_clauses_iter.push(format!("\tname ~* '{}'", search_text));
    };
    if let Some(user_role) = m_user_role {
        where_clauses_iter.push(format!("\tuser_role = '{}'", user_role));
    };
    if let Some(user_type) = m_user_type {
        where_clauses_iter.push(format!("\tuser_type = '{}'", user_type));
    };
    let where_clauses = where_clauses_iter
        .into_iter()
        .collect::<Vec<String>>()
        .join(" AND\n");
    let limit = per_page;
    let offset = page * per_page;
    let query = format!(
        "
WITH client_individual_info AS (
    SELECT
        auth_individuals.ulid,
        auth_individuals.email,
        CONCAT(
            onboard_individual_clients.first_name,
            ' ',
            onboard_individual_clients.last_name
        ) AS name,
        'client' AS user_role,
        'individual' AS user_type
    FROM
        onboard_individual_clients
        LEFT OUTER JOIN auth_individuals ON auth_individuals.ulid = onboard_individual_clients.ulid
),
client_entity_info AS (
    SELECT
        auth_entities.ulid,
        auth_entities.email,
        onboard_entity_clients.company_name AS name,
        'client' AS user_role,
        'entity' AS user_type
    FROM
        onboard_entity_clients
        LEFT OUTER JOIN auth_entities ON auth_entities.ulid = onboard_entity_clients.ulid
),
contractor_individual_info AS (
    SELECT
        auth_individuals.ulid,
        auth_individuals.email,
        CONCAT(
            onboard_individual_contractors.first_name,
            ' ',
            onboard_individual_contractors.last_name
        ) AS name,
        'contractor' AS user_role,
        'individual' AS user_type
    FROM
        onboard_individual_contractors
        LEFT OUTER JOIN auth_individuals ON auth_individuals.ulid = onboard_individual_contractors.ulid
),
contractor_entity_info AS (
    SELECT
        auth_entities.ulid,
        auth_entities.email,
        onboard_entity_contractors.company_name AS name,
        'contractor' AS user_role,
        'entity' AS user_type
    FROM
        onboard_entity_contractors
        LEFT OUTER JOIN auth_entities ON auth_entities.ulid = onboard_entity_contractors.ulid
),
result AS (
    SELECT
        *
    FROM
        client_individual_info
    UNION
    SELECT
        *
    FROM
        client_entity_info
    UNION
    SELECT
        *
    FROM
        contractor_individual_info
    UNION
    SELECT
        *
    FROM
        contractor_entity_info
)
SELECT
    ulid,
    name,
    email,
    user_role,
    user_type
FROM
    result
{}
{where_clauses}
LIMIT
    {limit}
OFFSET
    {offset}
",
        if where_clauses.is_empty() {
            ""
        } else {
            "WHERE"
        },
    );
    query
}

impl Database {
    pub async fn prefill_onboard_individual_contractors(
        &self,
        email: String,
        details: IndividualDetails,
    ) -> Result<(), Error> {
        let query = "
            INSERT INTO  prefilled_onboard_individual_contractors
            (email, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($13, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(email) DO UPDATE SET 
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
            .bind(email)
            .execute(&self.0)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }
}
