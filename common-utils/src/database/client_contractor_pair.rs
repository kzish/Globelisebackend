use crate::{calc_limit_and_offset, error::GlobeliseResult};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::Database;

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientContractorPairIndex {
    client_ulid: Uuid,
    contractor_ulid: Uuid,
}

impl Database {
    pub async fn select_many_client_contractor_pair_index(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        client_ulid: Option<Uuid>,
        contractor_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Vec<ClientContractorPairIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                client_contractor_pairs
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2)
            LIMIT
                $3
            OFFSET
                $4",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_client_contractor_pair(
        &self,
        client_ulid: Uuid,
        contractor_ulid: Uuid,
        _contract_ulid: Option<Uuid>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO client_contractor_pairs (client_ulid, contractor_ulid) VALUES ($1, $2)
            ON CONFLICT (client_ulid, contractor_ulid) DO NOTHING
            ",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
