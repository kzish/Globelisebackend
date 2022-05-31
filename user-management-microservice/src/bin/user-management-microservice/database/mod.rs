use std::{sync::Arc, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

mod auth;

/// Convenience wrapper around PostgreSQL.
pub struct Database(pub Pool<Postgres>);

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
}
