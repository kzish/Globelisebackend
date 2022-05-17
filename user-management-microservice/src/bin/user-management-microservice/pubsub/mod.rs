use axum::{Extension, Json};
use common_utils::{
    error::GlobeliseResult,
    pubsub::{CreateOrUpdateContracts, TopicSubscriberEvent},
    ulid_to_sql_uuid,
};

use crate::database::{Database, SharedDatabase};

pub async fn create_or_update_contracts(
    Json(body): Json<TopicSubscriberEvent<CreateOrUpdateContracts>>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .pubsub_create_or_update_contracts(body.data)
        .await?;

    Ok(())
}

impl Database {
    pub async fn pubsub_create_or_update_contracts(
        &self,
        CreateOrUpdateContracts {
            ulid,
            client_ulid,
            contractor_ulid,
            contract_name,
            contract_type,
            contract_status,
            contract_amount,
            currency,
            job_title,
            seniority,
            begin_at,
            end_at,
            branch_ulid,
        }: CreateOrUpdateContracts,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO pubsub_contracts (
                ulid, client_ulid, contractor_ulid, contract_name, contract_type,
                contract_status, contract_amount, currency, job_title, seniority,
                begin_at, end_at, branch_ulid
            ) VALUES (
                $1::uuid, $2::uuid, $3::uuid, $4, $5,
                $6, $7, $8::currency, $9, $10,
                $11, $12, $13
            ) ON CONFLICT(ulid) DO UPDATE SET 
            client_ulid = $2, contractor_ulid = $3, contract_name = $4, contract_type = $5,
            contract_status = $6, contract_amount = $7, currency = $8, job_title = $9, seniority = $10,
            begin_at = $11, end_at = $12, branch_ulid = $13
        ",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .bind(ulid_to_sql_uuid(client_ulid))
        .bind(ulid_to_sql_uuid(contractor_ulid))
        .bind(contract_name)
        .bind(contract_type)
        .bind(contract_status)
        .bind(contract_amount)
        .bind(currency)
        .bind(job_title)
        .bind(seniority)
        .bind(begin_at)
        .bind(end_at)
        .bind(ulid_to_sql_uuid(branch_ulid))
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
