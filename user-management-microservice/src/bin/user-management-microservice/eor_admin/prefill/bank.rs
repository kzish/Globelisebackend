use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorBankDetails {
    pub email: EmailWrapper,
    pub client_ulid: Uuid,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

pub async fn individual_contractor_post_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillIndividualContractorBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_prefilled_individual_contractor_bank_details(
            body.email,
            body.client_ulid,
            body.bank_name,
            body.bank_account_name,
            body.bank_account_number,
            body.bank_code,
            body.branch_code,
        )
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorBankDetailsQuery {
    pub email: EmailWrapper,
    pub client_ulid: Uuid,
}

pub async fn individual_contractor_get_one(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<PrefillIndividualContractorBankDetailsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorBankDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefilled_individual_contractor_bank_details(query.email, query.client_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientBankDetails {
    pub email: EmailWrapper,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

pub async fn entity_client_post_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_prefilled_entity_client_bank_details(
            body.email,
            body.bank_name,
            body.bank_account_name,
            body.bank_account_number,
            body.bank_code,
            body.branch_code,
        )
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientBankDetailsQuery {
    pub email: EmailWrapper,
}

pub async fn entity_client_get_one(
    _: Token<AdminAccessToken>,
    Query(query): Query<PrefillEntityClientBankDetailsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PrefillEntityClientBankDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_prefilled_entity_client_bank_details(query.email)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

impl Database {
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_prefilled_individual_contractor_bank_details(
        &self,
        email: EmailWrapper,
        client_ulid: Uuid,
        bank_name: String,
        bank_account_name: String,
        bank_account_number: String,
        bank_code: String,
        branch_code: String,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_individual_contractors_bank_details (
                email, client_ulid, bank_name, bank_account_name, bank_account_number,
                bank_code, branch_code
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7
            ) ON CONFLICT(email, client_ulid) DO UPDATE SET 
                bank_name = $3, bank_account_name = $4, bank_account_number = $5,
                bank_code = $6, branch_code = $7";

        sqlx::query(query)
            .bind(email)
            .bind(client_ulid)
            .bind(bank_name)
            .bind(bank_account_name)
            .bind(bank_account_number)
            .bind(bank_code)
            .bind(branch_code)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_prefilled_individual_contractor_bank_details(
        &self,
        email: EmailWrapper,
        client_ulid: Uuid,
    ) -> GlobeliseResult<Option<PrefillIndividualContractorBankDetails>> {
        let query = "
            SELECT
               *
            FROM
                prefilled_individual_contractors_bank_details
            WHERE
                email = $1 AND
                client_ulid = $2";

        let result = sqlx::query_as(query)
            .bind(email)
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn insert_one_prefilled_entity_client_bank_details(
        &self,
        email: EmailWrapper,
        bank_name: String,
        bank_account_name: String,
        bank_account_number: String,
        bank_code: String,
        branch_code: String,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_bank_details (
                email, bank_name, bank_account_name, bank_account_number, bank_code, 
                branch_code
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6
            ) ON CONFLICT(email) DO UPDATE SET 
                bank_name = $2, bank_account_name = $3, bank_account_number = $4, bank_code = $5,
                branch_code = $6";

        sqlx::query(query)
            .bind(email)
            .bind(bank_name)
            .bind(bank_account_name)
            .bind(bank_account_number)
            .bind(bank_code)
            .bind(branch_code)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_prefilled_entity_client_bank_details(
        &self,
        email: EmailWrapper,
    ) -> GlobeliseResult<Option<PrefillEntityClientBankDetails>> {
        let query = "
            SELECT
               *
            FROM
                prefilled_individual_contractors_bank_details
            WHERE
                email = $1";

        let result = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
