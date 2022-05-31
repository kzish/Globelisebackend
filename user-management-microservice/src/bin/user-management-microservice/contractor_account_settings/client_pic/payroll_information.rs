use axum::{
    extract::{Extension, Path},
    Json,
};
use common_utils::{
    calc_limit_and_offset,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListClientContractorPayrollInformationRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Option<Uuid>,
    pub contractor_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListClientContractorPayrollInformationResponse {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
    pub monthly_basic_salary_amount: f64,
    pub monthly_added_pay_items_for_addition_section: f64,
    pub monthly_added_pay_items_for_deduction_section: f64,
    pub monthly_added_pay_items_for_statement_only_section: f64,
    pub monthly_added_pay_items_for_employers_contribution_section: f64,
}

impl<'r> FromRow<'r, PgRow> for ListClientContractorPayrollInformationResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            contractor_ulid: row.try_get("contractor_ulid")?,
            client_ulid: row.try_get("client_ulid")?,
            monthly_basic_salary_amount: row.try_get("monthly_basic_salary_amount")?,
            monthly_added_pay_items_for_addition_section: row
                .try_get("monthly_added_pay_items_for_addition_section")?,
            monthly_added_pay_items_for_deduction_section: row
                .try_get("monthly_added_pay_items_for_deduction_section")?,
            monthly_added_pay_items_for_statement_only_section: row
                .try_get("monthly_added_pay_items_for_statement_only_section")?,
            monthly_added_pay_items_for_employers_contribution_section: row
                .try_get("monthly_added_pay_items_for_employers_contribution_section")?,
        })
    }
}

pub async fn get_payroll_information_individual(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ListClientContractorPayrollInformationResponse>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let req = ListClientContractorPayrollInformationRequest {
        page: Some(1),
        per_page: Some(1),
        client_ulid: None,
        contractor_ulid: Some(contractor_ulid),
    };

    let response = database.get_payroll_information_individual(req).await?;

    Ok(Json(response))
}

pub async fn post_payroll_information_individual(
    claims: Token<UserAccessToken>,
    Json(request): Json<ListClientContractorPayrollInformationResponse>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .post_payroll_information_individual(request)
        .await?;

    Ok(())
}

pub async fn get_payroll_information_entity(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ListClientContractorPayrollInformationResponse>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let req = ListClientContractorPayrollInformationRequest {
        page: Some(1),
        per_page: Some(1),
        client_ulid: None,
        contractor_ulid: Some(contractor_ulid),
    };
    let response = database.get_payroll_information_entity(req).await?;

    Ok(Json(response))
}

pub async fn post_payroll_information_entity(
    claims: Token<UserAccessToken>,
    Json(request): Json<ListClientContractorPayrollInformationResponse>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(
            claims.payload.ulid,
            Some(request.contractor_ulid).unwrap_or_default(),
        )
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.post_payroll_information_entity(request).await?;

    Ok(())
}

impl Database {
    pub async fn get_payroll_information_all(
        &self,
        request: ListClientContractorPayrollInformationRequest,
    ) -> GlobeliseResult<Vec<ListClientContractorPayrollInformationResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                contractor_payroll_information 
            WHERE client_ulid = $3
            LIMIT $1  
            OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(request.client_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn get_payroll_information_individual(
        &self,
        request: ListClientContractorPayrollInformationRequest,
    ) -> GlobeliseResult<ListClientContractorPayrollInformationResponse> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                individual_contractor_payroll_information
             WHERE ($1 IS NULL OR client_ulid = $1)
            AND ($2 IS NULL OR contractor_ulid = $2)",
        )
        .bind(request.client_ulid)
        .bind(request.contractor_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_payroll_information_individual(
        &self,
        request: ListClientContractorPayrollInformationResponse,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                        individual_contractor_payroll_information
                (contractor_ulid, client_ulid, monthly_basic_salary_amount, monthly_added_pay_items_for_addition_section,
                monthly_added_pay_items_for_deduction_section, monthly_added_pay_items_for_statement_only_section, monthly_added_pay_items_for_employers_contribution_section)
                VALUES($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT(contractor_ulid, client_ulid) DO UPDATE SET 
                   monthly_basic_salary_amount = $3,
                   monthly_added_pay_items_for_addition_section = $4,
                   monthly_added_pay_items_for_deduction_section = $5,
                   monthly_added_pay_items_for_statement_only_section = $6,
                   monthly_added_pay_items_for_employers_contribution_section = $7"
        )
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .bind(request.monthly_basic_salary_amount)
        .bind(request.monthly_added_pay_items_for_addition_section)
        .bind(request.monthly_added_pay_items_for_deduction_section)
        .bind(request.monthly_added_pay_items_for_statement_only_section)
        .bind(request.monthly_added_pay_items_for_employers_contribution_section)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_payroll_information_entity(
        &self,
        request: ListClientContractorPayrollInformationRequest,
    ) -> GlobeliseResult<ListClientContractorPayrollInformationResponse> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                entity_contractor_payroll_information 
            WHERE ($1 IS NULL OR client_ulid = $1)
            AND ($2 IS NULL OR contractor_ulid = $2)",
        )
        .bind(request.client_ulid)
        .bind(request.contractor_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_payroll_information_entity(
        &self,
        request: ListClientContractorPayrollInformationResponse,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                        entity_contractor_payroll_information
                (contractor_ulid, client_ulid, monthly_basic_salary_amount, monthly_added_pay_items_for_addition_section,
                monthly_added_pay_items_for_deduction_section, monthly_added_pay_items_for_statement_only_section, monthly_added_pay_items_for_employers_contribution_section)
                VALUES($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT(contractor_ulid, client_ulid) DO UPDATE SET 
                   monthly_basic_salary_amount = $3,
                   monthly_added_pay_items_for_addition_section = $4,
                   monthly_added_pay_items_for_deduction_section = $5,
                   monthly_added_pay_items_for_statement_only_section = $6,
                   monthly_added_pay_items_for_employers_contribution_section = $7"
        )
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .bind(request.monthly_basic_salary_amount)
        .bind(request.monthly_added_pay_items_for_addition_section)
        .bind(request.monthly_added_pay_items_for_deduction_section)
        .bind(request.monthly_added_pay_items_for_statement_only_section)
        .bind(request.monthly_added_pay_items_for_employers_contribution_section)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
