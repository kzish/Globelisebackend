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
    client_name: String,
    contractor_ulid: Uuid,
    contractor_name: String,
}

impl Database {
    pub async fn select_many_client_contractor_pair_index(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        client_ulid: Option<Uuid>,
        contractor_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Vec<ClientContractorPairIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                client_contractor_pair_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR client_name ~* $3 OR contractor_name ~* $3)
            LIMIT
                $4
            OFFSET
                $5",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(query)
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
        contract_ulid: Option<Uuid>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO client_contractor_pairs (
                client_ulid, contractor_ulid, contract_ulid
            ) VALUES (
                $1, $2, $3)",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(contract_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
