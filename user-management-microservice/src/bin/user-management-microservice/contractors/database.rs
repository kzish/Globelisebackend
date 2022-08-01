use common_utils::{calc_limit_and_offset, error::GlobeliseResult};

use super::{ContractorsQuery, ContractorsResponse};

use crate::database::Database;

impl Database {
    pub async fn client_get_contractors(
        &self,
        request: ContractorsQuery,
    ) -> GlobeliseResult<Vec<ContractorsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "
            SELECT * FROM
                contractors_index_for_clients
            WHERE 
                ($3 IS NULL OR (name ~* $3 OR branch_name ~* $3 OR cost_center_name ~* $3 OR email ~* $3))
            AND 
                ($4 IS NULL OR client_ulid = $4)
            AND
                ($5 IS NULL OR branch_ulid = $5)
            AND 
                ($6 IS NULL OR cost_center_ulid = $6)
            LIMIT $1
            OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(request.query)
        .bind(request.client_ulid)
        .bind(request.branch_ulid)
        .bind(request.cost_center_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }
}
