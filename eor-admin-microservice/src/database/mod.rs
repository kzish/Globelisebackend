use std::{sync::Arc, time::Duration};

use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

mod auth;
mod onboard;

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

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
