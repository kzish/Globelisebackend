//! this module performs the same functions as eor_admin/pay_items.rs from entity_client

use std::num::NonZeroU32;

use argon2::verify_encoded;
use axum::extract::{Extension, Json, Path, Query};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

pub async fn get_pay_items(
    claims: Token<UserAccessToken>,
    Query(request): Query<PayItemsIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<PayItem>>> {
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

    let pay_items = database.get_pay_items(request).await?;

    Ok(Json(pay_items))
}

pub async fn create_pay_item(
    claims: Token<UserAccessToken>,
    Json(pay_item): Json<CreatePayItem>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, pay_item.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database.create_pay_item(pay_item).await?;

    Ok(())
}

pub async fn update_pay_item(
    claims: Token<UserAccessToken>,
    Json(pay_item): Json<PayItem>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, pay_item.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database.update_pay_item(pay_item).await?;

    Ok(())
}

//requires password to delete
pub async fn delete_pay_item(
    claims: Token<UserAccessToken>,
    Path(pay_item_ulid): Path<Uuid>,
    Json(request): Json<DeletePayItemRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if let Some(pay_item) = database.get_pay_item_by_id(pay_item_ulid).await? {
        if !database
            .client_owns_branch(claims.payload.ulid, pay_item.branch_ulid)
            .await?
        {
            return Err(GlobeliseError::Forbidden);
        }

        let res_verify_password = database
            .verify_password(claims.payload.ulid, request.password)
            .await?;

        if !res_verify_password {
            return Err(GlobeliseError::unauthorized("Invalid password"));
        }

        database.delete_pay_item(pay_item_ulid).await?;

        Ok(())
    } else {
        Err(GlobeliseError::NotFound)
    }
}

pub async fn get_pay_item_by_id(
    claims: Token<UserAccessToken>,
    Path(pay_item_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PayItem>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if let Some(pay_item) = database.get_pay_item_by_id(pay_item_ulid).await? {
        if !database
            .client_owns_branch(claims.payload.ulid, pay_item.branch_ulid)
            .await?
        {
            return Err(GlobeliseError::Forbidden);
        }

        Ok(Json(pay_item))
    } else {
        Err(GlobeliseError::NotFound)
    }
}

impl Database {
    // verify_password_for_delete_operation
    pub async fn verify_password(&self, ulid: Uuid, password: String) -> GlobeliseResult<bool> {
        let query = "
            SELECT 
                password 
            FROM 
                users
            WHERE
                ulid = $1";
        let password_hash: String = sqlx::query(query)
            .bind(ulid)
            .map(|row| row.get("password"))
            .fetch_optional(&self.0)
            .await?
            .ok_or_else(|| GlobeliseError::not_found("Cannot find password for user"))?;

        let res = verify_encoded(&password_hash, password.as_bytes())?;

        Ok(res)
    }

    pub async fn create_pay_item(&self, pay_item: CreatePayItem) -> GlobeliseResult<Uuid> {
        let ulid = Uuid::new_v4();

        let query = "
            INSERT INTO entity_client_branch_pay_items (
                ulid, branch_ulid, pay_item_type, pay_item_custom_name, use_pay_item_type_name,
                pay_item_method, employers_contribution, require_employee_id
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8
            )";
        sqlx::query(query)
            .bind(ulid)
            .bind(pay_item.branch_ulid)
            .bind(pay_item.pay_item_type)
            .bind(pay_item.pay_item_custom_name)
            .bind(pay_item.use_pay_item_type_name)
            .bind(pay_item.pay_item_method)
            .bind(pay_item.employers_contribution)
            .bind(pay_item.require_employee_id)
            .execute(&self.0)
            .await?;

        Ok(ulid)
    }

    pub async fn update_pay_item(&self, pay_item: PayItem) -> GlobeliseResult<()> {
        let query = "
            UPDATE 
                entity_client_branch_pay_items 
            SET
                pay_item_type = $2,
                pay_item_custom_name = $3,
                use_pay_item_type_name = $4,
                pay_item_method = $5,
                employers_contribution = $6,
                require_employee_id = $7
            WHERE 
                ulid = $1
            ";
        sqlx::query(query)
            .bind(pay_item.ulid)
            .bind(pay_item.pay_item_type)
            .bind(pay_item.pay_item_custom_name)
            .bind(pay_item.use_pay_item_type_name)
            .bind(pay_item.pay_item_method)
            .bind(pay_item.employers_contribution)
            .bind(pay_item.require_employee_id)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn delete_pay_item(&self, ulid: Uuid) -> GlobeliseResult<()> {
        let query = "
            DELETE FROM 
                entity_client_branch_pay_items 
            WHERE
                ulid = $1
            ";
        sqlx::query(query).bind(ulid).execute(&self.0).await?;

        Ok(())
    }

    pub async fn get_pay_item_by_id(&self, ulid: Uuid) -> GlobeliseResult<Option<PayItem>> {
        let query = "
            SELECT 
                *
            FROM 
                entity_client_branch_pay_items 
            WHERE 
                ulid = $1";
        let pay_item = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(pay_item)
    }

    pub async fn get_pay_items(
        &self,
        request: PayItemsIndexQuery,
    ) -> GlobeliseResult<Vec<PayItem>> {
        let query = "
            SELECT 
                * 
            FROM 
                entity_client_branch_pay_items 
            WHERE 
                branch_ulid = $1 AND
                ($2 IS NULL OR pay_item_custom_name LIKE '%$2%')
            LIMIT $3 
            OFFSET $4";
        let pay_items = sqlx::query_as(query)
            .bind(request.branch_ulid)
            .bind(request.search_param)
            .bind(request.per_page.get())
            .bind((request.page.get() - 1) * request.per_page.get())
            .fetch_all(&self.0)
            .await?;

        Ok(pay_items)
    }
}

/// Request for logging a user in.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeletePayItemRequest {
    pub password: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PayItemsIndexQuery {
    pub branch_ulid: Uuid,
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub search_param: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PayItemType {
    Tax,
    StatutoryFund,
    Allowance,
    Incentive,
    Commission,
    Bonus,
    Claim,
    Others,
}

impl PayItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PayItemType::Tax => "tax",
            PayItemType::StatutoryFund => "statutory_fund",
            PayItemType::Allowance => "allowance",
            PayItemType::Incentive => "incentive",
            PayItemType::Commission => "commission",
            PayItemType::Bonus => "bonus",
            PayItemType::Claim => "claim",
            PayItemType::Others => "others",
        }
    }

    pub fn from_str(string: &str) -> Option<PayItemType> {
        match string {
            "tax" => Some(PayItemType::Tax),
            "statutory_fund" => Some(PayItemType::StatutoryFund),
            "allowance" => Some(PayItemType::Allowance),
            "incentive" => Some(PayItemType::Incentive),
            "commission" => Some(PayItemType::Commission),
            "bonus" => Some(PayItemType::Bonus),
            "claim" => Some(PayItemType::Claim),
            "others" => Some(PayItemType::Others),
            _ => None,
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for PayItemType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("text")
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for PayItemType {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let pay_item_type_str: &'_ str = sqlx::decode::Decode::decode(value)?;
        let pay_item_type = PayItemType::from_str(pay_item_type_str).ok_or(format!(
            "Cannot convert {} into a PayItemType",
            pay_item_type_str
        ))?;
        Ok(pay_item_type)
    }
}

impl sqlx::encode::Encode<'_, sqlx::Postgres> for PayItemType {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::encode(val, buf)
    }
    fn size_hint(&self) -> std::primitive::usize {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::size_hint(&val)
    }
}

//Rules:
//-When statement only is selected, this pay item will be reflected on payroll table and payroll report. but not included in total earning, total deductions and net pay. Also not reflected on payslip.
//-When employer's contribution is selected, this pay item will be reflected on payroll table, payroll report as well as payslip, but not included in total earning, total deductions and net pay
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PayItemMethod {
    Addition,
    Deduction,
    EmployersContribution,
    StatementOnly,
}

impl PayItemMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            PayItemMethod::Addition => "addition",
            PayItemMethod::Deduction => "deduction",
            PayItemMethod::EmployersContribution => "employers_contribution",
            PayItemMethod::StatementOnly => "statement_only",
        }
    }

    pub fn from_str(string: &str) -> Option<PayItemMethod> {
        match string {
            "addition" => Some(PayItemMethod::Addition),
            "deduction" => Some(PayItemMethod::Deduction),
            "employers_contribution" => Some(PayItemMethod::EmployersContribution),
            "statement_only" => Some(PayItemMethod::StatementOnly),
            _ => None,
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for PayItemMethod {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("text")
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for PayItemMethod {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let pay_item_type_str: &'_ str = sqlx::decode::Decode::decode(value)?;
        let pay_item_type = PayItemMethod::from_str(pay_item_type_str).ok_or(format!(
            "Cannot convert {} into a PayItemMethod",
            pay_item_type_str
        ))?;
        Ok(pay_item_type)
    }
}

impl sqlx::encode::Encode<'_, sqlx::Postgres> for PayItemMethod {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::encode(val, buf)
    }
    fn size_hint(&self) -> std::primitive::usize {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::size_hint(&val)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreatePayItem {
    pub branch_ulid: Uuid,
    pub pay_item_type: PayItemType,
    pub pay_item_custom_name: String,
    pub use_pay_item_type_name: bool,
    pub pay_item_method: PayItemMethod,
    pub employers_contribution: String,
    pub require_employee_id: bool,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PayItem {
    pub ulid: Uuid,
    pub branch_ulid: Uuid,
    pub pay_item_type: PayItemType,
    pub pay_item_custom_name: String,
    pub use_pay_item_type_name: bool,
    pub pay_item_method: PayItemMethod,
    pub employers_contribution: String,
    pub require_employee_id: bool,
}
