use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{Currency, DateWrapper, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{Role, UserType},
};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardClientPaymentDetails {
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub cutoff_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub created_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub updated_at: sqlx::types::time::OffsetDateTime,
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
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub cutoff_date: sqlx::types::time::Date,
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
            body,
        )
        .await
}

impl Database {
    pub async fn insert_one_onboard_client_payment_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
        details: InsertOneOnboardClientPaymentDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = user_type.db_onboard_details_prefix(Role::Client) + "_payment_details";
        let query = format!(
            "
            INSERT INTO {target_table} (
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
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn select_one_onboard_client_payment_details(
        &self,
        ulid: Uuid,
        user_type: UserType,
    ) -> GlobeliseResult<Option<OnboardClientPaymentDetails>> {
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = user_type.db_onboard_details_prefix(Role::Client) + "_payment_details";
        let query = format!(
            "
            SELECT
                ulid, currency, payment_date, cutoff_date, updated_at,
                created_at
            FROM 
                {target_table}
            WHERE
                ulid = $1",
        );

        let result = sqlx::query_as(&query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}
