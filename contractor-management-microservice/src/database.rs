use std::{sync::Arc, time::Duration};

use common_utils::{error::GlobeliseResult, ulid_to_sql_uuid};
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

pub type SharedDatabase = Arc<Mutex<Database>>;

/// Convenience wrapper around PostgreSQL.
pub struct Database(pub Pool<Postgres>);

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

    /// Create a client/contractor pair
    ///
    /// This does not require users to be fully onboarded.
    pub async fn update_client_contractor_pair(
        &self,
        client_ulid: Ulid,
        contractor_ulid: Ulid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO client_contractor_pairs 
                (client_ulid, contractor_ulid)
            VALUES
                ($1, $2)
            ON CONFLICT 
                (client_ulid, contractor_ulid)
            DO NOTHING",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .execute(&self.0)
        .await?;
        Ok(())
    }

    /// Update a client's name
    pub async fn update_client_name(&self, ulid: Ulid, name: String) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO client_names
                (ulid, name)
            VALUES
                ($1, $2)
            ON CONFLICT (ulid)
            DO UPDATE SET
                name = $2",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .bind(name)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    /// Update a contractor's name
    pub async fn update_contractor_name(&self, ulid: Ulid, name: String) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO contractor_names 
                (ulid, name)
            VALUES
                ($1, $2)
            ON CONFLICT (ulid)
            DO UPDATE SET
                name = $2",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .bind(name)
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
