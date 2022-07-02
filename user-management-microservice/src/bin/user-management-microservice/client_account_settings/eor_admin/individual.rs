use crate::{
    client_account_settings::client::individual::{
        IndividualClientAccountDetails, IndividualClientAccountDetailsDeleteRequest,
        IndividualClientAccountDetailsRequest, IndividualClientPaymentDetails,
        IndividualClientPaymentDetailsDeleteRequest, IndividualClientPaymentDetailsRequest,
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

//IndividualClientAccountDetails
pub async fn get_individual_client_account_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<IndividualClientAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualClientAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_individual_client_account_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//IndividualClientAccountDetails
pub async fn update_individual_client_account_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<IndividualClientAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_individual_client_account_details(request)
        .await?;

    Ok(())
}
//IndividualClientAccountDetails
pub async fn delete_individual_client_account_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<IndividualClientAccountDetailsDeleteRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_individual_client_account_details(request)
        .await?;

    Ok(())
}

//IndividualClientPaymentDetails
pub async fn get_individual_client_payment_details(
    _: Token<AdminAccessToken>,
    Query(request): Query<IndividualClientPaymentDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualClientPaymentDetails>> {
    let database = database.lock().await;

    let response = database
        .get_individual_client_payment_details(request.ulid)
        .await?;

    Ok(Json(response))
}
//IndividualClientPaymentDetails
pub async fn update_individual_client_payment_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<IndividualClientPaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .update_individual_client_payment_details(request)
        .await?;

    Ok(())
}
//IndividualClientPaymentDetails
pub async fn delete_individual_client_payment_details(
    _: Token<AdminAccessToken>,
    Json(request): Json<IndividualClientPaymentDetailsDeleteRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_individual_client_payment_details(request)
        .await?;

    Ok(())
}
