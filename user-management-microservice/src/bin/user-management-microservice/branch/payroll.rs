use axum::extract::{Extension, Json};
use common_utils::{
    custom_serde::DateWrapper,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchPaymentDetails {
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub cutoff_date: sqlx::types::time::Date,
}

pub async fn post_branch_payroll_details(
    claims: Token<AccessToken>,
    Json(details): Json<BranchPaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .post_branch_payroll_details(claims.payload.ulid, details)
        .await
}

impl Database {
    pub async fn post_branch_payroll_details(
        &self,
        ulid: Ulid,
        details: BranchPaymentDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        sqlx::query(
            "
        INSERT INTO entity_clients_payroll_details (
            ulid, cutoff_date, payment_date
        ) VALUES (
            $1, $2, $3
        ) ON CONFLICT(ulid) DO UPDATE SET 
            cutoff_date = $2, payment_date = $3
        ",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .bind(details.payment_date)
        .bind(details.cutoff_date)
        .execute(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_branch_payroll_details(
        &self,
        ulid: Ulid,
    ) -> GlobeliseResult<BranchPaymentDetails> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let result = sqlx::query_as(
            "
        SELECT 
            ulid, cutoff_date, payment_date
        FROM
            entity_clients_payroll_details
        WHERE
            ulid = $1
        ",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .fetch_one(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
