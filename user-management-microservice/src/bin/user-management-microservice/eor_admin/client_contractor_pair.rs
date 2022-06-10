use axum::{extract::Query, Extension, Json};
use common_utils::{calc_limit_and_offset, error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPair {
    pub client_ulid: Uuid,
    pub contractor_ulid: Uuid,
}

pub async fn post_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(request): Json<ClientContractorPair>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .create_client_contractor_pairs(request.client_ulid, request.contractor_ulid)
        .await?;

    Ok(())
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPairQueryRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,

    pub client_ulid: Option<Uuid>,

    pub contractor_ulid: Option<Uuid>,
}

pub async fn get_many(
    _: Token<AdminAccessToken>,
    Query(query): Query<ClientContractorPairQueryRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ClientContractorPair>>> {
    let database = database.lock().await;
    Ok(Json(database.client_contractor_pair_index(query).await?))
}

impl Database {
    /// Create a client/contractor pair
    pub async fn create_client_contractor_pairs(
        &self,
        client_ulid: Uuid,
        contractor_ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO client_contractor_pairs 
                (client_ulid, contractor_ulid)
            VALUES
                ($1, $2)",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    /// Index client/contractor pairs
    pub async fn client_contractor_pair_index(
        &self,
        query: ClientContractorPairQueryRequest,
    ) -> GlobeliseResult<Vec<ClientContractorPair>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let result = sqlx::query_as(
            "
            SELECT 
                client_ulid, contractor_ulid
            FROM
                client_contractor_pairs
            WHERE
                ($1 IS NULL OR (client_ulid = $1)) AND
                ($2 IS NULL OR (contractor_ulid = $2))
            LIMIT
                $3
            OFFSET
                $4",
        )
        .bind(query.client_ulid)
        .bind(query.contractor_ulid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }
}
