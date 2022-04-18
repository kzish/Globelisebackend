use axum::extract::{Extension, Json};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

use self::{account::BranchAccountDetails, bank::BranchBankDetails, payroll::BranchPaymentDetails};

pub mod account;
pub mod bank;
pub mod payroll;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchDetails {
    pub account: BranchAccountDetails,
    pub bank: BranchBankDetails,
    pub payroll: BranchPaymentDetails,
}

pub async fn create_branch(
    claims: Token<AccessToken>,
    Json(request): Json<BranchDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    database.create_branch(claims.payload.ulid).await?;
    database
        .branch_account_details(claims.payload.ulid, request.account)
        .await?;
    database
        .branch_bank_details(claims.payload.ulid, request.bank)
        .await?;
    database
        .branch_payroll_details(claims.payload.ulid, request.payroll)
        .await?;

    Ok(())
}

impl Database {
    pub async fn create_branch(&self, client_ulid: Ulid) -> GlobeliseResult<()> {
        if self
            .user(client_ulid, Some(UserType::Entity))
            .await?
            .is_none()
        {
            return Err(GlobeliseError::Forbidden);
        }

        let ulid = Ulid::generate();

        let query = "
            INSERT INTO entity_client_branches (
                ulid, client_ulid
            ) VALUES (
                $1, $2
            )";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(ulid_to_sql_uuid(client_ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}
