use axum::extract::{ContentLengthLimit, Extension, Json, Path, Query};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

use self::{account::BranchAccountDetails, bank::BranchBankDetails, payroll::BranchPayrollDetails};

pub mod account;
pub mod bank;
pub mod pay_items;
pub mod payroll;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostBranchDetailsRequest {
    pub account: BranchAccountDetails,
    pub bank: BranchBankDetails,
    pub payroll: BranchPayrollDetails,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchDetails {
    pub ulid: Ulid,
    pub account: BranchAccountDetails,
    pub bank: BranchBankDetails,
    pub payroll: BranchPayrollDetails,
}

impl FromRow<'_, PgRow> for BranchDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let account = BranchAccountDetails::from_row(row)?;
        let bank = BranchBankDetails::from_row(row)?;
        let payroll = BranchPayrollDetails::from_row(row)?;
        Ok(BranchDetails {
            ulid,
            account,
            bank,
            payroll,
        })
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteBranchRequest {
    pub branch_ulid: Ulid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetBranchDetailsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn post_branch(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PostBranchDetailsRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
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

    Ok(ulid.to_string())
}

pub async fn get_branches(
    claims: Token<UserAccessToken>,
    Query(query): Query<GetBranchDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let result = database
        .get_entity_clients_branch_details(claims.payload.ulid, query)
        .await?;

    Ok(Json(result))
}

pub async fn get_one_branch(
    claims: Token<UserAccessToken>,
    Path(branch_ulid): Path<Ulid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchDetails>> {
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

    let account = database.get_one_branch_account_details(branch_ulid).await?;

    let bank = database.get_one_branch_bank_details(branch_ulid).await?;

    let payroll = database.get_one_branch_payroll_details(branch_ulid).await?;

    Ok(Json(BranchDetails {
        ulid: branch_ulid,
        account,
        bank,
        payroll,
    }))
}

pub async fn delete_branch(
    claims: Token<UserAccessToken>,
    Json(request): Json<DeleteBranchRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    database.delete_branch(request, claims.payload.ulid).await?;

    Ok(())
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
            SELECT
                *
            FROM
                entity_client_branches
            WHERE
                ulid = $1 AND
                client_ulid = $2";

        let result = sqlx::query(query)
            .bind(ulid_to_sql_uuid(branch_ulid))
            .bind(ulid_to_sql_uuid(client_ulid))
            .fetch_optional(&self.0)
            .await?
            .is_some();

        Ok(result)
    }

    pub async fn delete_branch(
        &self,
        request: DeleteBranchRequest,
        client_ulid: Ulid,
    ) -> GlobeliseResult<()> {
        if self
            .user(client_ulid, Some(UserType::Entity))
            .await?
            .is_none()
        {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            DELETE FROM
                entity_client_branches 
            WHERE 
                ulid = $1
            AND
                client_ulid = $2";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(request.branch_ulid))
            .bind(ulid_to_sql_uuid(client_ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_entity_clients_branch_details(
        &self,
        client_ulid: Ulid,
        request: GetBranchDetailsRequest,
    ) -> GlobeliseResult<Vec<BranchDetails>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let query = "
            SELECT
                -- account details
                ulid, company_name, country, entity_type, registration_number, tax_id, 
                statutory_contribution_submission_number, company_address, city, 
                postal_code, time_zone, logo,
                -- bank details
                currency, bank_name, bank_account_name, bank_account_number,
                swift_code, bank_key, iban, bank_code, branch_code,
                -- payment details
                cutoff_date, payment_date
            FROM
                entity_clients_branch_details
            WHERE
                client_ulid = $1
            LIMIT $2 OFFSET $3";

        let result = sqlx::query_as(query)
            .bind(ulid_to_sql_uuid(client_ulid))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
