use axum::extract::{Extension, Json};
use common_utils::{
    custom_serde::{Currency, DateWrapper, EmailWrapper},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{
    token::AccessToken,
    user::{Role, UserType},
};

use crate::database::{Database, SharedDatabase};

pub async fn onboard_client_payment_details(
    claims: Token<AccessToken>,
    Json(details): Json<PaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .onboard_client_payment_details(claims.payload.ulid, claims.payload.user_type, details)
        .await
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PaymentDetails {
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub cutoff_date: sqlx::types::time::Date,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefilledPaymentDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub payment_date: sqlx::types::time::Date,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub cutoff_date: sqlx::types::time::Date,
}

impl Database {
    pub async fn onboard_client_payment_details(
        &self,
        ulid: Ulid,
        user_type: UserType,
        details: PaymentDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(user_type)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = user_type.db_onboard_details_prefix(Role::Client) + "_payment_details";
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, currency, payment_date, cutoff_date)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(ulid) DO UPDATE SET 
            currency = $2, payment_date = $3, cutoff_date = $4",
        );
        sqlx::query(&query)
            .bind(ulid_to_sql_uuid(ulid))
            .bind(details.currency)
            .bind(details.payment_date)
            .bind(details.cutoff_date)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}
