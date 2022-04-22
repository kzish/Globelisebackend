use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::FORM_DATA_LENGTH_LIMIT,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchDetailsRequest {
    pub branch_ulid: Ulid,
}

pub async fn post_branch(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<BranchDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let ulid = database.create_branch(claims.payload.ulid).await?;

    database
        .post_branch_account_details(ulid, request.account)
        .await?;

    database
        .post_branch_bank_details(ulid, request.bank)
        .await?;

    database
        .post_branch_payroll_details(ulid, request.payroll)
        .await?;

    Ok(())
}

pub async fn get_branch(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<BranchDetailsRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, request.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let account = database
        .get_branch_account_details(request.branch_ulid)
        .await?;

    let bank = database
        .get_branch_bank_details(request.branch_ulid)
        .await?;

    let payroll = database
        .get_branch_payroll_details(request.branch_ulid)
        .await?;

    Ok(Json(BranchDetails {
        account,
        bank,
        payroll,
    }))
}

impl Database {
    pub async fn create_branch(&self, client_ulid: Ulid) -> GlobeliseResult<Ulid> {
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

        Ok(ulid)
    }

    pub async fn client_owns_branch(
        &self,
        client_ulid: Ulid,
        branch_ulid: Ulid,
    ) -> GlobeliseResult<bool> {
        let query = "
            INSERT INTO entity_client_branches (
                ulid, client_ulid
            ) VALUES (
                $1, $2
            )";

        let result = sqlx::query(query)
            .bind(ulid_to_sql_uuid(branch_ulid))
            .bind(ulid_to_sql_uuid(client_ulid))
            .fetch_optional(&self.0)
            .await?
            .is_some();

        Ok(result)
    }
}
