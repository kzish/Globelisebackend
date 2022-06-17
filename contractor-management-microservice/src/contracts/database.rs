use common_utils::{calc_limit_and_offset, custom_serde::Currency, error::GlobeliseResult};
use user_management_microservice_sdk::user::UserRole;
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::Database};

use super::{ClientsIndex, ContractorsIndex, ContractsIndex};

impl Database {
    /// Counts the number of contracts.
    pub async fn count_number_of_contracts(
        &self,
        ulid: Uuid,
        role: UserRole,
    ) -> GlobeliseResult<i64> {
        let client_ulid = match role {
            UserRole::Client => Some(ulid),
            UserRole::Contractor => None,
        };
        let contractor_ulid = match role {
            UserRole::Client => None,
            UserRole::Contractor => Some(ulid),
        };

        let result = sqlx::query_scalar(
            "
            SELECT
                COUNT(*)
            FROM
                contracts
            WHERE
                ($1 IS NULL OR (client_ulid = $1)) AND
                ($2 IS NULL OR (contractor_ulid = $2))",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }

    /// Indexes clients that a contractor works for.
    pub async fn select_many_clients_for_contractors(
        &self,
        contractor_ulid: Uuid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ClientsIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT DISTINCT
                client_ulid, client_name
            FROM
                clients_index_for_contractors
            WHERE
                contractor_ulid = $1 AND
                ($2 IS NULL OR client_name ~* $2)
            LIMIT $3 OFFSET $4",
        )
        .bind(contractor_ulid)
        .bind(query.search_text)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Indexes contracts working for a client.
    pub async fn select_many_contractors_for_clients(
        &self,
        client_ulid: Uuid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractorsIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                contractor_ulid, contractor_name, contract_name, contract_status,
                job_title, seniority
            FROM
                contractors_index_for_clients
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR (contractor_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(client_ulid)
        .bind(query.search_text)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    pub async fn select_many_contracts(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        contractor_ulid: Option<Uuid>,
        client_ulid: Option<Uuid>,
        branch_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Vec<ContractsIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let index = sqlx::query_as(
            "
            SELECT
                *
            FROM
                contracts_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (contract_name ~* $3 OR client_name ~* $3))AND
                ($4 IS NULL OR branch_ulid = $4)
            LIMIT
                $5
            OFFSET
                $6",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(query)
        .bind(branch_ulid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_contract(
        &self,
        client_ulid: Uuid,
        contractor_ulid: Uuid,
        branch_ulid: Option<Uuid>,
        contract_name: &String,
        contract_type: &String,
        job_title: &String,
        contract_status: &String,
        contract_amount: sqlx::types::Decimal,
        currency: Currency,
        seniority: &String,
        begin_at: &sqlx::types::time::OffsetDateTime,
        end_at: &sqlx::types::time::OffsetDateTime,
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "
            INSERT INTO contracts (
                ulid, client_ulid, contractor_ulid, contract_name, contract_type,
                contract_status, contract_amount, currency, job_title, seniority,
                begin_at, end_at, branch_ulid
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13
                    )",
        )
        .bind(ulid)
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(contract_name)
        .bind(contract_type)
        .bind(contract_status)
        .bind(contract_amount)
        .bind(currency)
        .bind(job_title)
        .bind(seniority)
        .bind(begin_at)
        .bind(end_at)
        .bind(branch_ulid)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }

    pub async fn select_one_contract(
        &self,
        contract_ulid: Option<Uuid>,
        contractor_ulid: Option<Uuid>,
        client_ulid: Option<Uuid>,
        query: Option<String>,
        branch_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Option<ContractsIndex>> {
        let result = sqlx::query_as(
            "
        SELECT
            *
        FROM
            contracts_index
        WHERE
            ($1 IS NULL OR contract_ulid = $1) AND
            ($2 IS NULL OR client_ulid = $2) AND
            ($3 IS NULL OR contractor_ulid = $3) AND
            ($4 IS NULL OR (contract_name ~* $4 OR client_name ~* $4 OR contractor_name ~* $4)) AND
            ($5 IS NULL OR branch_ulid = $5)",
        )
        .bind(contract_ulid)
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(query)
        .bind(branch_ulid)
        .fetch_optional(&self.0)
        .await?;

        Ok(result)
    }
}
