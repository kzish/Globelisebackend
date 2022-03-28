use std::{sync::Arc, time::Duration};

use common_utils::error::{GlobeliseError, GlobeliseResult};
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

use crate::{
    auth::user::{Role, UserType},
    eor_admin::UserIndex,
    onboard::individual::IndividualDetails,
};

mod auth;
mod onboard;

/// Convenience wrapper around PostgreSQL.
pub struct Database(Pool<Postgres>);

pub type SharedDatabase = Arc<Mutex<Database>>;

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
    ) -> GlobeliseResult<Vec<UserIndex>> {
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
            .await?
            .into_iter()
            .map(UserIndex::from_pg_row)
            .collect::<GlobeliseResult<Vec<UserIndex>>>()?;
        Ok(result)
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
    ) -> GlobeliseResult<()> {
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
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}
