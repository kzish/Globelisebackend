use common_utils::{error::GlobeliseResult, ulid_to_sql_uuid};

use crate::database::Database;

use super::{
    InvoiceGroupIndex, InvoiceGroupIndexQuery, InvoiceIndividualIndex, InvoiceIndividualIndexQuery,
};

impl Database {
    /// Index individual invoice
    pub async fn invoice_individual_index(
        &self,
        query: InvoiceIndividualIndexQuery,
    ) -> GlobeliseResult<Vec<InvoiceIndividualIndex>> {
        let index = sqlx::query_as(
            "
                SELECT
                    ulid, invoice_group_ulid, contractor_ulid, invoice_id,
                    invoice_due, invoice_status, invoice_amount
                FROM
                    invoice_individual_index
                WHERE
                    invoice_group_ulid = $1 AND
                    ($2 IS NULL OR (invoice_name ~* $2)) AND
                    ($3 IS NULL OR (contractor_ulid = $3)) AND
                    ($4 IS NULL OR (client_ulid = $4))
                LIMIT $5 OFFSET $6",
        )
        .bind(ulid_to_sql_uuid(query.invoice_group_ulid))
        .bind(query.query)
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    /// Index group invoice
    pub async fn invoice_group_index(
        &self,
        query: InvoiceGroupIndexQuery,
    ) -> GlobeliseResult<Vec<InvoiceGroupIndex>> {
        let index = sqlx::query_as(
            "
                SELECT
                    ulid, invoice_group_ulid, contractor_ulid, invoice_id,
                    invoice_due, invoice_status, invoice_amount
                FROM
                    invoice_group_index
                WHERE
                    ($1 IS NULL OR ($1 = ANY(invoice_name))) AND
                    ($2 IS NULL OR ($2= ANY(contractor_ulid))) AND
                    ($3 IS NULL OR ($3 = ANY(client_ulid)))
                LIMIT $4 OFFSET $5",
        )
        .bind(query.query)
        .bind(query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.per_page.get())
        .bind((query.page.get() - 1) * query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }
}
