use std::{sync::Arc, time::Duration};

use common_utils::error::{GlobeliseError, GlobeliseResult};
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;
use user_management_microservice_sdk::Role;

use crate::contracts::{ContractorIndex, ContractorIndexQuery};

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
    /// Counts the number of contracts.
    pub async fn count_number_of_contracts(
        &self,
        ulid: &Ulid,
        role: &Role,
    ) -> GlobeliseResult<i64> {
        let result = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM contractors WHERE {} = $1",
            match role {
                Role::Client => "client_ulid",
                Role::Contractor => "contractor_ulid",
            }
        ))
        .bind(ulid_to_sql_uuid(*ulid))
        .fetch_one(&self.0)
        .await
        .map_err(|e| GlobeliseError::Internal(e.to_string()))?;
        Ok(result)
    }

    /// Indexes contractors working for a client.
    pub async fn contractor_index(
        &self,
        client_ulid: Ulid,
        query: ContractorIndexQuery,
    ) -> GlobeliseResult<Vec<ContractorIndex>> {
        let index = sqlx::query_as(&format!(
            "SELECT * FROM contractor_index WHERE client_ulid = $1 {} LIMIT $2 OFFSET $3",
            match query.search_text {
                Some(search_text) => format!("AND name ~* '{}'", search_text),
                None => "".into(),
            }
        ))
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }
}

fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}
