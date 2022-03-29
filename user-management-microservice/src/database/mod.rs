use std::{sync::Arc, time::Duration};

use common_utils::error::GlobeliseResult;

use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

use crate::eor_admin::{UserIndex, UserIndexQuery};

mod auth;
mod onboard;
mod prefill_onboard;

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
    pub async fn user_index(&self, query: UserIndexQuery) -> GlobeliseResult<Vec<UserIndex>> {
        let result = sqlx::query_as(
            "
            SELECT 
                ulid,
                name,
                email,
                user_role,
                user_type
            FROM 
                user_index 
            WHERE
                ($1 IS NULL OR name ~* $1) AND
                ($2 IS NULL OR user_role = $2) AND
                ($3 IS NULL OR user_type = $3)
            LIMIT
                $4
            OFFSET
                $5",
        )
        .bind(query.search_text)
        .bind(query.user_role.map(|s| s.to_string()))
        .bind(query.user_type.map(|s| s.to_string()))
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;
        Ok(result)
    }
}

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
