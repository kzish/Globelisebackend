use axum::{extract::Query, Extension, Json};
use common_utils::{
    calc_limit_and_offset,
    error::GlobeliseResult,
    pubsub::{AddClientContractorPair, SharedPubSub},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use eor_admin_microservice_sdk::token::AccessToken as AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::database::{Database, SharedDatabase};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPair {
    pub client_ulid: Ulid,
    pub contractor_ulid: Ulid,
}

impl FromRow<'_, PgRow> for ClientContractorPair {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(ClientContractorPair {
            client_ulid: ulid_from_sql_uuid(row.try_get("client_ulid")?),
            contractor_ulid: ulid_from_sql_uuid(row.try_get("contractor_ulid")?),
        })
    }
}

pub async fn eor_admin_create_client_contractor_pairs(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(request): Json<ClientContractorPair>,
    Extension(pubsub): Extension<SharedPubSub>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .create_client_contractor_pairs(request.client_ulid, request.contractor_ulid)
        .await?;

    // Publish event to DAPR
    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(AddClientContractorPair {
            client_ulid: request.client_ulid,
            contractor_ulid: request.contractor_ulid,
        })
        .await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPairQueryRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Option<Ulid>,
    pub contractor_ulid: Option<Ulid>,
}

pub async fn eor_admin_client_contractor_index(
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
        client_ulid: Ulid,
        contractor_ulid: Ulid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO client_contractor_pairs 
                (client_ulid, contractor_ulid)
            VALUES
                ($1, $2)",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(ulid_to_sql_uuid(contractor_ulid))
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
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;
        Ok(result)
    }
}
