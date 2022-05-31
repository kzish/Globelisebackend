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

use crate::contractor_account_settings::contractor::bank_details::BankDetailsRequest;
use crate::contractor_account_settings::contractor::bank_details::BankDetailsResponse;

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
