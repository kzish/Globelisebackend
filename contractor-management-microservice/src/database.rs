use std::{sync::Arc, time::Duration};

use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

use crate::{error::Error, microservices::user_management::Role};

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
            r##"
            WITH result1 AS (
                SELECT
                    *
                FROM
                    {}_and_{}
            ),
            result2 AS (
                SELECT
                    *
                FROM
                    {}_and_{}
            ),
            results_union AS (
                SELECT
                    *
                FROM
                    result1
                UNION
                SELECT
                    *
                FROM
                    result2
            )
            SELECT
                COUNT(client_ulid) as count
            FROM
                results_union
            WHERE {} = $1;"##,
            role.as_db_name(),
            match role {
                Role::ClientIndividual | Role::ClientEntity =>
                    Role::ContractorIndividual.as_db_name(),
                Role::ContractorIndividual | Role::ContractorEntity =>
                    Role::ClientIndividual.as_db_name(),
                Role::EorAdmin =>
                    return Err(Error::BadRequest("EOR admins cannot have any contracts")),
            },
            role.as_db_name(),
            match role {
                Role::ClientIndividual | Role::ClientEntity => Role::ContractorEntity.as_db_name(),
                Role::ContractorIndividual | Role::ContractorEntity =>
                    Role::ClientEntity.as_db_name(),
                Role::EorAdmin =>
                    return Err(Error::BadRequest("EOR admins cannot have any contracts")),
            },
            match role {
                Role::ClientIndividual | Role::ClientEntity => "client_ulid",
                Role::ContractorIndividual | Role::ContractorEntity => "contractor_ulid",
                Role::EorAdmin =>
                    return Err(Error::BadRequest("EOR admins cannot have any contracts")),
            }
        );
        let result: i64 = sqlx::query_scalar(&query)
            .bind(ulid_to_sql_uuid(*ulid))
            .fetch_one(&self.0)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(result)
    }
}

fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}
