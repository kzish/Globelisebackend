use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};

use crate::database::SharedDatabase;

use super::{bank::BankDetails, individual::IndividualDetails};

pub async fn prefill_individual_contractor_account_details(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<PrefillIndividualDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !EmailAddress::is_valid(&request.email) {
        return Err(GlobeliseError::BadRequest(
            "Please provide a valid email address",
        ));
    }

    let (email, details) = request.split()?;
    let database = database.lock().await;
    database
        .prefill_onboard_individual_contractors_account_details(&email, details)
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
    if !EmailAddress::is_valid(&request.email) {
        return Err(GlobeliseError::BadRequest(
            "Please provide a valid email address",
        ));
    }

    let (email, details) = request.split()?;
    let database = database.lock().await;
    database
        .prefill_onboard_individual_contractors_bank_details(&email, details)
        .await?;

    Ok(())
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualDetails {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}

impl PrefillIndividualDetails {
    pub fn split(self) -> GlobeliseResult<(EmailAddress, IndividualDetails)> {
        Ok((
            self.email.parse::<EmailAddress>()?,
            IndividualDetails {
                first_name: self.first_name,
                last_name: self.last_name,
                dob: self.dob,
                dial_code: self.dial_code,
                phone_number: self.phone_number,
                country: self.country,
                city: self.city,
                address: self.address,
                postal_code: self.postal_code,
                tax_id: self.tax_id,
                time_zone: self.time_zone,
                profile_picture: self.profile_picture,
            },
        ))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillBankDetails {
    pub email: String,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

impl PrefillBankDetails {
    pub fn split(self) -> GlobeliseResult<(EmailAddress, BankDetails)> {
        Ok((
            self.email.parse::<EmailAddress>()?,
            BankDetails {
                bank_name: self.bank_name,
                account_name: self.account_name,
                account_number: self.account_number,
            },
        ))
    }
}
