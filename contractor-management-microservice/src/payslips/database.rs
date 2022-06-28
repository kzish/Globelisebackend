use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use uuid::Uuid;

use crate::database::Database;

use super::PayslipsIndex;

impl Database {
    pub async fn select_many_payslips(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        query: Option<String>,
        contractor_ulid: Option<Uuid>,
        client_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Vec<PayslipsIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let index = sqlx::query_as(
            "
            SELECT
                *
            FROM
                payslips_index
            WHERE
                ($1 IS NULL OR client_ulid = $1) AND
                ($2 IS NULL OR contractor_ulid = $2) AND
                ($3 IS NULL OR (client_name ~* $3 OR contractor_name ~* $3))
            LIMIT $4 OFFSET $5",
        )
        .bind(client_ulid)
        .bind(contractor_ulid)
        .bind(query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    pub async fn select_one_payslip_index(
        &self,
        ulid: Uuid,
        client_ulid: Option<Uuid>,
        contractor_ulid: Option<Uuid>,
    ) -> GlobeliseResult<Option<PayslipsIndex>> {
        let query = "
            SELECT
                *
            FROM
                payslips_index
            WHERE
                payslip_ulid = $1 AND
                ($2 IS NULL OR client_ulid = $2) AND
                ($3 IS NULL OR contractor_ulid = $3)";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .bind(client_ulid)
            .bind(contractor_ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_payslip(
        &self,
        client_ulid: Uuid,
        contractor_ulid: Uuid,
        contract_ulid: Option<Uuid>,
        payslip_title: String,
        payment_date: sqlx::types::time::OffsetDateTime,
        begin_period: sqlx::types::time::OffsetDateTime,
        end_period: sqlx::types::time::OffsetDateTime,
        payslip_file: Vec<u8>,
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        let query = "
        INSERT INTO payslips (
            ulid, client_ulid, contractor_ulid, contract_ulid, payslip_title,
            payment_date, begin_period, end_period, payslip_file
        ) VALUES (
            $1, $2, $3, $4, $5, 
            $6, $7, $8, $9)";

        sqlx::query(query)
            .bind(ulid)
            .bind(client_ulid)
            .bind(contractor_ulid)
            .bind(contract_ulid)
            .bind(payslip_title)
            .bind(payment_date)
            .bind(begin_period)
            .bind(end_period)
            .bind(payslip_file)
            .execute(&self.0)
            .await?;

        Ok(ulid)
    }

    pub async fn delete_one_payslip(
        &self,
        payslip_ulid: Uuid,
        client_ulid: Option<Uuid>,
        contractor_ulid: Option<Uuid>,
    ) -> GlobeliseResult<()> {
        let query = "
        DELETE FROM
            payslips
        WHERE 
            (ulid = $1) AND
            ($2 IS NULL OR client_ulid = $2) AND
            ($3 IS NULL OR contractor_ulid = $3)";

        sqlx::query(query)
            .bind(payslip_ulid)
            .bind(client_ulid)
            .bind(contractor_ulid)
            .execute(&self.0)
            .await?;

        Ok(())
    }
}
