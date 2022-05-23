use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::Database};

use super::{CreateTaxReportIndex, TaxReportIndex};

impl Database {
    /// Indexes tax report.
    pub async fn tax_report_index(
        &self,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<TaxReportIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                tax_report_ulid, client_name, contractor_name, contract_name, tax_interval,
                tax_name, begin_period, end_period, country
            FROM
                tax_reports_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (client_name ~* $3 OR contractor_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(query.client_ulid)
        .bind(query.contractor_ulid)
        .bind(query.query)
        .bind(limit)
        .bind(offset)
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
            ($1, $2, $3, $4, $5::interval_type, $6, $7, $8, $9, $10)",
        )
        .bind(Uuid::new_v4())
        .bind(query.client_ulid)
        .bind(query.contractor_ulid)
        .bind(query.contract_ulid)
        .bind(query.tax_interval.as_str())
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
