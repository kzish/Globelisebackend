use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{UserType, FORM_DATA_LENGTH_LIMIT},
    database::{onboard::payment::OnboardClientPaymentDetails, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::SharedDatabase;

pub async fn user_get_one_payment_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<OnboardClientPaymentDetails>> {
    let database = database.lock().await;

    let result = database
        .select_one_onboard_client_payment_details(claims.payload.ulid, claims.payload.user_type)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found("Cannot find client payment details for this user")
        })?;

    Ok(Json(result))
}

pub async fn admin_get_one_payment_details(
    _: Token<AdminAccessToken>,
    Path((user_ulid, user_type)): Path<(Uuid, UserType)>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<OnboardClientPaymentDetails>> {
    let database = database.lock().await;

    let result = database
        .select_one_onboard_client_payment_details(user_ulid, user_type)
        .await?
        .ok_or_else(|| {
            GlobeliseError::not_found("Cannot find client payment details for this user")
        })?;

    Ok(Json(result))
}

pub async fn user_post_one_payment_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<OnboardClientPaymentDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(common_database): Extension<CommonDatabase>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let common_database = common_database.lock().await;
    let shared_database = shared_database.lock().await;

    common_database
        .insert_one_onboard_client_payment_details(
            claims.payload.ulid,
            claims.payload.user_type,
            body.currency,
            &body.payment_date,
            &body.cutoff_date,
        )
        .await?;

    // ADDITIONAL SIDE-EFFECTS FROM SIGNING UP ENTITY CLIENT
    // Since this is the last step for the onboarding of entity clients
    if claims.payload.user_type == UserType::Entity
        && shared_database
            .select_one_entity_clients_branch_details(None, Some(claims.payload.ulid))
            .await?
            .is_none()
    {
        let branch_ulid = shared_database
            .insert_one_entity_client_branch(claims.payload.ulid)
            .await?;
        if let Some(entity_client_details) = common_database
            .select_one_onboard_entity_client_account_details(claims.payload.ulid)
            .await?
        {
            shared_database
                .post_branch_account_details(
                    branch_ulid,
                    entity_client_details.company_name,
                    entity_client_details.country,
                    entity_client_details.entity_type,
                    entity_client_details.registration_number,
                    entity_client_details.tax_id,
                    None,
                    entity_client_details.company_address,
                    entity_client_details.city,
                    entity_client_details.postal_code,
                    entity_client_details.time_zone,
                    entity_client_details.logo,
                )
                .await?;
        }
        // Entity client does not have bank information?
        /*
        database
        .post_branch_bank_details(
            branch_ulid,
            branch_name,
            country,
            entity_type,
            registration_number,
            tax_id,
            statutory_contribution_submission_number,
            company_address,
            city,
            postal_code,
        )
        .await?;
        */
        shared_database
            .post_branch_payroll_details(branch_ulid, body.payment_date, body.cutoff_date)
            .await?;
    }
    Ok(())
}

pub async fn admin_post_one_payment_details(
    _: Token<AdminAccessToken>,
    Path((user_ulid, user_type)): Path<(Uuid, UserType)>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<OnboardClientPaymentDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(common_database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let common_database = common_database.lock().await;

    common_database
        .insert_one_onboard_client_payment_details(
            user_ulid,
            user_type,
            body.currency,
            &body.payment_date,
            &body.cutoff_date,
        )
        .await?;

    Ok(())
}
