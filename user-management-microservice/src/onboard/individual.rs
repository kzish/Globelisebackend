use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};

use crate::{
    auth::{
        token::AccessToken,
        user::{Role, UserType},
    },
    database::SharedDatabase,
};

pub async fn account_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<IndividualDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }

    let ulid = claims.payload.ulid;
    let database = database.lock().await;
    database
        .onboard_individual_details(ulid, role, request)
        .await
}

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

    let (email, details) = request.split();
    let database = database.lock().await;
    database
        .prefill_onboard_individual_contractors(email, details)
        .await
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualDetails {
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
    pub fn split(self) -> (String, IndividualDetails) {
        (
            self.email,
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
        )
    }
}
