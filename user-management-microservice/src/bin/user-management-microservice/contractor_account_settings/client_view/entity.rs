use crate::{
    contractor_account_settings::contractor::entity::{
        EntityContractorAccountDetails, EntityContractorAccountDetailsRequest,
        EntityContractorBankDetails, EntityContractorBankDetailsRequest,
        EntityContractorEmployementInformation, EntityContractorEmployementInformationRequest,
        EntityContractorPayrollInformation, EntityContractorPayrollInformationRequest,
        EntityContractorPicDetails, EntityContractorPicDetailsRequest,
    },
    database::SharedDatabase,
};
use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::error::{GlobeliseError, GlobeliseResult};
use common_utils::token::Token;
use user_management_microservice_sdk::token::UserAccessToken;
//
//######### methods #########
//

//EntityContractorAccountDetails
pub async fn get_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorAccountDetails>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_entity_contractor_account_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//EntityContractorAccountDetails
pub async fn update_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorAccountDetails>,
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
        .update_entity_contractor_account_details(request)
        .await?;

    Ok(())
}
//EntityContractorAccountDetails
pub async fn delete_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorAccountDetailsRequest>,
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
        .delete_entity_contractor_account_details(request.ulid)
        .await?;

    Ok(())
}

//EntityContractorEmployementInformation
pub async fn get_entity_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorEmployementInformation>> {
    let database = database.lock().await;

    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_uuid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_entity_contractor_employment_information(request)
        .await?;

    Ok(Json(response))
}
//EntityContractorEmployementInformation
pub async fn update_entity_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorEmployementInformation>,
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
        .update_entity_contractor_employment_information(request)
        .await?;

    Ok(())
}
//EntityContractorEmployementInformation
pub async fn delete_entity_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorEmployementInformationRequest>,
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
        .delete_entity_contractor_employment_information(request)
        .await?;

    Ok(())
}

//EntityContractorPayrollInformation
pub async fn get_entity_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorPayrollInformation>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_entity_contractor_payroll_information(request)
        .await?;

    Ok(Json(response))
}
//EntityContractorPayrollInformation
pub async fn update_entity_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPayrollInformation>,
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
        .update_entity_contractor_payroll_information(request)
        .await?;

    Ok(())
}
//EntityContractorPayrollInformation
pub async fn delete_entity_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPayrollInformationRequest>,
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
        .delete_entity_contractor_payroll_information(request)
        .await?;

    Ok(())
}

//EntityContractorPicDetails
pub async fn get_entity_contractor_pic_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorPicDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorPicDetails>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_entity_contractor_pic_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//EntityContractorPicDetails
pub async fn update_entity_contractor_pic_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPicDetails>,
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
        .update_entity_contractor_pic_details(request)
        .await?;

    Ok(())
}
//EntityContractorPicDetails
pub async fn delete_entity_contractor_pic_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPicDetailsRequest>,
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
        .delete_entity_contractor_pic_details(request.ulid)
        .await?;

    Ok(())
}

//EntityContractorBankDetails
pub async fn get_entity_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorBankDetails>> {
    let database = database.lock().await;
    if !database
        .is_client_contractor_pair(claims.payload.ulid, request.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_entity_contractor_bank_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//EntityContractorBankDetails
pub async fn update_entity_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorBankDetails>,
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
        .update_entity_contractor_bank_details(request)
        .await?;

    Ok(())
}
//EntityContractorBankDetails
pub async fn delete_entity_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorBankDetailsRequest>,
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
        .delete_entity_contractor_bank_details(request.ulid)
        .await?;

    Ok(())
}
