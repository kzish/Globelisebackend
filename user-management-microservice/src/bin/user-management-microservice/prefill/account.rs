use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{EmailWrapper, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{prefill::account::PrefillIndividualContractorAccountDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use user_management_microservice_sdk::token::UserAccessToken;

pub async fn user_post_one_individual_contractor(
    token: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillIndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database
        .insert_one_client_prefill_individual_contractor_account_details(
            token.payload.ulid,
            body.email,
            body.first_name,
            body.last_name,
            body.dob,
            body.dial_code,
            body.phone_number,
            body.country,
            body.city,
            body.address,
            body.postal_code,
            body.tax_id,
            body.time_zone,
        )
        .await?;
    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserGetOnePrefillIndividualContractorDetailsQuery {
    email: EmailWrapper,
}

pub async fn user_get_one_individual_contractor(
    token: Token<UserAccessToken>,
    Query(query): Query<UserGetOnePrefillIndividualContractorDetailsQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<PrefillIndividualContractorAccountDetails>> {
    if !matches!(token.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;
    let result = database
        .select_one_client_prefill_individual_contractor_account_details(
            query.email,
            token.payload.ulid,
        )
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found(
                "Cannot find prefilled individual contractor account details from the query",
            )
        })?;

    Ok(Json(result))
}
