use std::{sync::Arc, time::Duration};

use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;
use user_management_microservice_sdk::user_index::{OnboardedUserIndex, UserIndex};

use crate::eor_admin::{OnboardedUserIndexQuery, UserIndexQuery};

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

    /// Index users (client and contractors)
    ///
    /// Currently, the search functionality only works on the name.
    /// For entities, this is the company's name.
    /// For individuals, this is a concat of their first and last name.
    pub async fn onboarded_user_index(
        &self,
        query: OnboardedUserIndexQuery,
    ) -> GlobeliseResult<Vec<OnboardedUserIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let result = sqlx::query_as(
            "
            SELECT 
                created_at,
                ulid,
                name,
                email,
                user_role,
                user_type
            FROM 
                onboarded_user_index 
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
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;
        Ok(result)
    }

    /// Index users ULID and email
    ///
    /// This does not require users to be fully onboarded.
    pub async fn user_index(&self, query: UserIndexQuery) -> GlobeliseResult<Vec<UserIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let result = sqlx::query_as(
            "
            SELECT 
                created_at,
                ulid,
                email
            FROM 
                user_index 
            WHERE
                ($1 IS NULL OR email = $1)
            LIMIT
                $2
            OFFSET
                $3",
        )
        .bind(query.search_text)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;
        Ok(result)
    }
}
