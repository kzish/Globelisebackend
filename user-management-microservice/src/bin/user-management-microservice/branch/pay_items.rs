//! this module performs the same functions as eor_admin/pay_items.rs from entity_client

use std::num::NonZeroU32;

use crate::database::{Database, SharedDatabase};
use argon2::verify_encoded;
use axum::extract::{Extension, Json, Path, Query};
use common_utils::custom_serde::DateWrapper;
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid, ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::TryFromInto;
use sqlx::{postgres::PgRow, FromRow, Row};
use unicode_normalization::UnicodeNormalization;
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

pub async fn get_pay_items(
    claims: Token<AccessToken>,
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
    claims: Token<AccessToken>,
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
    claims: Token<AccessToken>,
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
    claims: Token<AccessToken>,
    Path(pay_item_ulid): Path<Ulid>,
    Json(request): Json<DeletePayItemRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let _password: String = request.password.nfc().collect();

    let database = database.lock().await;

    let pay_item = database.get_pay_item_by_id(pay_item_ulid).await?;

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
}

pub async fn get_pay_item_by_id(
    claims: Token<AccessToken>,
    Path(pay_item_ulid): Path<Ulid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<PayItem>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let pay_item = database.get_pay_item_by_id(pay_item_ulid).await?;

    if !database
        .client_owns_branch(claims.payload.ulid, pay_item.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(pay_item))
}

impl Database {
    //verify_password_for_delete_operation
    pub async fn verify_password(&self, ulid: Ulid, password: String) -> GlobeliseResult<bool> {
        let query = "SELECT 
                    password 
                FROM 
                    auth_entities
                WHERE
                    ulid = $1";
        let password_hash: Option<String> = sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .map(|row| row.get("password"))
            .fetch_optional(&self.0)
            .await?;

        let res = verify_encoded(&password_hash.unwrap(), password.as_bytes()).unwrap();

        Ok(res)
    }

    pub async fn create_pay_item(&self, pay_item: CreatePayItem) -> GlobeliseResult<Ulid> {
        let ulid = Ulid::generate();

        let query = "
            INSERT INTO 
                entity_clients_branch_pay_items (
                ulid,
                branch_ulid,
                pay_item_type,
                pay_item_custom_name,
                use_pay_item_type_name,
                pay_item_method,
                employers_contribution,
                require_employee_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(ulid_to_sql_uuid(pay_item.branch_ulid))
            .bind(pay_item.pay_item_type.as_str())
            .bind(pay_item.pay_item_custom_name)
            .bind(pay_item.use_pay_item_type_name)
            .bind(pay_item.pay_item_method.as_str())
            .bind(pay_item.employers_contribution)
            .bind(pay_item.require_employee_id)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(ulid)
    }

    pub async fn update_pay_item(&self, pay_item: PayItem) -> GlobeliseResult<()> {
        let query = "
            UPDATE 
                entity_clients_branch_pay_items 
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
            .bind(ulid_to_sql_uuid(pay_item.ulid))
            .bind(pay_item.pay_item_type.as_str())
            .bind(pay_item.pay_item_custom_name)
            .bind(pay_item.use_pay_item_type_name)
            .bind(pay_item.pay_item_method.as_str())
            .bind(pay_item.employers_contribution)
            .bind(pay_item.require_employee_id)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_pay_item(&self, ulid: Ulid) -> GlobeliseResult<()> {
        let query = "
            DELETE FROM 
                entity_clients_branch_pay_items 
            WHERE
                ulid = $1
            ";
        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_pay_item_by_id(&self, ulid: Ulid) -> GlobeliseResult<PayItem> {
        let query = "
            SELECT * FROM 
                entity_clients_branch_pay_items 
            WHERE 
                ulid = $1";
        let pay_item = sqlx::query_as(query)
            .bind(ulid_to_sql_uuid(ulid))
            .fetch_one(&self.0)
            .await?;

        Ok(pay_item)
    }

    pub async fn get_pay_items(
        &self,
        request: PayItemsIndexQuery,
    ) -> GlobeliseResult<Vec<PayItem>> {
        let mut search_param = String::from("");
        if request.search_param != None {
            search_param = format!(
                "AND pay_item_custom_name LIKE '%{}%'",
                request.search_param.unwrap()
            );
        }
        let query = format!(
            "
            SELECT * FROM 
                entity_clients_branch_pay_items 
            WHERE 
                branch_ulid = $1
            {}
            LIMIT $2 
            OFFSET $3",
            search_param
        );
        let pay_items = sqlx::query_as(&query)
            .bind(ulid_to_sql_uuid(request.branch_ulid))
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PayItemsIndexQuery {
    pub branch_ulid: Ulid,
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

    pub fn fr_str(string: &str) -> Option<PayItemType> {
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

    pub fn fr_str(string: &str) -> Option<PayItemMethod> {
        match string {
            "addition" => Some(PayItemMethod::Addition),
            "deduction" => Some(PayItemMethod::Deduction),
            "employers_contribution" => Some(PayItemMethod::EmployersContribution),
            "statement_only" => Some(PayItemMethod::StatementOnly),
            _ => None,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreatePayItem {
    pub branch_ulid: Ulid,
    pub pay_item_type: PayItemType,
    pub pay_item_custom_name: String,
    pub use_pay_item_type_name: bool,
    pub pay_item_method: PayItemMethod,
    pub employers_contribution: String,
    pub require_employee_id: bool,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PayItem {
    pub ulid: Ulid,
    pub branch_ulid: Ulid,
    pub pay_item_type: PayItemType,
    pub pay_item_custom_name: String,
    pub use_pay_item_type_name: bool,
    pub pay_item_method: PayItemMethod,
    pub employers_contribution: String,
    pub require_employee_id: bool,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub created_at: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub updated_at: sqlx::types::time::Date,
}

impl<'r> FromRow<'r, PgRow> for PayItem {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let _pay_item_type: String = row.try_get("pay_item_type")?;
        let _pay_item_method: String = row.try_get("pay_item_method")?;

        let created_at_offset: sqlx::types::time::OffsetDateTime = row.try_get("created_at")?;
        let (ca_year, ca_month) = created_at_offset.iso_year_week();
        let ca_weekday = created_at_offset.weekday();
        let _created_at = sqlx::types::time::Date::try_from_iso_ywd(ca_year, ca_month, ca_weekday)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let updated_at_offset: sqlx::types::time::OffsetDateTime = row.try_get("updated_at")?;
        let (ua_year, ua_month) = updated_at_offset.iso_year_week();
        let ua_weekday = updated_at_offset.weekday();
        let _updated_at = sqlx::types::time::Date::try_from_iso_ywd(ua_year, ua_month, ua_weekday)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(PayItem {
            ulid: ulid_from_sql_uuid(row.try_get("ulid")?),
            branch_ulid: ulid_from_sql_uuid(row.try_get("branch_ulid")?),
            pay_item_type: PayItemType::fr_str(_pay_item_type.as_str()).unwrap(),
            pay_item_custom_name: row.try_get("pay_item_custom_name")?,
            use_pay_item_type_name: row.try_get("use_pay_item_type_name")?,
            pay_item_method: PayItemMethod::fr_str(_pay_item_method.as_str()).unwrap(),
            employers_contribution: row.try_get("employers_contribution")?,
            require_employee_id: row.try_get("require_employee_id")?,
            created_at: _created_at,
            updated_at: _updated_at,
        })
    }
}
