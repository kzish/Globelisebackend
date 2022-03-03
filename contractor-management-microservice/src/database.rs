use std::{sync::Arc, time::Duration};

use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;
use user_management_microservice_sdk::Role;

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
}

impl Database {
    /// Count the number of contracts
    pub async fn count_number_of_contracts(&self, ulid: &Ulid, role: &Role) -> Result<i64, Error> {
        let query = format!(
            "
        WITH result1 AS (
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                client_entities_and_contractor_entities
        ),
        result2 AS (
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                client_entities_and_contractor_individuals
        ),
        result3 AS (
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                client_individuals_and_contractor_entities
        ),
        result4 AS (
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                client_individuals_and_contractor_individuals
        ),
        results_union AS (
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                result1
            UNION
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                result2
            UNION
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                result3
            UNION
            SELECT
                client_ulid,
                contractor_ulid,
                created_at
            FROM
                result4
            WHERE
                {} = $1
        )
        SELECT
            COUNT(client_ulid) as count
        FROM
            results_union;",
            match role {
                Role::Client => "client_ulid",
                Role::Contractor => "contractor_ulid",
            }
        );
        let result: i64 = sqlx::query_scalar(&query)
            .bind(ulid_to_sql_uuid(*ulid))
            .fetch_one(&self.0)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(result)
    }
}

fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}
