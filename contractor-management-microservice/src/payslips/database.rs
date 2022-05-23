use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use uuid::Uuid;

use crate::{common::PaginatedQuery, database::Database};

use super::{CreatePayslipsIndex, PayslipsIndex};

impl Database {
    /// Indexes tax report.
    pub async fn payslips_index(
        &self,
        query: PaginatedQuery,
    ) -> GlobeliseResult<Vec<PayslipsIndex>> {
        let (limit, offset) = calc_limit_and_offset(query.per_page, query.page);

        let index = sqlx::query_as(
            "
            SELECT
                payslip_ulid, client_name, contractor_name, contract_name, payslip_title,
                payment_date, begin_period, end_period
            FROM
                payslips_index
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
    pub async fn create_payslip(&self, query: CreatePayslipsIndex) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "
            INSERT INTO payslips
            (ulid, client_ulid, contractor_ulid, contract_ulid, payslip_title,
            payment_date, begin_period, end_period, payslip_file)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(ulid)
        .bind(query.client_ulid)
        .bind(query.contractor_ulid)
        .bind(query.contract_ulid)
        .bind(query.payslip_title)
        .bind(query.payment_date)
        .bind(query.begin_period)
        .bind(query.end_period)
        .bind(query.payslip_file)
        .execute(&self.0)
        .await?;

        Ok(ulid)
    }
}
