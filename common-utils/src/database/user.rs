use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    calc_limit_and_offset,
    custom_serde::{EmailWrapper, OffsetDateWrapper, UserRole, UserType},
    error::{GlobeliseError, GlobeliseResult},
};

use super::Database;

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardedUserIndex {
    // NOTE: Since we also join with prefilled_individual_contractor table
    pub ulid: Option<Uuid>,
    pub name: String,
    pub user_role: UserRole,
    pub user_type: UserType,
    pub email: EmailWrapper,
    pub contract_count: i64,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub ulid: Uuid,
    pub email: EmailWrapper,
    pub password: Option<String>,
    pub is_google: bool,
    pub is_outlook: bool,
    pub is_entity: bool,
    pub is_individual: bool,
    pub is_client: bool,
    pub is_contractor: bool,
}

impl User {
    pub fn user_type(&self) -> GlobeliseResult<UserType> {
        if self.is_individual {
            Ok(UserType::Individual)
        } else if self.is_entity {
            Ok(UserType::Entity)
        } else {
            Err(GlobeliseError::internal("User is not configured properly"))
        }
    }
}

impl Database {
    #[allow(clippy::too_many_arguments)]
    pub async fn select_many_onboarded_user_index(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        user_type: Option<UserType>,
        user_role: Option<UserRole>,
        created_after: Option<sqlx::types::time::OffsetDateTime>,
        created_before: Option<sqlx::types::time::OffsetDateTime>,
    ) -> GlobeliseResult<Vec<OnboardedUserIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM 
                onboarded_user_index 
            WHERE
                ($1 IS NULL OR name ~* $1 OR email ~* $1) AND
                ($2 IS NULL OR user_role = $2) AND
                ($3 IS NULL OR user_type = $3) AND
                ($4 IS NULL OR created_at > $4) AND
                ($5 IS NULL OR created_at < $5)
            LIMIT
                $6
            OFFSET
                $7",
        )
        .bind(query)
        .bind(user_role)
        .bind(user_type)
        .bind(created_after)
        .bind(created_before)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;
        Ok(result)
    }
}

#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserIndex {
    pub ulid: Uuid,
    pub user_type: UserType,
    pub email: EmailWrapper,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
}

impl Database {
    #[allow(clippy::too_many_arguments)]
    pub async fn create_user(
        &self,
        email: EmailWrapper,
        password: Option<String>,
        is_google: bool,
        is_outlook: bool,
        is_entity: bool,
        is_individual: bool,
        is_client: bool,
        is_contractor: bool,
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO users (
                ulid, email, password, is_google, is_outlook,
                is_entity, is_individual, is_client, is_contractor
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9
            )",
        )
        .bind(ulid)
        .bind(email)
        .bind(password)
        .bind(is_google)
        .bind(is_outlook)
        .bind(is_entity)
        .bind(is_individual)
        .bind(is_client)
        .bind(is_contractor)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }

    pub async fn update_user_password_hash(
        &self,
        ulid: Uuid,
        user_type: UserType,
        // TODO: Create a newtype to ensure only hashed password are inserted
        new_password_hash: Option<String>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            UPDATE 
                users 
            SET 
                password = $1 
            WHERE 
                ulid = $2 AND
                is_entity = $2 AND
                is_individual = $3",
        )
        .bind(new_password_hash)
        .bind(ulid)
        .bind(user_type == UserType::Entity)
        .bind(user_type == UserType::Individual)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn find_one_user(
        &self,
        ulid: Option<Uuid>,
        email: Option<&EmailWrapper>,
        user_type: Option<UserType>,
    ) -> GlobeliseResult<Option<User>> {
        let maybe_user = sqlx::query_as(
            "
            SELECT 
                ulid, email, password, is_google, is_outlook, 
                is_entity, is_individual, is_client, is_contractor
            FROM 
                users
            WHERE 
                ($1 IS NULL OR ulid = $1) AND
                ($2 IS NULL OR email = $2) AND
                ($3 IS NULL OR is_entity = $3) AND
                ($4 IS NULL OR is_individual = $4)",
        )
        .bind(ulid)
        .bind(email)
        .bind(user_type.map(|t| t == UserType::Entity))
        .bind(user_type.map(|t| t == UserType::Individual))
        .fetch_optional(&self.0)
        .await?;

        Ok(maybe_user)
    }

    pub async fn select_many_user_index(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        user_type: Option<UserType>,
        created_after: Option<sqlx::types::time::OffsetDateTime>,
        created_before: Option<sqlx::types::time::OffsetDateTime>,
    ) -> GlobeliseResult<Vec<UserIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT 
                *
            FROM 
                users_index 
            WHERE
                ($1 IS NULL OR email ~* $1) AND
                ($2 IS NULL OR user_type = $2) AND
                ($3 IS NULL OR created_at > $3) AND
                ($4 IS NULL OR created_at < $4)
            LIMIT
                $5
            OFFSET
                $6",
        )
        .bind(query)
        .bind(user_type)
        .bind(created_after)
        .bind(created_before)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn select_many_clients_index_for_contractors(
        &self,
        contractor_ulid: Option<Uuid>,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        user_type: Option<UserType>,
        user_role: Option<UserRole>,
        created_after: Option<sqlx::types::time::OffsetDateTime>,
        created_before: Option<sqlx::types::time::OffsetDateTime>,
    ) -> GlobeliseResult<Vec<OnboardedUserIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT 
                *
            FROM 
                clients_index_for_contractors 
            WHERE
                ($1 IS NULL OR contractor_ulid = $1) AND
                ($2 IS NULL OR email ~* $2 OR name ~* $2) AND
                ($3 IS NULL OR user_type = $3) AND
                ($4 IS NULL OR user_role = $4) AND
                ($5 IS NULL OR created_at > $5) AND
                ($6 IS NULL OR created_at < $6)
            LIMIT
                $7
            OFFSET
                $8",
        )
        .bind(contractor_ulid)
        .bind(query)
        .bind(user_type)
        .bind(user_role)
        .bind(created_after)
        .bind(created_before)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn select_many_contractors_index_for_clients(
        &self,
        client_ulid: Option<Uuid>,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        user_type: Option<UserType>,
        user_role: Option<UserRole>,
        created_after: Option<sqlx::types::time::OffsetDateTime>,
        created_before: Option<sqlx::types::time::OffsetDateTime>,
    ) -> GlobeliseResult<Vec<OnboardedUserIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT 
                *
            FROM 
                contractors_index_for_clients 
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR email ~* $2 OR name ~* $2) AND
                ($3 IS NULL OR user_type = $3) AND
                ($4 IS NULL OR user_role = $4) AND
                ($5 IS NULL OR created_at > $5) AND
                ($6 IS NULL OR created_at < $6)
            LIMIT
                $7
            OFFSET
                $8",
        )
        .bind(client_ulid)
        .bind(query)
        .bind(user_type)
        .bind(user_role)
        .bind(created_after)
        .bind(created_before)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }
}
