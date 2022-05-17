use axum::extract::{ContentLengthLimit, Extension, Json, Path, Query};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

use self::{
    account::{BranchAccountDetails, PostBranchAccountDetailsInput},
    bank::{BranchBankDetails, PostBranchBankDetailsInput},
    payroll::BranchPayrollDetails,
};

pub mod account;
pub mod bank;
pub mod pay_items;
pub mod payroll;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchDetails {
    pub ulid: Ulid,
    pub client_ulid: Ulid,
    pub account: BranchAccountDetails,
    pub bank: BranchBankDetails,
    pub payroll: BranchPayrollDetails,
}

impl FromRow<'_, PgRow> for BranchDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let client_ulid = ulid_from_sql_uuid(row.try_get("client_ulid")?);
        let account = BranchAccountDetails::from_row(row)?;
        let bank = BranchBankDetails::from_row(row)?;
        let payroll = BranchPayrollDetails::from_row(row)?;
        Ok(BranchDetails {
            ulid,
            client_ulid,
            account,
            bank,
            payroll,
        })
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostBranchDetailsRequest {
    pub account: BranchAccountDetails,
    pub bank: BranchBankDetails,
    pub payroll: BranchPayrollDetails,
}

pub async fn user_post_branch(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
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
        .post_branch_account_details(PostBranchAccountDetailsInput {
            ulid,
            company_name: body.account.company_name,
            country: body.account.country,
            entity_type: body.account.entity_type,
            registration_number: body.account.registration_number,
            tax_id: body.account.tax_id,
            statutory_contribution_submission_number: body
                .account
                .statutory_contribution_submission_number,
            company_address: body.account.company_address,
            city: body.account.city,
            postal_code: body.account.postal_code,
            time_zone: body.account.time_zone,
            logo: body.account.logo,
        })
        .await?;

    database
        .post_branch_bank_details(PostBranchBankDetailsInput {
            ulid,
            currency: body.bank.currency,
            bank_name: body.bank.bank_name,
            bank_account_name: body.bank.bank_account_name,
            bank_account_number: body.bank.bank_account_number,
            swift_code: body.bank.swift_code,
            bank_key: body.bank.bank_key,
            iban: body.bank.iban,
            bank_code: body.bank.bank_code,
            branch_code: body.bank.branch_code,
        })
        .await?;

    database
        .post_branch_payroll_details(ulid, body.payroll.payment_date, body.payroll.cutoff_date)
        .await?;

    Ok(ulid.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetBranchDetailsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn user_get_branches(
    claims: Token<UserAccessToken>,
    Query(query): Query<GetBranchDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let result = database
        .get_entity_clients_branch_details(Some(claims.payload.ulid), query.page, query.per_page)
        .await?;

    Ok(Json(result))
}

pub async fn user_get_branch_by_ulid(
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

    let details = database
        .get_one_entity_clients_branch_details(branch_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(details))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteBranchRequest {
    pub branch_ulid: Ulid,
}

pub async fn user_delete_branch(
    claims: Token<UserAccessToken>,
    Query(query): Query<DeleteBranchRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, query.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_branch(claims.payload.ulid, query.branch_ulid)
        .await?;

    Ok(())
}

/// EOR-ADMIN

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostBranchDetailsRequestForAdmin {
    pub client_ulid: Ulid,
    pub account: BranchAccountDetails,
    pub bank: BranchBankDetails,
    pub payroll: BranchPayrollDetails,
}

pub async fn admin_post_branch(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostBranchDetailsRequestForAdmin>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let ulid = database.create_branch(body.client_ulid).await?;

    database
        .post_branch_account_details(PostBranchAccountDetailsInput {
            ulid,
            company_name: body.account.company_name,
            country: body.account.country,
            entity_type: body.account.entity_type,
            registration_number: body.account.registration_number,
            tax_id: body.account.tax_id,
            statutory_contribution_submission_number: body
                .account
                .statutory_contribution_submission_number,
            company_address: body.account.company_address,
            city: body.account.city,
            postal_code: body.account.postal_code,
            time_zone: body.account.time_zone,
            logo: body.account.logo,
        })
        .await?;

    database
        .post_branch_bank_details(PostBranchBankDetailsInput {
            ulid,
            currency: body.bank.currency,
            bank_name: body.bank.bank_name,
            bank_account_name: body.bank.bank_account_name,
            bank_account_number: body.bank.bank_account_number,
            swift_code: body.bank.swift_code,
            bank_key: body.bank.bank_key,
            iban: body.bank.iban,
            bank_code: body.bank.bank_code,
            branch_code: body.bank.branch_code,
        })
        .await?;

    database
        .post_branch_payroll_details(ulid, body.payroll.payment_date, body.payroll.cutoff_date)
        .await?;

    Ok(ulid.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetBranchDetailsRequestForAdmin {
    pub client_ulid: Option<Ulid>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn admin_get_branches(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetBranchDetailsRequestForAdmin>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
    let database = database.lock().await;

    let result = database
        .get_entity_clients_branch_details(query.client_ulid, query.page, query.per_page)
        .await?;

    Ok(Json(result))
}

pub async fn admin_get_branch_by_ulid(
    _: Token<AdminAccessToken>,
    Path(branch_ulid): Path<Ulid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchDetails>> {
    let database = database.lock().await;

    let details = database
        .get_one_entity_clients_branch_details(branch_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(details))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteBranchRequestForAdmin {
    pub client_ulid: Ulid,
    pub branch_ulid: Ulid,
}

pub async fn admin_delete_branch(
    _: Token<AdminAccessToken>,
    Query(query): Query<DeleteBranchRequestForAdmin>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_branch(query.client_ulid, query.branch_ulid)
        .await?;

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

    pub async fn delete_branch(&self, client_ulid: Ulid, branch_ulid: Ulid) -> GlobeliseResult<()> {
        let query = "
            DELETE FROM
                entity_client_branches 
            WHERE 
                ulid = $1";

        sqlx::query(query)
            .bind(ulid_to_sql_uuid(branch_ulid))
            .bind(ulid_to_sql_uuid(client_ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_entity_clients_branch_details(
        &self,
        client_ulid: Option<Ulid>,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<BranchDetails>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let query = "
            SELECT
                -- branch details
                ulid, client_ulid,
                -- account details
                company_name, country, entity_type, registration_number, tax_id, 
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
                $1 IS NULL OR (client_ulid = $1)
            LIMIT $2 OFFSET $3";

        let result = sqlx::query_as(query)
            .bind(client_ulid.map(ulid_to_sql_uuid))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }

    pub async fn get_one_entity_clients_branch_details(
        &self,
        branch_ulid: Ulid,
    ) -> GlobeliseResult<Option<BranchDetails>> {
        let query = "
            SELECT
                -- branch details
                ulid, client_ulid
                -- account details
                company_name, country, entity_type, registration_number, tax_id, 
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
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid_to_sql_uuid(branch_ulid))
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
