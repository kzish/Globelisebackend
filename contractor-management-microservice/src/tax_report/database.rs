use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use uuid::Uuid;

use crate::database::Database;

use super::{CreateTaxReportIndex, TaxReportIndex};

impl Database {
    pub async fn select_many_tax_reports(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        search_text: Option<String>,
        contractor_ulid: Option<Uuid>,
        client_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Vec<TaxReportIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

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
            LIMIT 
                $4
            OFFSET 
                $5",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(search_text)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    pub async fn select_one_tax_report(
        &self,
        tax_report_ulid: Option<Uuid>,
        contractor_ulid: Option<Uuid>,
        client_ulid: Option<Uuid>,
        search_text: Option<String>,
    ) -> GlobeliseResult<Option<TaxReportIndex>> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                tax_reports_index
            WHERE
                ($1 IS NULL OR tax_report_ulid = $1) AND
                ($2 IS NULL OR client_ulid = $2) AND
                ($3 IS NULL OR contractor_ulid = $3) AND
                ($4 IS NULL OR (client_name ~* $4 OR contractor_name ~* $4))",
        )
        .bind(tax_report_ulid)
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(search_text)
        .fetch_optional(&self.0)
        .await?;

        Ok(result)
    }

    pub async fn insert_one_tax_report(&self, query: CreateTaxReportIndex) -> GlobeliseResult<()> {
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
