use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{Currency, UserType, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

impl Database {
    #[allow(clippy::too_many_arguments)]
    pub async fn post_branch_bank_details(
        &self,
        ulid: Uuid,
        currency: Currency,
        bank_name: String,
        bank_account_name: String,
        bank_account_number: String,
        swift_code: Option<String>,
        bank_key: Option<String>,
        iban: Option<String>,
        bank_code: Option<String>,
        branch_code: Option<String>,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO entity_client_branch_bank_details (
                ulid, currency, bank_name, bank_account_name, bank_account_number,
                swift_code, bank_key, iban, bank_code, branch_code
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10
            ) ON CONFLICT(ulid) DO UPDATE SET 
                currency = $2, bank_name = $3, bank_account_name = $4, bank_account_number = $5,
                swift_code = $6, bank_key = $7, iban = $8, bank_code = $9, branch_code = $10",
        )
        .bind(ulid)
        .bind(currency)
        .bind(bank_name)
        .bind(bank_account_name)
        .bind(bank_account_number)
        .bind(swift_code)
        .bind(bank_key)
        .bind(iban)
        .bind(bank_code)
        .bind(branch_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_one_branch_bank_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<BranchBankDetails>> {
        let result = sqlx::query_as(
            "
            SELECT  
                ulid, currency, bank_name, bank_account_name, bank_account_number,
                swift_code, bank_key, iban, bank_code, branch_code
            FROM
                entity_client_branch_bank_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_optional(&self.0)
        .await?;

        Ok(result)
    }
}

pub async fn post_branch_bank_details(
    claims: Token<UserAccessToken>,
    Path(branch_ulid): Path<Uuid>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<BranchBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database
        .post_branch_bank_details(
            branch_ulid,
            body.currency,
            body.bank_name,
            body.bank_account_name,
            body.bank_account_number,
            body.swift_code,
            body.bank_key,
            body.iban,
            body.bank_code,
            body.branch_code,
        )
        .await
}

pub async fn get_branch_bank_details(
    claims: Token<UserAccessToken>,
    Path(branch_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchBankDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let result = database
        .get_one_branch_bank_details(branch_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(result))
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchBankDetails {
    pub currency: Currency,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub swift_code: Option<String>,
    pub bank_key: Option<String>,
    pub iban: Option<String>,
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchBankDetailsRequest {
    branch_ulid: Uuid,
}
