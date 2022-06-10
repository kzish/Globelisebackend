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

use crate::contractor_account_settings::contractor::bank_details::{
    BankDetailsRequest, BankDetailsResponse,
};
use crate::database::SharedDatabase;

//methods
pub async fn get_bank_details_entity(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetailsResponse>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database.get_bank_details_entity(contractor_ulid).await?;

    Ok(Json(response))
}

pub async fn post_bank_details_entity(
    claims: Token<UserAccessToken>,
    Json(request): Json<BankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.post_bank_details_entity(request).await?;

    Ok(())
}

pub async fn get_bank_details_individual(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetailsResponse>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_bank_details_individual(contractor_ulid)
        .await?;

    Ok(Json(response))
}

pub async fn post_bank_details_individual(
    claims: Token<UserAccessToken>,
    Json(request): Json<BankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.post_bank_details_individual(request).await?;

    Ok(())
}
