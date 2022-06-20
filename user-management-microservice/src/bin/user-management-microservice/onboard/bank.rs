use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::FORM_DATA_LENGTH_LIMIT,
    database::{onboard::bank::ContractorBankDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use user_management_microservice_sdk::token::UserAccessToken;

pub async fn post_onboard_contractor_bank_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<ContractorBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_contractor_bank_details(claims.payload.ulid, claims.payload.user_type, body)
        .await?;

    Ok(())
}

pub async fn get_onboard_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<ContractorBankDetails>> {
    let database = database.lock().await;

    let result = database
        .select_one_contractor_bank_detail(claims.payload.ulid, claims.payload.user_type)
        .await?
        .ok_or(GlobeliseError::NotFound)?;

    Ok(Json(result))
}
