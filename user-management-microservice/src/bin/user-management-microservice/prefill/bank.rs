use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{EmailWrapper, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{onboard::bank::ContractorUserDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillUserBankDetails {
    pub email: EmailWrapper,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

pub async fn user_post_one_individual_contractor(
    token: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillUserBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    let ulid = database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find a user with this email"))?
        .ulid;

    if database
        .select_one_onboard_user_bank_detail(ulid, UserType::Individual)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_user_bank_details(
                ulid,
                UserType::Individual,
                &ContractorUserDetails {
                    bank_name: body.bank_name,
                    bank_account_name: body.bank_account_name,
                    bank_account_number: body.bank_account_number,
                    bank_code: body.bank_code,
                    branch_code: body.branch_code,
                },
            )
            .await?;
    }

    Ok(())
}

pub async fn admin_post_one_individual_contractor(
    // Only needed for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillUserBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let ulid = database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find a user with this email"))?
        .ulid;

    database
        .insert_one_onboard_user_bank_details(
            ulid,
            UserType::Individual,
            &ContractorUserDetails {
                bank_name: body.bank_name,
                bank_account_name: body.bank_account_name,
                bank_account_number: body.bank_account_number,
                bank_code: body.bank_code,
                branch_code: body.branch_code,
            },
        )
        .await?;

    Ok(())
}

pub async fn admin_post_one_entity_client(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillUserBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let ulid = database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find a user with this email"))?
        .ulid;

    if database
        .select_one_onboard_user_bank_detail(ulid, UserType::Individual)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_user_bank_details(
                ulid,
                UserType::Individual,
                &ContractorUserDetails {
                    bank_name: body.bank_name,
                    bank_account_name: body.bank_account_name,
                    bank_account_number: body.bank_account_number,
                    bank_code: body.bank_code,
                    branch_code: body.branch_code,
                },
            )
            .await?;
    }

    Ok(())
}
