use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{UserType, FORM_DATA_LENGTH_LIMIT},
    database::{
        onboard::entity::{EntityClientAccountDetails, EntityContractorAccountDetails},
        CommonDatabase,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use user_management_microservice_sdk::token::UserAccessToken;

pub async fn get_onboard_entity_client_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<EntityClientAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let result = database
        .select_one_onboard_entity_client_account_details(claims.payload.ulid)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found("Cannot find entity client account details for this user")
        })?;

    Ok(Json(result))
}

pub async fn post_onboard_entity_client_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<EntityClientAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database
        .insert_one_onboard_entity_client_account_details(claims.payload.ulid, body)
        .await?;

    Ok(())
}

pub async fn post_onboard_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<EntityContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    database
        .insert_one_onboard_entity_contractor_account_details(claims.payload.ulid, body)
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot insert entity client onboard data into the database because \n{:#?}",
                e
            ))
        })?;

    Ok(())
}

pub async fn get_onboard_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<EntityContractorAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_entity_contractor_account_details(claims.payload.ulid)
            .await?
            .ok_or_else(|| {
                GlobeliseError::not_found(
                    "Cannot find entity contractor account details for this user",
                )
            })?,
    ))
}
