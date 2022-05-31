use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use common_utils::custom_serde::OffsetDateWrapper;
use common_utils::{
    custom_serde::ImageData,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{Role, UserType},
};

use crate::database::{Database, SharedDatabase};
use sqlx::types::Uuid;

use crate::contractor_account_settings::contractor::personal_information::EntityContractorProfileSettingsRequest;
use crate::contractor_account_settings::contractor::personal_information::EntityContractorProfileSettingsResponse;
use crate::contractor_account_settings::contractor::personal_information::IndividualContractorProfileSettingsRequest;
use crate::contractor_account_settings::contractor::personal_information::IndividualContractorProfileSettingsResponse;

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
