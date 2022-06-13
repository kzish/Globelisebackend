use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

use crate::database::Database;

use self::{account::BranchAccountDetails, bank::BranchBankDetails, payroll::BranchPayrollDetails};

pub mod account;
pub mod bank;
pub mod pay_items;
pub mod payroll;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchDetails {
    pub ulid: Uuid,
    pub client_ulid: Uuid,
    pub account: Option<BranchAccountDetails>,
    pub bank: Option<BranchBankDetails>,
    pub payroll: Option<BranchPayrollDetails>,
}

impl FromRow<'_, PgRow> for BranchDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let ulid = row.try_get("ulid")?;
        let client_ulid = row.try_get("client_ulid")?;
        let account = BranchAccountDetails::from_row(row).ok();
        let bank = BranchBankDetails::from_row(row).ok();
        let payroll = BranchPayrollDetails::from_row(row).ok();
        Ok(BranchDetails {
            ulid,
            client_ulid,
            account,
            bank,
            payroll,
        })
    }
}

pub mod user {
    use axum::extract::{ContentLengthLimit, Extension, Json, Path, Query};
    use common_utils::{
        custom_serde::FORM_DATA_LENGTH_LIMIT,
        error::{GlobeliseError, GlobeliseResult},
        token::Token,
    };
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
    use uuid::Uuid;

    use crate::database::SharedDatabase;

    use super::{
        account::BranchAccountDetails, bank::BranchBankDetails, payroll::BranchPayrollDetails,
    };

    use super::BranchDetails;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct PostBranchDetailsRequest {
        pub account: BranchAccountDetails,
        pub bank: BranchBankDetails,
        pub payroll: BranchPayrollDetails,
    }

