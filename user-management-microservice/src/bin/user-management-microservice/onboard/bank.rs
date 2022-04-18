use axum::extract::{Extension, Json};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::Deserialize;
use user_management_microservice_sdk::{
    token::AccessToken,
    user::{Role, UserType},
};

use crate::database::{Database, SharedDatabase};

pub async fn onboard_contractor_bank_details(
    claims: Token<AccessToken>,
    Json(details): Json<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .onboard_contractor_bank_details(claims.payload.ulid, claims.payload.user_type, details)
        .await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

impl Database {
    pub async fn onboard_contractor_bank_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        details: BankDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = user_type.db_onboard_details_prefix(Role::Contractor) + "_bank_details";
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, bank_name, bank_account_name, bank_account_number)
            VALUES ($4, $1, $2, $3)
            ON CONFLICT(ulid) DO UPDATE SET 
            bank_name = $1, bank_account_name = $2, bank_account_number = $3",
        );
        sqlx::query(&query)
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
