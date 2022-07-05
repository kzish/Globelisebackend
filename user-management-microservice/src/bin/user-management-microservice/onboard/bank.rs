use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{UserType, FORM_DATA_LENGTH_LIMIT},
    database::{onboard::bank::ContractorUserDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

pub async fn user_post_one_bank_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<ContractorUserDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_onboard_user_bank_details(claims.payload.ulid, claims.payload.user_type, &body)
        .await?;

    Ok(())
}

pub async fn admin_post_one_bank_details(
    _: Token<AdminAccessToken>,
    Path((user_ulid, user_type)): Path<(Uuid, UserType)>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<ContractorUserDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_onboard_user_bank_details(user_ulid, user_type, &body)
        .await?;

    Ok(())
}

pub async fn user_get_one_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<ContractorUserDetails>> {
    let database = database.lock().await;

    let result = database
        .select_one_onboard_user_bank_detail(claims.payload.ulid, claims.payload.user_type)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found("Cannot find contractor bank details for this user")
        })?;

    Ok(Json(result))
}

pub async fn admin_get_one_bank_details(
    _: Token<AdminAccessToken>,
    Path((user_ulid, user_type)): Path<(Uuid, UserType)>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<ContractorUserDetails>> {
    let database = database.lock().await;

    let result = database
        .select_one_onboard_user_bank_detail(user_ulid, user_type)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found("Cannot find contractor bank details for this user")
        })?;

    Ok(Json(result))
}
