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

use crate::contractor_account_settings::client_pic::payroll_information::ListClientContractorPayrollInformationRequest;
use crate::contractor_account_settings::client_pic::payroll_information::ListClientContractorPayrollInformationResponse;

pub async fn get_payroll_information_all(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListClientContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListClientContractorPayrollInformationResponse>>> {
    let database = database.lock().await;

    let response = database.get_payroll_information_all(request).await?;

    Ok(Json(response))
}

pub async fn get_payroll_information_individual(
    _: Token<AdminAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Query(mut request): Query<ListClientContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ListClientContractorPayrollInformationResponse>> {
    let database = database.lock().await;

    request.contractor_ulid = Some(contractor_ulid);
    
    let response = database.get_payroll_information_individual(request).await?;

    Ok(Json(response))
}

pub async fn post_payroll_information_individual(
    _: Token<AdminAccessToken>,
    Json(request): Json<ListClientContractorPayrollInformationResponse>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .post_payroll_information_individual(request)
        .await?;

    Ok(())
}

pub async fn get_payroll_information_entity(
    _: Token<AdminAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Query(mut request): Query<ListClientContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ListClientContractorPayrollInformationResponse>> {
    let database = database.lock().await;

    request.contractor_ulid = Some(contractor_ulid);

    let response = database.get_payroll_information_entity(request).await?;

    Ok(Json(response))
}

pub async fn post_payroll_information_entity(
    _: Token<AdminAccessToken>,
    Json(request): Json<ListClientContractorPayrollInformationResponse>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.post_payroll_information_entity(request).await?;

    Ok(())
}
