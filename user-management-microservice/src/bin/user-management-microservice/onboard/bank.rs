use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{UserRole, UserType, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorBankDetails {
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

pub async fn post_onboard_contractor_bank_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<ContractorBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_contractor_bank_details(claims.payload.ulid, claims.payload.user_type, body)
        .await
}

pub async fn get_onboard_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ContractorBankDetails>> {
    let database = database.lock().await;

    if !claims.payload.user_roles.contains(&UserRole::Contractor) {
        return Err(GlobeliseError::Forbidden);
    }

    let result = database
        .select_one_contractor_bank_detail(claims.payload.ulid, claims.payload.user_type)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(result))
}

impl Database {
    pub async fn insert_one_contractor_bank_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
        details: ContractorBankDetails,
    ) -> GlobeliseResult<()> {
        let table = match user_type {
            UserType::Individual => "individual_contractor_bank_details",
            UserType::Entity => "entity_contractor_bank_details",
        };

        sqlx::query(&format!(
            "
        INSERT INTO {table} (
            ulid, bank_name, bank_account_name, bank_account_number, bank_code,
            branch_code
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6
        ) ON CONFLICT(ulid) DO UPDATE SET 
            bank_name = $2, bank_account_name = $3, bank_account_number = $4, bank_code = $5,
            branch_code = $6",
        ))
        .bind(ulid)
        .bind(details.bank_name)
        .bind(details.bank_account_name)
        .bind(details.bank_account_number)
        .bind(details.bank_code)
        .bind(details.branch_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn select_one_contractor_bank_detail(
        &self,
        ulid: Uuid,
        user_type: UserType,
    ) -> GlobeliseResult<Option<ContractorBankDetails>> {
        let table = match user_type {
            UserType::Individual => "individual_contractor_bank_details",
            UserType::Entity => "entity_contractor_bank_details",
        };

        let result = sqlx::query_as(&format!(
            "
        SELECT
            ulid, bank_name, bank_account_name, bank_account_number, bank_code,
            branch_code
        FROM
            {table}
        WHERE
            ulid = $1",
        ))
        .bind(ulid)
        .fetch_optional(&self.0)
        .await?;

        Ok(result)
    }
}
