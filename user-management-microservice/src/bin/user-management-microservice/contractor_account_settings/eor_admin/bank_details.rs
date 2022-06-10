use axum::{
    extract::{Extension, Path},
    Json,
};
use common_utils::{error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use sqlx::types::Uuid;

use crate::contractor_account_settings::contractor::bank_details::{
    BankDetailsRequest, BankDetailsResponse,
};
use crate::database::SharedDatabase;

//methods
pub async fn get_bank_details_entity(
    _: Token<AdminAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetailsResponse>> {
    let database = database.lock().await;

    let response = database.get_bank_details_entity(contractor_ulid).await?;

    Ok(Json(response))
}

pub async fn post_bank_details_entity(
    _: Token<AdminAccessToken>,
    Json(request): Json<BankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.post_bank_details_entity(request).await?;

    Ok(())
}

pub async fn get_bank_details_individual(
    _: Token<AdminAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetailsResponse>> {
    let database = database.lock().await;

    let response = database
        .get_bank_details_individual(contractor_ulid)
        .await?;

    Ok(Json(response))
}

pub async fn post_bank_details_individual(
    _: Token<AdminAccessToken>,
    Json(request): Json<BankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.post_bank_details_individual(request).await?;

    Ok(())
}