    pub async fn post_one_branch(
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

        let ulid = database
            .insert_one_entity_client_branch(claims.payload.ulid)
            .await?;

        database
            .post_branch_account_details(
                ulid,
                body.account.branch_name,
                body.account.country,
                body.account.entity_type,
                body.account.registration_number,
                body.account.tax_id,
                body.account.statutory_contribution_submission_number,
                body.account.company_address,
                body.account.city,
                body.account.postal_code,
                body.account.time_zone,
                body.account.logo,
            )
            .await?;

        database
            .post_branch_bank_details(
                ulid,
                body.bank.currency,
                body.bank.bank_name,
                body.bank.bank_account_name,
                body.bank.bank_account_number,
                body.bank.swift_code,
                body.bank.bank_key,
                body.bank.iban,
                body.bank.bank_code,
                body.bank.branch_code,
            )
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

    pub async fn get_many_branches(
        claims: Token<UserAccessToken>,
        Query(query): Query<GetBranchDetailsRequest>,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
        if !matches!(claims.payload.user_type, UserType::Entity) {
            return Err(GlobeliseError::Forbidden);
        }

        let database = database.lock().await;

        let result = database
            .select_many_entity_clients_branch_details(
                Some(claims.payload.ulid),
                query.page,
                query.per_page,
            )
            .await?;

        Ok(Json(result))
    }

    pub async fn get_one_branch_by_ulid(
        claims: Token<UserAccessToken>,
        Path(branch_ulid): Path<Uuid>,
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
            .select_one_entity_clients_branch_details(branch_ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?;

        Ok(Json(details))
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct DeleteBranchRequest {
        pub branch_ulid: Uuid,
    }

    pub async fn delete_one_branch(
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
            .delete_one_branch(claims.payload.ulid, query.branch_ulid)
            .await?;

        Ok(())
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct GetManyBranchesIndividualContractorQuery {
        pub branch_ulid: Uuid,
        pub page: Option<u32>,
        pub per_page: Option<u32>,
    }

    pub async fn get_many_individual_contractors(
        claims: Token<UserAccessToken>,
        Query(query): Query<GetManyBranchesIndividualContractorQuery>,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
        let database = database.lock().await;

        if !database
            .client_owns_branch(claims.payload.ulid, query.branch_ulid)
            .await?
        {
            return Err(GlobeliseError::unauthorized(
                "This client does not own this branch",
            ));
        }

        let result = database
            .select_many_entity_client_branch_individual_contractors(
                Some(query.branch_ulid),
                query.page,
                query.per_page,
            )
            .await?;

        Ok(Json(result))
    }
}

pub mod eor_admin {
    use axum::extract::{ContentLengthLimit, Extension, Json, Path, Query};
    use common_utils::{
        custom_serde::FORM_DATA_LENGTH_LIMIT,
        error::{GlobeliseError, GlobeliseResult},
        token::Token,
    };
    use eor_admin_microservice_sdk::token::AdminAccessToken;
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use uuid::Uuid;

    use crate::database::SharedDatabase;

    use super::{
        account::BranchAccountDetails, bank::BranchBankDetails, payroll::BranchPayrollDetails,
        BranchDetails,
    };

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct PostBranchDetailsRequestForAdmin {
        pub client_ulid: Uuid,
        pub account: BranchAccountDetails,
        pub bank: BranchBankDetails,
        pub payroll: BranchPayrollDetails,
    }

    pub async fn post_one_branch(
        _: Token<AdminAccessToken>,
        ContentLengthLimit(Json(body)): ContentLengthLimit<
            Json<PostBranchDetailsRequestForAdmin>,
            FORM_DATA_LENGTH_LIMIT,
        >,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<String> {
        let database = database.lock().await;

        let ulid = database
            .insert_one_entity_client_branch(body.client_ulid)
            .await?;

        database
            .post_branch_account_details(
                ulid,
                body.account.branch_name,
                body.account.country,
                body.account.entity_type,
                body.account.registration_number,
                body.account.tax_id,
                body.account.statutory_contribution_submission_number,
                body.account.company_address,
                body.account.city,
                body.account.postal_code,
                body.account.time_zone,
                body.account.logo,
            )
            .await?;

        database
            .post_branch_bank_details(
                ulid,
                body.bank.currency,
                body.bank.bank_name,
                body.bank.bank_account_name,
                body.bank.bank_account_number,
                body.bank.swift_code,
                body.bank.bank_key,
                body.bank.iban,
                body.bank.bank_code,
                body.bank.branch_code,
            )
            .await?;

        database
            .post_branch_payroll_details(ulid, body.payroll.payment_date, body.payroll.cutoff_date)
            .await?;

        Ok(ulid.to_string())
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct GetManyBranchesQuery {
        pub client_ulid: Option<Uuid>,
        pub page: Option<u32>,
        pub per_page: Option<u32>,
    }

    pub async fn get_many_branches(
        _: Token<AdminAccessToken>,
        Query(query): Query<GetManyBranchesQuery>,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
        let database = database.lock().await;

        let result = database
            .select_many_entity_clients_branch_details(
                query.client_ulid,
                query.page,
                query.per_page,
            )
            .await?;

        Ok(Json(result))
    }

    pub async fn get_one_branch_by_ulid(
        _: Token<AdminAccessToken>,
        Path(branch_ulid): Path<Uuid>,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<Json<BranchDetails>> {
        let database = database.lock().await;

        let details = database
            .select_one_entity_clients_branch_details(branch_ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?;

        Ok(Json(details))
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct DeleteBranchRequestForAdmin {
        pub client_ulid: Uuid,
        pub branch_ulid: Uuid,
    }

    pub async fn delete_one_branch(
        _: Token<AdminAccessToken>,
        Query(query): Query<DeleteBranchRequestForAdmin>,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<()> {
        let database = database.lock().await;

        database
            .delete_one_branch(query.client_ulid, query.branch_ulid)
            .await?;

        Ok(())
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct GetManyBranchesIndividualContractorQuery {
        pub branch_ulid: Option<Uuid>,
        pub page: Option<u32>,
        pub per_page: Option<u32>,
    }

    pub async fn get_many_individual_contractors(
        _: Token<AdminAccessToken>,
        Query(query): Query<GetManyBranchesIndividualContractorQuery>,
        Extension(database): Extension<SharedDatabase>,
    ) -> GlobeliseResult<Json<Vec<BranchDetails>>> {
        let database = database.lock().await;

        let result = database
            .select_many_entity_client_branch_individual_contractors(
                query.branch_ulid,
                query.page,
                query.per_page,
            )
            .await?;

        Ok(Json(result))
    }
}

impl Database {
    pub async fn insert_one_entity_client_branch(
        &self,
        client_ulid: Uuid,
    ) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        let query = "
            INSERT INTO entity_client_branches (
                ulid, client_ulid
            ) VALUES (
                $1, $2
            )";

        sqlx::query(query)
            .bind(ulid)
            .bind(client_ulid)
            .execute(&self.0)
            .await?;

        Ok(ulid)
    }

    pub async fn client_owns_branch(
        &self,
        client_ulid: Uuid,
        branch_ulid: Uuid,
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
            .bind(branch_ulid)
            .bind(client_ulid)
            .fetch_optional(&self.0)
            .await?
            .is_some();

        Ok(result)
    }

    pub async fn delete_one_branch(
        &self,
        client_ulid: Uuid,
        branch_ulid: Uuid,
    ) -> GlobeliseResult<()> {
        let query = "
            DELETE FROM
                entity_client_branches 
            WHERE 
                ulid = $1";

        sqlx::query(query)
            .bind(branch_ulid)
            .bind(client_ulid)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_many_entity_clients_branch_details(
        &self,
        client_ulid: Option<Uuid>,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<BranchDetails>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let query = "
            SELECT
                -- branch details
                ulid, client_ulid,
                -- account details
                branch_name, country, entity_type, registration_number, tax_id, 
                statutory_contribution_submission_number, company_address, city, 
                postal_code, time_zone, logo,
                -- bank details
                currency, bank_name, bank_account_name, bank_account_number,
                swift_code, bank_key, iban, bank_code, branch_code,
                -- payment details
                cutoff_date, payment_date
            FROM
                entity_client_branch_details
            WHERE
                $1 IS NULL OR (client_ulid = $1)
            LIMIT $2 OFFSET $3";

        let result = sqlx::query_as(query)
            .bind(client_ulid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn select_many_entity_client_branch_individual_contractors(
        &self,
        branch_ulid: Option<Uuid>,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<BranchDetails>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let query = "
            SELECT
                *
            FROM
                entity_client_branch_department_individual_contractors_index
            WHERE
                ($1 IS NULL OR branch_ulid = $1)
            LIMIT 
                $2 
            OFFSET 
                $3";

        let result = sqlx::query_as(query)
            .bind(branch_ulid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn select_one_entity_clients_branch_details(
        &self,
        branch_ulid: Uuid,
    ) -> GlobeliseResult<Option<BranchDetails>> {
        let query = "
            SELECT
                -- branch details
                ulid, client_ulid
                -- account details
                branch_name, country, entity_type, registration_number, tax_id, 
                statutory_contribution_submission_number, company_address, city, 
                postal_code, time_zone, logo,
                -- bank details
                currency, bank_name, bank_account_name, bank_account_number,
                swift_code, bank_key, iban, bank_code, branch_code,
                -- payment details
                cutoff_date, payment_date
            FROM
                entity_client_branch_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(branch_ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
