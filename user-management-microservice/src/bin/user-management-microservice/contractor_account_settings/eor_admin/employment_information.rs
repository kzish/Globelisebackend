use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use common_utils::{error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use sqlx::types::Uuid;

use crate::contractor_account_settings::client_pic::employment_information::{
    EmploymentInformation, ListClientContractorEmploymentInformationRequest,
    ListClientContractorEmploymentInformationResponse,
};
use crate::database::SharedDatabase;

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
