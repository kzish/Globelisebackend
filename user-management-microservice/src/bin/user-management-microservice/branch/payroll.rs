use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{OffsetDateWrapper, UserType, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchPayrollDetails {
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

pub async fn user_post_branch_payroll_details(
    claims: Token<UserAccessToken>,
    Path(branch_ulid): Path<Uuid>,
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
        .post_branch_payroll_details(branch_ulid, body.payment_date, body.cutoff_date)
        .await
}

pub async fn user_get_branch_payroll_details(
    claims: Token<UserAccessToken>,
    Path(branch_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchPayrollDetails>> {
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

    let result = database
        .get_one_branch_payroll_details(branch_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(result))
}

pub async fn admin_post_branch_payroll_details(
    _: Token<AdminAccessToken>,
    Path(branch_ulid): Path<Uuid>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<BranchPayrollDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .post_branch_payroll_details(branch_ulid, body.payment_date, body.cutoff_date)
        .await
}

pub async fn admin_get_branch_payroll_details(
    _: Token<AdminAccessToken>,
    Path(branch_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchPayrollDetails>> {
    let database = database.lock().await;

    let result = database
        .get_one_branch_payroll_details(branch_ulid)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(result))
}

impl Database {
    pub async fn post_branch_payroll_details(
        &self,
        ulid: Uuid,
        payment_date: sqlx::types::time::OffsetDateTime,
        cutoff_date: sqlx::types::time::OffsetDateTime,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO entity_client_branch_payroll_details (
            ulid, cutoff_date, payment_date
        ) VALUES (
            $1, $2, $3
        ) ON CONFLICT(ulid) DO UPDATE SET 
            cutoff_date = $2, payment_date = $3
        ",
        )
        .bind(ulid)
        .bind(payment_date)
        .bind(cutoff_date)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_one_branch_payroll_details(
        &self,
        branch_ulid: Uuid,
    ) -> GlobeliseResult<Option<BranchPayrollDetails>> {
        let result = sqlx::query_as(
            "
        SELECT 
            ulid, cutoff_date, payment_date
        FROM
            entity_client_branch_payroll_details
        WHERE
            ulid = $1
        ",
        )
        .bind(branch_ulid)
        .fetch_optional(&self.0)
        .await?;

        Ok(result)
    }
}
