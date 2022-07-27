use std::sync::Arc;

use axum::{
    extract::{Extension, Query},
    Json,
};
use common_utils::{
    database::{user::OnboardedUserIndex, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
// use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use tokio::sync::Mutex;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

mod database;

use common_utils::custom_serde::EmailWrapper;
use common_utils::custom_serde::OptionOffsetDateWrapper;
use lettre::{Message, SmtpTransport, Transport};

use crate::{
    common::PaginatedQuery,
    database::{Database, SharedDatabase},
};

use crate::env::{GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteContractRequest {
    pub contract_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetContractsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
    pub contract_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SignContractRequest {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
    pub contract_ulid: Uuid,
    pub signature: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RevokeSignContractRequest {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
    pub contract_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SignContractInviteRequest {
    pub email: EmailWrapper,
    pub body: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsIndexResponse {
    pub ulid: Uuid,
    pub client_ulid: Uuid,
    pub contractor_ulid: Uuid,
    pub contract_name: Option<String>,
    pub contract_type: Option<String>,
    pub contract_status: Option<String>,
    pub currency: Option<String>,
    pub job_title: Option<String>,
    pub seniority: Option<String>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub begin_at: Option<sqlx::types::time::OffsetDateTime>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub end_at: Option<sqlx::types::time::OffsetDateTime>,
    pub branch_ulid: Option<Uuid>,
    pub client_signature: Option<String>,
    pub contractor_signature: Option<String>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub client_date_signed: Option<sqlx::types::time::OffsetDateTime>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub contractor_date_signed: Option<sqlx::types::time::OffsetDateTime>,
    pub team_ulid: Option<Uuid>,
    pub job_scope: Option<String>,
    pub contract_amount: f64,
    pub country_of_contractors_tax_residence: Option<String>,
    pub notice_period: Option<i32>,
    pub offer_stock_option: bool,
    pub special_clause: Option<String>,
    pub cut_off: Option<i32>,
    pub pay_day: Option<i32>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub due_date: Option<sqlx::types::time::OffsetDateTime>,
    pub contractor_name: Option<String>,
    pub client_name: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct SingleContractsIndexResponse {
    pub ulid: Uuid,
    pub client_ulid: Uuid,
    pub contractor_ulid: Uuid,
    pub contract_name: Option<String>,
    pub contract_type: Option<String>,
    pub contract_status: Option<String>,
    pub currency: Option<String>,
    pub job_title: Option<String>,
    pub seniority: Option<String>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub begin_at: Option<sqlx::types::time::OffsetDateTime>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub end_at: Option<sqlx::types::time::OffsetDateTime>,
    pub branch_ulid: Option<Uuid>,
    pub client_signature: Option<String>,
    pub contractor_signature: Option<String>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub client_date_signed: Option<sqlx::types::time::OffsetDateTime>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub contractor_date_signed: Option<sqlx::types::time::OffsetDateTime>,
    pub team_ulid: Option<Uuid>,
    pub job_scope: Option<String>,
    pub contract_amount: f64,
    pub country_of_contractors_tax_residence: Option<String>,
    pub notice_period: Option<i32>,
    pub offer_stock_option: bool,
    pub special_clause: Option<String>,
    pub cut_off: Option<i32>,
    pub pay_day: Option<i32>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub due_date: Option<sqlx::types::time::OffsetDateTime>,
    pub contractor_name: Option<String>,
    pub client_name: Option<String>,
    //:TODO Add claim items
    pub pay_items: Vec<ContractsPayItemsResponse>,
    pub additional_documents: Vec<ContractsAdditionalDocumentsResponse>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsRequest {
    pub ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub contractor_ulid: Option<Uuid>,
    pub contract_name: Option<String>,
    pub contract_type: Option<String>,
    pub contract_status: Option<String>,
    pub currency: Option<String>,
    pub job_title: Option<String>,
    pub seniority: Option<String>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub begin_at: Option<sqlx::types::time::OffsetDateTime>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub end_at: Option<sqlx::types::time::OffsetDateTime>,
    pub branch_ulid: Option<Uuid>,
    pub client_signature: Option<String>,
    pub contractor_signature: Option<String>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub client_date_signed: Option<sqlx::types::time::OffsetDateTime>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub contractor_date_signed: Option<sqlx::types::time::OffsetDateTime>,
    pub team_ulid: Option<Uuid>,
    pub job_scope: Option<String>,
    pub contract_amount: f64,
    pub country_of_contractors_tax_residence: Option<String>,
    pub notice_period: Option<i32>,
    pub offer_stock_option: bool,
    pub special_clause: Option<String>,
    pub cut_off: Option<i32>,
    pub pay_day: Option<i32>,
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub due_date: Option<sqlx::types::time::OffsetDateTime>,
    pub additional_documents: Vec<ContractsAdditionalDocumentsPostRequest>,
    pub claim_items: Vec<ContractsClaimItemsPostRequest>,
    pub pay_items: Vec<ContractsPayItemsPostRequest>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsAdditionalDocumentsPostRequest {
    pub file_name: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub file_data: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsAdditionalDocumentsResponse {
    pub ulid: Uuid,
    pub contract_ulid: Uuid,
    pub file_name: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub file_data: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsClaimItemsPostRequest {
    pub claim_item_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsPayItemsPostRequest {
    pub pay_item_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsPayItemsResponse {
    pub ulid: Uuid,
    pub branch_ulid: Uuid,
    pub pay_item_type: Option<String>,
    pub pay_item_custom_name: Option<String>,
    pub use_pay_item_type_name: Option<bool>,
    pub pay_item_method: Option<String>,
    pub employers_contribution: Option<String>,
    pub require_employee_id: Option<bool>,
}

pub async fn client_get_combine_single_contract_index(
    claims: Token<UserAccessToken>,
    Query(request): Query<GetContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<SingleContractsIndexResponse>> {
    let response =
        get_combine_single_contract_index(request.contract_ulid.unwrap(), database).await?;

    if claims.payload.ulid != response.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}

pub async fn contractor_get_combine_single_contract_index(
    claims: Token<UserAccessToken>,
    Query(request): Query<GetContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<SingleContractsIndexResponse>> {
    let response =
        get_combine_single_contract_index(request.contract_ulid.unwrap(), database).await?;

    if claims.payload.ulid != response.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}

pub async fn get_combine_single_contract_index(
    contract_ulid: Uuid,
    database: Arc<Mutex<Database>>,
) -> GlobeliseResult<SingleContractsIndexResponse> {
    let database = database.lock().await;

    let contract_index = database.get_single_contract_index(contract_ulid).await?;

    let contract_index_additional_documents = database
        .get_single_contract_index_additional_documents(contract_ulid)
        .await?;

    let contract_index_pay_items = database
        .get_single_contract_index_pay_items(contract_ulid)
        .await?;

    let combined_contract_index = SingleContractsIndexResponse {
        ulid: contract_index.ulid,
        client_ulid: contract_index.client_ulid,
        contractor_ulid: contract_index.contractor_ulid,
        contract_name: contract_index.contract_name,
        contract_type: contract_index.contract_type,
        contract_status: contract_index.contract_status,
        currency: contract_index.currency,
        job_title: contract_index.job_title,
        seniority: contract_index.seniority,
        begin_at: contract_index.begin_at,
        end_at: contract_index.end_at,
        branch_ulid: contract_index.branch_ulid,
        client_signature: contract_index.client_signature,
        contractor_signature: contract_index.contractor_signature,
        client_date_signed: contract_index.client_date_signed,
        contractor_date_signed: contract_index.contractor_date_signed,
        team_ulid: contract_index.team_ulid,
        job_scope: contract_index.job_scope,
        contract_amount: contract_index.contract_amount,
        country_of_contractors_tax_residence: contract_index.country_of_contractors_tax_residence,
        notice_period: contract_index.notice_period,
        offer_stock_option: contract_index.offer_stock_option,
        special_clause: contract_index.special_clause,
        cut_off: contract_index.cut_off,
        pay_day: contract_index.pay_day,
        due_date: contract_index.due_date,
        contractor_name: contract_index.contractor_name,
        client_name: contract_index.client_name,
        pay_items: contract_index_pay_items,
        additional_documents: contract_index_additional_documents,
    };

    Ok(combined_contract_index)
}

pub async fn client_list_contracts(
    claims: Token<UserAccessToken>,
    Query(mut request): Query<GetContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndexResponse>>> {
    let database = database.lock().await;
    request.client_ulid = Some(claims.payload.ulid);
    let response = database.client_list_contracts(request).await?;

    Ok(Json(response))
}

pub async fn contractor_list_contracts(
    claims: Token<UserAccessToken>,
    Query(mut request): Query<GetContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndexResponse>>> {
    let database = database.lock().await;
    request.contractor_ulid = Some(claims.payload.ulid);
    let response = database.contractor_list_contracts(request).await?;

    Ok(Json(response))
}

pub async fn client_post_update_contract(
    claims: Token<UserAccessToken>,
    Json(mut request): Json<ContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.client_ulid.unwrap() {
        return Err(GlobeliseError::Forbidden);
    }

    if request.ulid.is_none() {
        //becomes a new contract
        request.ulid = Some(Uuid::new_v4());
    }

    database.client_post_update_contract(request).await?;

    Ok(())
}

pub async fn client_delete_contract(
    claims: Token<UserAccessToken>,
    Json(request): Json<DeleteContractRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let contract = database.select_one_contract(request.contract_ulid).await?;

    if contract.client_ulid != claims.payload.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .client_delete_contract(request.contract_ulid)
        .await?;

    Ok(())
}

pub async fn client_sign_contract(
    claims: Token<UserAccessToken>,
    Json(request): Json<SignContractRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database.client_sign_contract(request).await?;
    Ok(())
}

pub async fn client_revoke_sign_contract(
    claims: Token<UserAccessToken>,
    Json(request): Json<RevokeSignContractRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database.client_revoke_sign_contract(request).await?;
    Ok(())
}

pub async fn contractor_sign_contract(
    claims: Token<UserAccessToken>,
    Json(request): Json<SignContractRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database.contractor_sign_contract(request).await?;
    Ok(())
}

pub async fn contractor_revoke_sign_contract(
    claims: Token<UserAccessToken>,
    Json(request): Json<RevokeSignContractRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database.contractor_revoke_sign_contract(request).await?;
    Ok(())
}

//send email to contractor
pub async fn client_invite_contractor(
    _claims: Token<UserAccessToken>,
    Json(request): Json<SignContractInviteRequest>,
    Extension(_database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let receiver_email = request
        .email
        .0
        .to_display("")
        .parse()
        .map_err(GlobeliseError::bad_request)?;

    let email = Message::builder()
        .from(GLOBELISE_SENDER_EMAIL.clone())
        .reply_to(GLOBELISE_SENDER_EMAIL.clone())
        .to(receiver_email)
        .subject("Contract Signing Request")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        // TODO: Once designer have a template for this. Use a templating library to populate data.
        .body(format!(
            r##"
            <!DOCTYPE html>
            <html>
            <head>
                <title></title>
            </head>
            <body>
                {}
            </body>
            </html>
            "##,
            request.body,
        ))
        .map_err(GlobeliseError::internal)?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)
        .map_err(GlobeliseError::internal)?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    mailer.send(&email).map_err(GlobeliseError::internal)?;

    Ok(())
}

//TODO: to depricate method
pub async fn user_get_many_clients_for_contractors(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = database.lock().await;
    let result = database
        .select_many_clients_index_for_contractors(
            Some(access_token.payload.ulid),
            query.page,
            query.per_page,
            query.query,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(Json(result))
}

//TODO: to depricate method
pub async fn user_get_many_contractors_for_clients(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = database.lock().await;
    let result = database
        .select_many_contractors_index_for_clients(
            Some(access_token.payload.ulid),
            query.page,
            query.per_page,
            query.query,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(Json(result))
}
