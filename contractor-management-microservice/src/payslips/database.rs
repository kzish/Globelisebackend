use common_utils::error::GlobeliseResult;

use rusty_ulid::Ulid;

use crate::{common::ulid_to_sql_uuid, database::Database};

use super::{CreatePayslipsIndex, PayslipsIndex, PayslipsIndexQuery};

impl Database {
    /// Indexes tax report.
    pub async fn payslips_index(
        &self,
        query: PayslipsIndexQuery,
    ) -> GlobeliseResult<Vec<PayslipsIndex>> {
        let index = sqlx::query_as(
            "
            SELECT
                ulid, client_name, contractor_name, contract_name, payslip_title,
                payment_date, begin_period, end_period
            FROM
                payslips_index
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
    pub async fn create_payslip(&self, query: CreatePayslipsIndex) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO payslips
            (ulid, client_ulid, contractor_ulid, contract_ulid, payslip_title,
            payment_date, begin_period, end_period, payslip_file)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(ulid_to_sql_uuid(Ulid::generate()))
        .bind(ulid_to_sql_uuid(query.client_ulid))
        .bind(ulid_to_sql_uuid(query.contractor_ulid))
        .bind(query.contract_ulid.map(ulid_to_sql_uuid))
        .bind(query.payslip_title)
        .bind(query.payment_date)
        .bind(query.begin_period)
        .bind(query.end_period)
        .bind(query.payslip_file)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
