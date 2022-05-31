use axum::{
    extract::{Extension, Path},
    Json,
};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use sqlx::types::Uuid;
use user_management_microservice_sdk::token::UserAccessToken;

use crate::contractor_account_settings::contractor::personal_information::{
    EntityContractorProfileSettingsRequest, EntityContractorProfileSettingsResponse,
    IndividualContractorProfileSettingsRequest, IndividualContractorProfileSettingsResponse,
};
use crate::database::SharedDatabase;

pub async fn get_profile_settings_entity(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorProfileSettingsResponse>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let profile = database
        .get_profile_settings_entity(contractor_ulid)
        .await?;

    Ok(Json(profile))
}

pub async fn post_profile_settings_entity(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorProfileSettingsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.post_profile_settings_entity(request).await?;

    Ok(())
}

pub async fn get_profile_settings_individual(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorProfileSettingsResponse>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let profile = database
        .get_profile_settings_individual(contractor_ulid)
        .await?;

    Ok(Json(profile))
}

pub async fn post_profile_settings_individual(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorProfileSettingsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.post_profile_settings_individual(request).await?;

    Ok(())
}
