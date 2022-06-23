use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{UserType, FORM_DATA_LENGTH_LIMIT},
    database::{
        onboard::individual::{IndividualClientAccountDetails, IndividualContractorAccountDetails},
        CommonDatabase,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use user_management_microservice_sdk::token::UserAccessToken;

pub async fn post_onboard_individual_client_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<IndividualClientAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let ulid = claims.payload.ulid;

    let database = database.lock().await;
    database
        .insert_one_onboard_individual_client_account_details(ulid, body)
        .await?;

    Ok(())
}

pub async fn get_onboard_individual_client_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<IndividualClientAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    let result = database
        .select_one_onboard_individual_client_account_details(claims.payload.ulid)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found("Cannot find individual client account details for this user")
        })?;

    Ok(Json(result))
}

pub async fn post_onboard_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<IndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let ulid = claims.payload.ulid;

    let database = database.lock().await;
    database
        .insert_one_onboard_individual_contractor_account_details(ulid, body)
        .await?;

    Ok(())
}

pub async fn get_onboard_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<IndividualContractorAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    let result = database
        .select_one_onboard_individual_contractor_account_details(claims.payload.ulid)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found(
                "Cannot find individual contractor account details for this user",
            )
        })?;

    Ok(Json(result))
}
