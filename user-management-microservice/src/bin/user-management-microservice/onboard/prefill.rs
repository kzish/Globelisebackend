use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{DateWrapper, EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::Deserialize;
use serde_with::{serde_as, TryFromInto};
use uuid::Uuid;

use crate::database::SharedDatabase;
use crate::onboard::{
    entity::{PrefillEntityClientDetails, PrefilledPicDetails},
    payment::PrefilledPaymentDetails,
};

pub async fn prefill_individual_contractor_account_details(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(prefill_individual_details)): ContentLengthLimit<
        Json<PrefillIndividualDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .prefill_onboard_individual_contractors_account_details(prefill_individual_details)
        .await?;

    Ok(())
}

pub async fn prefill_individual_contractor_bank_details(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PrefillBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .prefill_onboard_individual_contractors_bank_details(request)
        .await?;

    Ok(())
}

pub async fn prefill_entity_client_account_details(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PrefillEntityClientDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .prefill_onboard_entity_client_details(request)
        .await?;

    Ok(())
}

pub async fn prefill_entity_client_bank_details(
    _: Token<AdminAccessToken>,
    Json(details): Json<PrefillBankDetailsNoULID>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .prefill_onboard_entity_clients_bank_details(details)
        .await
}

pub async fn prefill_entity_client_payment_details(
    _: Token<AdminAccessToken>,
    Json(details): Json<PrefilledPaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .prefill_onboard_entity_client_payment_details(details)
        .await
}

pub async fn prefill_entity_client_pic_details(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PrefilledPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database
        .prefill_onboard_entity_client_pic_details(request)
        .await
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillBankDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub client_ulid: Uuid,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillBankDetailsNoULID {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}