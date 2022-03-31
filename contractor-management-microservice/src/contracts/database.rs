use common_utils::{error::GlobeliseResult, ulid_to_sql_uuid};
use rusty_ulid::Ulid;
use user_management_microservice_sdk::user::Role;

use crate::{common::PaginatedQuery, database::Database};

use super::{
    ContractorsIndex, ContractsIndexForClient, ContractsIndexForContractor,
    ContractsIndexForEorAdmin,
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
        .bind(query.query)
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
                contract_ulid, contract_name, contract_type,
                contractor_name, contract_status, contract_amount, currency,
                begin_at, end_at
            FROM
                contracts_index
            WHERE
                client_ulid = $1 AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (contract_name ~* $3 OR contractor_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.query)
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
                contract_ulid, contract_name, contract_type,
                client_name, contract_status, contract_amount, currency,
                begin_at, end_at
            FROM
                contracts_index
            WHERE
                contractor_ulid = $1 AND
                ($2 IS NULL OR client_ulid = $2) AND
                ($3 IS NULL OR (contract_name ~* $3 OR client_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.query)
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract for EOR admin purposes
    pub async fn eor_admin_contracts_index(
        &self,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<ContractsIndexForEorAdmin>> {
        let index = sqlx::query_as(
            "
            SELECT
                contract_ulid, contract_name, contract_type, client_name
                contractor_name, contract_status, contract_amount, currency,
                begin_at, end_at
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
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }
}
