use axum::extract::{Extension, Json};
use common_utils::{
    custom_serde::Currency,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

impl Database {
    pub async fn branch_bank_details(
        &self,
        ulid: Ulid,
        details: BranchBankDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        sqlx::query(
            "
            INSERT INTO entity_clients_bank_details (
                ulid, currency, bank_name, bank_account_name, bank_account_number,
                swift_code, bank_key, iban, bank_code, branch_code
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10
            ) ON CONFLICT(ulid) DO UPDATE SET 
            currency = $2, bank_name = $3, bank_account_name = $4, bank_account_number = $5,
            swift_code = $6, bank_key = $7, iban = $8, bank_code = $9, branch_code = $10",
        )
        .bind(details.bank_name)
        .bind(details.account_name)
        .bind(details.account_number)
        .bind(ulid_to_sql_uuid(ulid))
        .execute(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}

pub async fn branch_bank_details(
    claims: Token<AccessToken>,
    Json(details): Json<BranchBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .branch_bank_details(claims.payload.ulid, details)
        .await
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchBankDetails {
    pub currency: Currency,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
    pub swift_code: Option<String>,
    pub bank_key: Option<String>,
    pub iban: Option<String>,
    pub bank_code: Option<String>,
}
