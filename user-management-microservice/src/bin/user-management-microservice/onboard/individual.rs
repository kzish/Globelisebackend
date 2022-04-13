use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    pubsub::{SharedPubSub, UpdateUserName},
    token::Token,
};
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

use crate::database::SharedDatabase;

pub async fn client_account_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<IndividualClientDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let ulid = claims.payload.ulid;
    let full_name = format!("{} {}", request.first_name, request.last_name);

    let database = database.lock().await;
    database
        .onboard_individual_client_details(ulid, request)
        .await?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Client(ulid, full_name))
        .await?;

    Ok(())
}

pub async fn contractor_account_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<IndividualContractorDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let ulid = claims.payload.ulid;
    let full_name = format!(
        "{} {}",
        request.common_info.first_name, request.common_info.last_name
    );

    let database = database.lock().await;
    database
        .onboard_individual_contractor_details(ulid, request)
        .await?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Contractor(ulid, full_name))
        .await?;

    Ok(())
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientDetails {
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
pub struct IndividualContractorDetails {
    #[serde(flatten)]
    pub common_info: IndividualClientDetails,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub cv: Option<Vec<u8>>,
}
