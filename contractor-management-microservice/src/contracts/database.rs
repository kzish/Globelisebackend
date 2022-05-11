use common_utils::{calc_limit_and_offset, error::GlobeliseResult, ulid_to_sql_uuid};
use rusty_ulid::Ulid;
use user_management_microservice_sdk::user::Role;

use crate::{common::PaginatedQuery, database::Database};

use super::{
    ClientsIndex, ContractorsIndex, ContractsIndexForClient, ContractsIndexForContractor,
    ContractsIndexForEorAdmin, CreateContractRequestForEorAdmin, GetContractsRequest,
};

impl Database {
    /// Counts the number of contracts.
    pub async fn count_number_of_contracts(&self, ulid: Ulid, role: Role) -> GlobeliseResult<i64> {
        let client_ulid = match role {
            Role::Client => Some(ulid_to_sql_uuid(ulid)),
            Role::Contractor => None,
        };
        let contractor_ulid = match role {
            Role::Client => None,
            Role::Contractor => Some(ulid_to_sql_uuid(ulid)),
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
    pub async fn clients_index(
        &self,
        contractor_ulid: Ulid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ClientsIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT DISTINCT
                client_ulid, client_name
            FROM
                contractors_index
            WHERE
                contractor_ulid = $1 AND
                ($2 IS NULL OR (client_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(query.query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Indexes contracts working for a client.
    pub async fn contractors_index(
        &self,
        client_ulid: Ulid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractorsIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                contractor_ulid, contractor_name, contract_name, contract_status,
                job_title, seniority
            FROM
                contractors_index
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR (contractor_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contracts_index_for_client(
        &self,
        client_ulid: Ulid,
        query: GetContractsRequest,
    ) -> GlobeliseResult<Vec<ContractsIndexForClient>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                contract_ulid, contract_name, contract_type,
                contractor_name, contract_status, contract_amount, currency,
                begin_at, end_at, branch_ulid, job_title
            FROM
                contracts_index
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (contract_name ~* $3 OR contractor_name ~* $3)) AND
                ($4 IS NULL OR branch_ulid = $4)
            LIMIT $5 OFFSET $6",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.query)
        .bind(query.branch_ulid.map(ulid_to_sql_uuid))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contracts_index_for_contractor(
        &self,
        contractor_ulid: Ulid,
        query: GetContractsRequest,
    ) -> GlobeliseResult<Vec<ContractsIndexForContractor>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                contract_ulid, contract_name, contract_type,
                client_name, contract_status, contract_amount, currency,
                begin_at, end_at, branch_ulid
            FROM
                contracts_index
            WHERE
                contractor_ulid = $1 AND
                ($2 IS NULL OR client_ulid = $2) AND
                ($3 IS NULL OR (contract_name ~* $3 OR client_name ~* $3)) AND
                ($4 IS NULL OR branch_ulid = $4)
            LIMIT $5 OFFSET $6",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.query)
        .bind(query.branch_ulid.map(ulid_to_sql_uuid))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract for EOR admin purposes
    pub async fn eor_admin_contracts_index(
        &self,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractsIndexForEorAdmin>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                contract_ulid, contract_name, contract_type, client_name, contractor_name, 
                contract_status, contract_amount, currency, begin_at, end_at, job_title
            FROM
                contracts_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (contract_name ~* $3 OR client_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Create contract
    pub async fn create_contract(
        &self,
        request: CreateContractRequestForEorAdmin,
    ) -> GlobeliseResult<()> {
        let ulid = rusty_ulid::Ulid::generate();

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
        .bind(ulid_to_sql_uuid(ulid))
        .bind(ulid_to_sql_uuid(request.client_ulid))
        .bind(ulid_to_sql_uuid(request.contractor_ulid))
        .bind(request.contract_name)
        .bind(request.contract_type)
        .bind(request.contract_status)
        .bind(request.contract_amount)
        .bind(request.currency)
        .bind(request.job_title)
        .bind(request.seniority)
        .bind(request.begin_at)
        .bind(request.end_at)
        .bind(ulid_to_sql_uuid(request.branch_ulid))
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
