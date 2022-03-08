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
        let result = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM contracts WHERE {} = $1",
            match role {
                Role::Client => "client_ulid",
                Role::Contractor => "contractor_ulid",
            }
        ))
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
