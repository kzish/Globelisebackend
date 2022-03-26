use common_utils::error::GlobeliseResult;

use rusty_ulid::Ulid;

use user_management_microservice_sdk::Role;

use crate::{
    common::{ulid_to_sql_uuid, PaginatedQuery},
    database::Database,
};

use super::{ContractorsIndex, ContractsIndexForClient, ContractsIndexForContractor};

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

    /// Indexes contracts working for a client.
    pub async fn contractors_index(
        &self,
        client_ulid: Ulid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractorsIndex>> {
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
        .bind(query.search_text)
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contracts_index_for_client(
        &self,
        client_ulid: Ulid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractsIndexForClient>> {
        let index = sqlx::query_as(
            "
            SELECT
                ulid, contract_name, contract_type, contractor_ulid,
                contractor_name, contract_status, contract_amount, currency,
                begin_at, end_at
            FROM
                contracts_index_for_client
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR (contract_name ~* $2 OR contractor_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.search_text)
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contracts_index_for_contractor(
        &self,
        contractor_ulid: Ulid,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractsIndexForContractor>> {
        let index = sqlx::query_as(
            "
            SELECT
                ulid, contract_name, contract_type, client_ulid,
                client_name, contract_status, contract_amount, currency,
                begin_at, end_at
            FROM
                contracts_index_for_contractor
            WHERE
                contractor_ulid = $1 AND
                ($2 IS NULL OR (contract_name ~* $2 OR client_name ~* $2))
            LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(query.search_text)
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract for EOR admin purposes
    pub async fn eor_admin_contract_index(
        &self,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractsIndexForClient>> {
        let index = sqlx::query_as(
            "
            SELECT
                ulid, contract_name, contract_type, client_ulid,
                client_name, contract_status, contract_amount, currency,
                begin_at, end_at
            FROM
                contracts_index_for_contractor
            WHERE
                ($1 IS NULL OR (contract_name ~* $1 OR client_name ~* $1))
            LIMIT $2 OFFSET $3",
        )
        .bind(query.search_text)
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }
}
