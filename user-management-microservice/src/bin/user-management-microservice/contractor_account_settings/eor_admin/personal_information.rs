use axum::{
    extract::{Extension, Path},
    Json,
};
use common_utils::{error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use sqlx::types::Uuid;

use crate::contractor_account_settings::contractor::personal_information::{
    EntityContractorProfileSettingsRequest, EntityContractorProfileSettingsResponse,
    IndividualContractorProfileSettingsRequest, IndividualContractorProfileSettingsResponse,
};
use crate::database::SharedDatabase;

pub async fn get_profile_settings_entity(
    _: Token<AdminAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorProfileSettingsResponse>> {
    let database = database.lock().await;

    let profile = database
        .get_profile_settings_entity(contractor_ulid)
        .await?;

    Ok(Json(profile))
}

pub async fn post_profile_settings_entity(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityContractorProfileSettingsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.post_profile_settings_entity(request).await?;

    Ok(())
}

pub async fn get_profile_settings_individual(
    _: Token<AdminAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorProfileSettingsResponse>> {
    let database = database.lock().await;

    let profile = database
        .get_profile_settings_individual(contractor_ulid)
        .await?;

    Ok(Json(profile))
}

pub async fn post_profile_settings_individual(
    _: Token<AdminAccessToken>,
    Json(request): Json<IndividualContractorProfileSettingsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.post_profile_settings_individual(request).await?;

    Ok(())
}
