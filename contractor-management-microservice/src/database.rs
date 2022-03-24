use std::{sync::Arc, time::Duration};

use common_utils::error::GlobeliseResult;
use rusty_ulid::Ulid;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;
use user_management_microservice_sdk::Role;

use crate::{
    contracts::{
        ContractorsIndex, ContractsIndexForClient, ContractsIndexForContractor, PaginationQuery,
    },
    tax_report::{CreateTaxReportIndex, TaxReportIndex, TaxReportIndexQuery},
};

pub type SharedDatabase = Arc<Mutex<Database>>;

/// Convenience wrapper around PostgreSQL.
pub struct Database(Pool<Postgres>);

impl Database {
    /// Connects to PostgreSQL.
    pub async fn new() -> Self {
        let connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_timeout(Duration::from_secs(3))
            .connect(&connection_str)
            .await
            .expect("Cannot connect to database");

        Self(pool)
    }
}

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
        query: PaginationQuery,
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
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contracts_index_for_client(
        &self,
        client_ulid: Ulid,
        query: PaginationQuery,
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
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract of a given contractor
    pub async fn contracts_index_for_contractor(
        &self,
        contractor_ulid: Ulid,
        query: PaginationQuery,
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
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index contract for EOR admin purposes
    pub async fn eor_admin_contract_index(
        &self,
        query: PaginationQuery,
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
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Indexes tax report.
    pub async fn tax_report_index(
        &self,
        ulid: Ulid,
        query: TaxReportIndexQuery,
    ) -> GlobeliseResult<Vec<TaxReportIndex>> {
        let client_ulid = match query.role {
            Role::Client => Some(ulid_to_sql_uuid(ulid)),
            Role::Contractor => None,
        };
        let contractor_ulid = match query.role {
            Role::Client => None,
            Role::Contractor => Some(ulid_to_sql_uuid(ulid)),
        };
        let index = sqlx::query_as(
            "
            SELECT
                ulid, client_name, contractor_name, contract_name, tax_interval,
                tax_name, begin_period, end_period, country
            FROM
                tax_reports_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (client_name ~* $3 OR contractor_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(query.search_text)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Create tax report
    pub async fn create_tax_report(&self, query: CreateTaxReportIndex) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO tax_reports
            (ulid, client_ulid, contractor_ulid, contract_ulid, tax_interval,
            tax_name, begin_period, end_period, country, tax_report_file)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(ulid_to_sql_uuid(Ulid::generate()))
        .bind(ulid_to_sql_uuid(query.client_ulid))
        .bind(ulid_to_sql_uuid(query.contractor_ulid))
        .bind(query.contract_ulid.map(ulid_to_sql_uuid))
        .bind(query.tax_interval)
        .bind(query.tax_name)
        .bind(query.begin_period)
        .bind(query.end_period)
        .bind(query.country)
        .bind(query.tax_report_file)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
