use common_utils::error::GlobeliseResult;

use rusty_ulid::Ulid;

use crate::{common::ulid_to_sql_uuid, database::Database};

use super::{CreateTaxReportIndex, TaxReportIndex, TaxReportIndexQuery};

impl Database {
    /// Indexes tax report.
    pub async fn tax_report_index(
        &self,
        query: TaxReportIndexQuery,
    ) -> GlobeliseResult<Vec<TaxReportIndex>> {
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
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.paginated_search.search_text)
        .bind(query.paginated_search.per_page)
        .bind((query.paginated_search.page - 1) * query.paginated_search.per_page)
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
