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
use eor_admin_microservice_sdk::token::AdminAccessToken;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::database::{Database, SharedDatabase};
use sqlx::types::Uuid;

use crate::contractor_account_settings::client_pic::employment_information::EmploymentInformation;
use crate::contractor_account_settings::client_pic::employment_information::ListClientContractorEmploymentInformationRequest;
use crate::contractor_account_settings::client_pic::employment_information::ListClientContractorEmploymentInformationResponse;

pub async fn get_employment_information_all(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListClientContractorEmploymentInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListClientContractorEmploymentInformationResponse>>> {
    let database = database.lock().await;

    let response = database.get_employment_information_all(request).await?;

    Ok(Json(response))
}

pub async fn get_employment_information_individual(
    _: Token<AdminAccessToken>,
    Path(uuid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EmploymentInformation>> {
    let database = database.lock().await;

    let response = database.get_employment_information_individual(uuid).await?;

    Ok(Json(response))
}

pub async fn post_employment_information_individual(
    _: Token<AdminAccessToken>,
    Json(request): Json<EmploymentInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .post_employment_information_individual(request)
        .await?;

    Ok(())
}

pub async fn get_employment_information_entity(
    _: Token<AdminAccessToken>,
    Path(uuid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EmploymentInformation>> {
    let database = database.lock().await;

    let response = database.get_employment_information_entity(uuid).await?;

    Ok(Json(response))
}

pub async fn post_employment_information_entity(
    _: Token<AdminAccessToken>,
    Json(request): Json<EmploymentInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.post_employment_information_entity(request).await?;

    Ok(())
}
