use axum::extract::{Extension, Json};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

pub async fn post_onboard_contractor_bank_details(
    claims: Token<AccessToken>,
    Json(details): Json<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if database
        .user(claims.payload.ulid, Some(claims.payload.user_type))
        .await?
        .is_none()
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .onboard_contractor_bank_details(claims.payload.ulid, claims.payload.user_type, details)
        .await
}

pub async fn get_onboard_contractor_bank_details(
    claims: Token<AccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetails>> {
    let database = database.lock().await;

    if database
        .user(claims.payload.ulid, Some(claims.payload.user_type))
        .await?
        .is_none()
    {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(
        database
            .get_onboard_contractor_bank_details(claims.payload.ulid, claims.payload.user_type)
            .await?,
    ))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

impl FromRow<'_, PgRow> for BankDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(BankDetails {
            bank_name: row.try_get("bank_name")?,
            account_name: row.try_get("bank_account_name")?,
            account_number: row.try_get("bank_account_number")?,
        })
    }
}

impl Database {
    pub async fn onboard_contractor_bank_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        details: BankDetails,
    ) -> GlobeliseResult<()> {
        match user_type {
            UserType::Individual => {
                sqlx::query(
                    "
        INSERT INTO individual_contractors_bank_details
        (ulid, bank_name, bank_account_name, bank_account_number)
        VALUES ($4, $1, $2, $3)
        ON CONFLICT(ulid) DO UPDATE SET 
        bank_name = $1, bank_account_name = $2, bank_account_number = $3",
                )
                .bind(details.bank_name)
                .bind(details.account_name)
                .bind(details.account_number)
                .bind(ulid_to_sql_uuid(ulid))
                .execute(&self.0)
                .await
                .map_err(|e| GlobeliseError::Database(e.to_string()))?;
            }
            UserType::Entity => {
                sqlx::query(
                    "
        INSERT INTO entity_contractors_bank_details
        (ulid, bank_name, bank_account_name, bank_account_number)
        VALUES ($4, $1, $2, $3)
        ON CONFLICT(ulid) DO UPDATE SET 
        bank_name = $1, bank_account_name = $2, bank_account_number = $3",
                )
                .bind(details.bank_name)
                .bind(details.account_name)
                .bind(details.account_number)
                .bind(ulid_to_sql_uuid(ulid))
                .execute(&self.0)
                .await
                .map_err(|e| GlobeliseError::Database(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub async fn get_onboard_contractor_bank_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
    ) -> GlobeliseResult<BankDetails> {
        match user_type {
            UserType::Individual => {
                let result = sqlx::query_as(
                    "
                SELECT
                    ulid, bank_name, bank_account_name, bank_account_number
                FROM
                    individual_contractors_bank_details
                WHERE
                    ulid = $1",
                )
                .bind(ulid_to_sql_uuid(ulid))
                .fetch_one(&self.0)
                .await
                .map_err(|e| GlobeliseError::Database(e.to_string()))?;
                Ok(result)
            }
            UserType::Entity => {
                let result = sqlx::query_as(
                    "
                SELECT
                    ulid, bank_name, bank_account_name, bank_account_number
                FROM
                    entity_contractors_bank_details
                WHERE
                    ulid = $1",
                )
                .bind(ulid_to_sql_uuid(ulid))
                .fetch_one(&self.0)
                .await
                .map_err(|e| GlobeliseError::Database(e.to_string()))?;
                Ok(result)
            }
        }
    }
}
