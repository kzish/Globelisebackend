use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    pubsub::{SharedPubSub, UpdateUserName},
    token::Token,
};
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};
use user_management_microservice_sdk::{
    token::AccessToken,
    user::{Role, UserType},
};

use crate::database::SharedDatabase;

pub async fn client_account_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let full_name = request.company_name.clone();

    let database = database.lock().await;
    database
        .onboard_entity_client_details(claims.payload.ulid, request)
        .await?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Client(claims.payload.ulid, full_name))
        .await?;

    Ok(())
}

pub async fn contractor_account_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityContractorDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let full_name = request.company_name.clone();

    let database = database.lock().await;
    database
        .onboard_entity_contractor_details(claims.payload.ulid, request)
        .await?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Contractor(claims.payload.ulid, full_name))
        .await?;

    Ok(())
}

pub async fn pic_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<Json<PicDetails>, FORM_DATA_LENGTH_LIMIT>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    database
        .onboard_pic_details(claims.payload.ulid, role, request)
        .await
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityDetails {
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    #[serde(default)]
    pub registration_number: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorDetails {
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    #[serde(default)]
    pub registration_number: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,

    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PicDetails {
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}
