use axum::extract::{Extension, Json};
use common_utils::{
    custom_serde::{Currency, DateWrapper},
    error::GlobeliseResult,
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;

use crate::{auth::token::AccessToken, database::SharedDatabase};

pub async fn bank_details(
    claims: Token<AccessToken>,
    Json(details): Json<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .onboard_bank_details(claims.payload.ulid, claims.payload.user_type, details)
        .await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

pub async fn payment_details(
    claims: Token<AccessToken>,
    Json(details): Json<PaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .onboard_payment_details(claims.payload.ulid, claims.payload.user_type, details)
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
