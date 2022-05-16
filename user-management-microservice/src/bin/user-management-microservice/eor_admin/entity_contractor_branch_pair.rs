use axum::{extract::Query, Extension, Json};
use common_utils::{
    calc_limit_and_offset, error::GlobeliseResult, token::Token, ulid_from_sql_uuid,
    ulid_to_sql_uuid,
};

use eor_admin_microservice_sdk::token::AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::database::{Database, SharedDatabase};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorBranchPair {
    pub contractor_ulid: Ulid,
    pub branch_ulid: Ulid,
}

impl FromRow<'_, PgRow> for EntityContractorBranchPair {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(EntityContractorBranchPair {
            contractor_ulid: ulid_from_sql_uuid(row.try_get("contractor_ulid")?),
            branch_ulid: ulid_from_sql_uuid(row.try_get("branch_ulid")?),
        })
    }
}

pub async fn post_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityContractorBranchPair>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .post_entity_contractor_branch_pairs(request.contractor_ulid, request.branch_ulid)
        .await?;

    Ok(())
}

pub async fn delete_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityContractorBranchPair>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .delete_entity_contractor_branch_pairs(request.contractor_ulid, request.branch_ulid)
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPairQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Option<Ulid>,
    pub contractor_ulid: Option<Ulid>,
}

pub async fn get_many(
    _: Token<AdminAccessToken>,
    Query(query): Query<ClientContractorPairQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<EntityContractorBranchPair>>> {
    let database = database.lock().await;
    Ok(Json(
        database.get_entity_contractor_branch_pairs(query).await?,
    ))
}

impl Database {
    /// Create a entity contractor and branch pair
    pub async fn post_entity_contractor_branch_pairs(
        &self,
        contractor_ulid: Ulid,
        branch_ulid: Ulid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO entity_contractor_branch_pairs 
                (contractor_ulid, branch_ulid)
            VALUES
                ($1, $2)",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(ulid_to_sql_uuid(branch_ulid))
        .execute(&self.0)
        .await?;
        Ok(())
    }

    /// Create a entity contractor and branch pair
    pub async fn delete_entity_contractor_branch_pairs(
        &self,
        contractor_ulid: Ulid,
        branch_ulid: Ulid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                entity_contractor_branch_pairs 
            WHERE
                contractor_ulid = $1 AND
                branch_ulid =$2",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(ulid_to_sql_uuid(branch_ulid))
        .execute(&self.0)
        .await?;
        Ok(())
    }

    /// Index entity contractor and branch pairs
    pub async fn get_entity_contractor_branch_pairs(
        &self,
        query: ClientContractorPairQuery,
    ) -> GlobeliseResult<Vec<EntityContractorBranchPair>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let result = sqlx::query_as(
            "
            SELECT 
                client_ulid, contractor_ulid
            FROM
                entity_contractor_branch_pairs
            WHERE
                ($1 IS NULL OR (client = $1)) AND
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
