use std::{sync::Arc, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

pub mod user;

/// Convenience wrapper around PostgreSQL.
pub struct Database(pub Pool<Postgres>);

pub type CommonDatabase = Arc<Mutex<Database>>;

impl Database {
    /// Connects to PostgreSQL.
    pub async fn new(connection_str: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_timeout(Duration::from_secs(3))
            .connect(connection_str)
            .await
            .expect("Cannot connect to database");

        Self(pool)
    }
}
