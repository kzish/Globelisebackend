use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{EmailWrapper, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{prefill::bank::PrefillIndividualContractorBankDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;

pub async fn user_post_one_individual_contractor(
    token: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillIndividualContractorBankDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    database
        .insert_one_client_prefill_individual_contractor_bank_details(
            token.payload.ulid,
            body.email,
            body.bank_name,
            body.bank_account_name,
            body.bank_account_number,
            body.bank_code,
            body.branch_code,
        )
        .await?;

    Ok(())
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserGetOnePrefillIndividualContractorBankDetailsQuery {
    email: EmailWrapper,
}

pub async fn user_get_one_individual_contractor(
    token: Token<UserAccessToken>,
    Query(query): Query<UserGetOnePrefillIndividualContractorBankDetailsQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorBankDetails>> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    let result = database
        .select_one_client_prefill_individual_contractor_bank_details(
            token.payload.ulid,
            query.email,
        )
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found(
                "Cannot find prefilled individual contractor bank details from the query",
            )
        })?;

    Ok(Json(result))
}
