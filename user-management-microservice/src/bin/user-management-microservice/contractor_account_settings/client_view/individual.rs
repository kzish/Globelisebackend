use crate::{
    contractor_account_settings::contractor::individual::{
        IndividualContractorAccountDetails, IndividualContractorAccountDetailsRequest,
        IndividualContractorBankDetails, IndividualContractorBankDetailsRequest,
        IndividualContractorEmployementInformation,
        IndividualContractorEmployementInformationRequest, IndividualContractorPayrollInformation,
        IndividualContractorPayrollInformationRequest,
    },
    database::SharedDatabase,
};
use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use user_management_microservice_sdk::token::UserAccessToken;

//
//######### methods #########
//

//IndividualContractorAccountDetails
pub async fn get_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<IndividualContractorAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorAccountDetails>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_individual_contractor_account_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//IndividualContractorAccountDetails
pub async fn update_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_individual_contractor_account_details(request)
        .await?;

    Ok(())
}
//IndividualContractorAccountDetails
pub async fn delete_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .delete_individual_contractor_account_details(request.ulid)
        .await?;

    Ok(())
}

//IndividualContractorBankDetails
pub async fn get_individual_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<IndividualContractorBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorBankDetails>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_individual_contractor_bank_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//IndividualContractorBankDetails
pub async fn update_individual_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_individual_contractor_bank_details(request)
        .await?;

    Ok(())
}
//IndividualContractorBankDetails
pub async fn delete_individual_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .delete_individual_contractor_bank_details(request.ulid)
        .await?;

    Ok(())
}

//IndividualContractorEmployementInformation
pub async fn get_individual_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Query(request): Query<IndividualContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorEmployementInformation>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_uuid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_individual_contractor_employment_information(request)
        .await?;

    Ok(Json(response))
}
//IndividualContractorEmployementInformation
pub async fn update_individual_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorEmployementInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_uuid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_individual_contractor_employment_information(request)
        .await?;

    Ok(())
}
//IndividualContractorEmployementInformation
pub async fn delete_individual_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_uuid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .delete_individual_contractor_employment_information(request)
        .await?;

    Ok(())
}

//IndividualContractorPayrollInformation
pub async fn get_individual_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Query(request): Query<IndividualContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorPayrollInformation>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_individual_contractor_payroll_information(request)
        .await?;

    Ok(Json(response))
}
//IndividualContractorPayrollInformation
pub async fn update_individual_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorPayrollInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_individual_contractor_payroll_information(request)
        .await?;

    Ok(())
}
//IndividualContractorPayrollInformation
pub async fn delete_individual_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .delete_individual_contractor_payroll_information(request)
        .await?;

    Ok(())
}
