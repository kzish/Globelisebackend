use common_utils::error::GlobeliseResult;

use crate::{common::ulid_to_sql_uuid, database::Database};

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
                    ulid, invoice_group_ulid, contract_ulid, invoice_id,
                    invoice_due, invoice_status, invoice_amount
                FROM
                    invoice_individual_index
                WHERE
                    invoice_group_ulid = $1 AND
                    ($2 IS NULL OR (invoice_name ~* $2)) AND
                    ($3 IS NULL OR (contractor_ulid ~* $3)) AND
                    ($4 IS NULL OR (client_ulid ~* $4)) AND
                LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(query.invoice_group_ulid))
        .bind(query.paginated_query.query)
        .bind(query.paginated_query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.paginated_query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.paginated_query.per_page.get())
        .bind((query.paginated_query.page.get() - 1) * query.paginated_query.per_page.get())
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
                    ulid, invoice_group_ulid, contract_ulid, invoice_id,
                    invoice_due, invoice_status, invoice_amount
                FROM
                    invoice_group_index
                WHERE
                    invoice_group_ulid = $1 AND
                    ($2 IS NULL OR (invoice_name ~* $2)) AND
                    ($3 IS NULL OR (contractor_ulid ~* $3)) AND
                    ($4 IS NULL OR (client_ulid ~* $4)) AND
                LIMIT $3 OFFSET $4",
        )
        .bind(ulid_to_sql_uuid(query.invoice_group_ulid))
        .bind(query.paginated_query.query)
        .bind(query.paginated_query.contractor_ulid.map(ulid_to_sql_uuid))
        .bind(query.paginated_query.client_ulid.map(ulid_to_sql_uuid))
        .bind(query.paginated_query.per_page.get())
        .bind((query.paginated_query.page.get() - 1) * query.paginated_query.per_page.get())
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }
}
