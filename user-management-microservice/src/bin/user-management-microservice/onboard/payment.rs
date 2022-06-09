use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{Currency, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardClientPaymentDetails {
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

pub async fn get_onboard_client_payment_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<OnboardClientPaymentDetails>> {
    let database = database.lock().await;
    let result = database
        .select_one_onboard_client_payment_details(claims.payload.ulid, claims.payload.user_type)
        .await?
        .ok_or(GlobeliseError::NotFound)?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertOneOnboardClientPaymentDetails {
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

pub async fn post_onboard_client_payment_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<InsertOneOnboardClientPaymentDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .insert_one_onboard_client_payment_details(
            claims.payload.ulid,
            claims.payload.user_type,
            &body,
        )
        .await?;

    // ADDITIONAL SIDE-EFFECTS FROM SIGNING UP ENTITY CLIENT
    // Since this is the last step for the onboarding of entity clients
    let branch_ulid = database
        .insert_one_entity_client_branch(claims.payload.ulid)
        .await?;
    if let Some(entity_client_details) = database
        .get_onboard_entity_client_account_details(claims.payload.ulid)
        .await?
    {
        database
            .post_branch_account_details(
                branch_ulid,
                entity_client_details.company_name,
                entity_client_details.country,
                entity_client_details.entity_type,
                entity_client_details.registration_number,
                entity_client_details.tax_id,
                None,
                entity_client_details.company_address,
                entity_client_details.city,
                entity_client_details.postal_code,
                entity_client_details.time_zone,
                entity_client_details.logo,
            )
            .await?;
    }
    // Entity client does not have bank information?
    /*
    database
    .post_branch_bank_details(
        branch_ulid,
        branch_name,
        country,
        entity_type,
        registration_number,
        tax_id,
        statutory_contribution_submission_number,
        company_address,
        city,
        postal_code,
    )
    .await?;
    */
    database
        .post_branch_payroll_details(branch_ulid, body.payment_date, body.cutoff_date)
        .await?;

    Ok(())
}

impl Database {
    pub async fn insert_one_onboard_client_payment_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
        details: &InsertOneOnboardClientPaymentDetails,
    ) -> GlobeliseResult<()> {
        let table = match user_type {
            UserType::Individual => "individual_clients_payment_details",
            UserType::Entity => "entity_clients_payment_details",
        };

        let query = format!(
            "
            INSERT INTO {table} (
                ulid, currency, payment_date, cutoff_date
            ) VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT(ulid) DO UPDATE SET 
                currency = $2, payment_date = $3, cutoff_date = $4",
        );

        sqlx::query(&query)
            .bind(ulid)
            .bind(details.currency)
            .bind(details.payment_date)
            .bind(details.cutoff_date)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_onboard_client_payment_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
    ) -> GlobeliseResult<Option<OnboardClientPaymentDetails>> {
        let table = match user_type {
            UserType::Individual => "individual_clients_payment_details",
            UserType::Entity => "entity_clients_payment_details",
        };

        let query = format!(
            "
            SELECT
                ulid, currency, payment_date, cutoff_date
            FROM 
                {table}
            WHERE
                ulid = $1",
        );

        let result = sqlx::query_as(&query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
