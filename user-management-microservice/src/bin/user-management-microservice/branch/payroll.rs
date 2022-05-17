use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{DateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchPayrollDetails {
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub cutoff_date: sqlx::types::time::Date,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchPaymentDetailsRequest {
    pub branch_ulid: Ulid,
}

pub async fn post_branch_payroll_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<BranchPayrollDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database
        .post_branch_payroll_details(claims.payload.ulid, body.payment_date, body.cutoff_date)
        .await
}

pub async fn get_branch_payroll_details(
    claims: Token<UserAccessToken>,
    Query(query): Query<BranchPaymentDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchPayrollDetails>> {
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

    Ok(Json(
        database
            .get_one_branch_payroll_details(query.branch_ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    ))
}

impl Database {
    pub async fn post_branch_payroll_details(
        &self,
        ulid: Ulid,
        payment_date: sqlx::types::time::Date,
        cutoff_date: sqlx::types::time::Date,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO entity_clients_branch_payroll_details (
            ulid, cutoff_date, payment_date
        ) VALUES (
            $1, $2, $3
        ) ON CONFLICT(ulid) DO UPDATE SET 
            cutoff_date = $2, payment_date = $3
        ",
        )
        .bind(ulid_to_sql_uuid(ulid))
        .bind(payment_date)
        .bind(cutoff_date)
        .execute(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_one_branch_payroll_details(
        &self,
        branch_ulid: Ulid,
    ) -> GlobeliseResult<Option<BranchPayrollDetails>> {
        let result = sqlx::query_as(
            "
        SELECT 
            ulid, cutoff_date, payment_date
        FROM
            entity_clients_branch_payroll_details
        WHERE
            ulid = $1
        ",
        )
        .bind(ulid_to_sql_uuid(branch_ulid))
        .fetch_optional(&self.0)
        .await
        .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
