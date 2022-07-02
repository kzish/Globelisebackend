use crate::{
    client_account_settings::client::entity::{
        EntityClientAccountDetails, EntityClientAccountDetailsRequest,
        EntityClientBranchAccountDetails, EntityClientBranchAccountDetailsRequest,
        EntityClientBranchBankDetails, EntityClientBranchBankDetailsRequest,
        EntityClientBranchPayrollDetails, EntityClientBranchPayrollDetailsRequest,
        EntityClientPaymentDetails, EntityClientPaymentDetailsRequest, EntityClientPicDetails,
        EntityClientPicDetailsRequest,
    },
    database::SharedDatabase,
};
use axum::{
    extract::{Extension, Query},
    Json,
};

use common_utils::{error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;

//
//######### methods #########
//

//EntityClientPicDetails
pub async fn get_entity_client_pic_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<EntityClientPicDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientPicDetails>> {
    let database = database.lock().await;

    let response = database.get_entity_client_pic_details(request.ulid).await?;

    Ok(Json(response))
}
//EntityClientPicDetails
pub async fn update_entity_client_pic_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientPicDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.update_entity_client_pic_details(request).await?;

    Ok(())
}
//EntityClientPicDetails
pub async fn delete_entity_client_pic_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientPicDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_entity_client_pic_details(request).await?;

    Ok(())
}

//EntityClientAccountDetails
pub async fn get_entity_client_account_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<EntityClientAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_account_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientAccountDetails
pub async fn update_entity_client_account_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_entity_client_account_details(request)
        .await?;

    Ok(())
}
//EntityClientAccountDetails
pub async fn delete_entity_client_account_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_client_account_details(request)
        .await?;

    Ok(())
}

//EntityClientBranchAccountDetails
pub async fn get_entity_client_branch_account_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<EntityClientBranchAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientBranchAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_branch_account_details(request.branch_ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientBranchAccountDetails
pub async fn update_entity_client_branch_account_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientBranchAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_entity_client_branch_account_details(request)
        .await?;

    Ok(())
}
//EntityClientBranchAccountDetails
pub async fn delete_entity_client_branch_account_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientBranchAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_client_branch_account_details(request)
        .await?;

    Ok(())
}

//EntityClientBranchBankDetails
pub async fn get_entity_client_branch_bank_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<EntityClientBranchBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientBranchBankDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_branch_bank_details(request.branch_ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientBranchBankDetails
pub async fn update_entity_client_branch_bank_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientBranchBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_entity_client_branch_bank_details(request)
        .await?;

    Ok(())
}
//EntityClientBranchBankDetails
pub async fn delete_entity_client_branch_bank_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientBranchBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_client_branch_bank_details(request)
        .await?;

    Ok(())
}

//EntityClientBranchPayrollDetails
pub async fn get_entity_client_branch_payroll_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<EntityClientBranchPayrollDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientBranchPayrollDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_branch_payroll_details(request.branch_ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientBranchPayrollDetails
pub async fn update_entity_client_branch_payroll_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientBranchPayrollDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_entity_client_branch_payroll_details(request)
        .await?;

    Ok(())
}
//EntityClientBranchPayrollDetails
pub async fn delete_entity_client_branch_payroll_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientBranchPayrollDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_client_branch_payroll_details(request)
        .await?;

    Ok(())
}

//EntityClientPaymentDetails
pub async fn get_entity_client_payment_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<EntityClientPaymentDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientPaymentDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_payment_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientPaymentDetails
pub async fn update_entity_client_payment_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientPaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_entity_client_payment_details(request)
        .await?;

    Ok(())
}
//EntityClientPaymentDetails
pub async fn delete_entity_client_payment_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<EntityClientPaymentDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_client_payment_details(request)
        .await?;

    Ok(())
}
