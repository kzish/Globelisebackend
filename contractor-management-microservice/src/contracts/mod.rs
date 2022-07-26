use axum::{
    extract::{ContentLengthLimit, Extension, Path, Query},
    Json,
};
use common_utils::{
    custom_serde::{Currency, OffsetDateWrapper, UserRole, FORM_DATA_LENGTH_LIMIT},
    database::{contract::ContractsIndex, user::OnboardedUserIndex, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

mod database;

use common_utils::custom_serde::EmailWrapper;
use lettre::{Message, SmtpTransport, Transport};

use crate::{common::PaginatedQuery, database::SharedDatabase};

use crate::env::{GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsPreview {
    ulid: Uuid,
    contract_preview_text: String,
    client_ulid: Uuid,
    branch_ulid: Uuid,
    team_ulid: Option<Uuid>,
    contract_name: String,
    job_title: String,
    contract_type: String,
    seniority_level: String,
    job_scope: String,
    currency: String,
    contract_amount: f64,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    start_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    end_date: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsPreviewCreateRequest {
    contract_preview_text: Option<String>,
    client_ulid: Uuid,
    branch_ulid: Uuid,
    team_ulid: Option<Uuid>,
    contract_name: String,
    job_title: String,
    contract_type: String,
    seniority_level: String,
    job_scope: String,
    currency: String,
    contract_amount: f64,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    start_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    end_date: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractsPreviewDeleteRequest {
    ulid: Uuid,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerateContractFromPreviewRequest {
    preview_ulid: Uuid,
    contractor_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListContractsPreviewRequest {
    branch_ulid: Uuid,
    contract_name: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetOneContractQuery {
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostOneContract {
    client_ulid: Uuid,
    contractor_ulid: Uuid,
    branch_ulid: Option<Uuid>,
    contract_name: String,
    contract_status: String,
    contract_type: String,
    job_title: String,
    contract_amount: sqlx::types::Decimal,
    currency: Currency,
    seniority: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyContractsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
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
pub struct SignContractInviteRequest {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
    pub contract_ulid: Uuid,
    pub title: String,
    pub message: String,
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
pub struct ContractorViewContractsRequest {
    pub contractor_ulid: Uuid,
    pub contract_name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorEmailDetails {
    pub ulid: Uuid,
    pub name: String,
    pub email: EmailWrapper,
}

#[serde_as]
#[derive(Debug, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractPreviewRequest {
    pub preview_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ContractPreviewsRequest {
    pub branch_ulid: Uuid,
    pub job_title: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractDetails {
    pub contract_ulid: Uuid,
    pub client_ulid: Uuid,
    pub branch_ulid: Uuid,
    pub client_name: String,
    pub contractor_ulid: Uuid,
    pub contractor_name: String,
    pub contract_name: String,
    pub contract_type: String,
    pub contract_status: String,
    pub contract_amount: f64,
    pub currency: String,
    // pub begin_at: String,
    // pub end_at: String,
    pub job_title: String,
    pub seniority: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
}

pub async fn user_get_many_contract_index(
    claims: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndex>>> {
    let database = database.lock().await;
    let results = match role {
        UserRole::Client => {
            database
                .select_many_contracts(
                    query.page,
                    query.per_page,
                    query.query,
                    query.contractor_ulid,
                    Some(claims.payload.ulid),
                    query.branch_ulid,
                )
                .await?
        }
        UserRole::Contractor => {
            database
                .select_many_contracts(
                    query.page,
                    query.per_page,
                    query.query,
                    Some(claims.payload.ulid),
                    query.client_ulid,
                    query.branch_ulid,
                )
                .await?
        }
    };

    Ok(Json(results))
}

pub async fn user_delete_one_contract(
    claims: Token<UserAccessToken>,
    Path((_user_role, contract_ulid)): Path<(UserRole, Uuid)>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let contract = database
        .select_one_contract(
            Some(contract_ulid),
            query.contractor_ulid,
            Some(claims.payload.ulid),
            query.query,
            query.branch_ulid,
        )
        .await?;
    if Some(contract).is_none() {
        return Err(GlobeliseError::Forbidden);
    }
    database.user_delete_one_contract(contract_ulid).await?;

    Ok(())
}

pub async fn user_get_one_contract_index(
    claims: Token<UserAccessToken>,
    Path((user_role, contract_ulid)): Path<(UserRole, Uuid)>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<ContractsIndex>> {
    let database = database.lock().await;
    let result = match user_role {
        UserRole::Client => database
            .select_one_contract(
                Some(contract_ulid),
                query.contractor_ulid,
                Some(claims.payload.ulid),
                query.query,
                query.branch_ulid,
            )
            .await?
            .ok_or_else(|| GlobeliseError::not_found("Cannot find the contract with that query"))?,
        UserRole::Contractor => database
            .select_one_contract(
                Some(contract_ulid),
                Some(claims.payload.ulid),
                query.client_ulid,
                query.query,
                query.branch_ulid,
            )
            .await?
            .ok_or_else(|| GlobeliseError::not_found("Cannot find the contract with that query"))?,
    };

    Ok(Json(result))
}

pub async fn user_sign_one_contract(
    claims: Token<UserAccessToken>,
    Path((user_role, contract_ulid)): Path<(UserRole, Uuid)>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if user_role != UserRole::Contractor
        || !claims.payload.user_roles.contains(&UserRole::Contractor)
    {
        return Err(GlobeliseError::unauthorized(
            "Only contractors can sign a contract",
        ));
    }

    database
        .sign_one_contract(contract_ulid, claims.payload.ulid)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find the contract with that query"))?;

    Ok(())
}

pub async fn admin_get_many_contract_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetManyContractsQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsIndex>>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .select_many_contracts(
                query.per_page,
                query.per_page,
                query.query,
                query.contractor_ulid,
                query.client_ulid,
                query.branch_ulid,
            )
            .await?,
    ))
}

pub async fn admin_get_one_contract_index(
    _: Token<AdminAccessToken>,
    Path(contract_ulid): Path<Uuid>,
    Query(query): Query<GetOneContractQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<ContractsIndex>> {
    let database = database.lock().await;
    let result = database
        .select_one_contract(
            Some(contract_ulid),
            query.contractor_ulid,
            query.client_ulid,
            query.query,
            query.branch_ulid,
        )
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find the contract with that query"))?;
    Ok(Json(result))
}

pub async fn admin_post_one_contract(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostOneContract>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<String> {
    let database = database.lock().await;

    let contract_ulid = database
        .insert_one_contract(
            body.client_ulid,
            body.contractor_ulid,
            body.branch_ulid,
            &body.contract_name,
            &body.contract_status,
            &body.contract_type,
            &body.job_title,
            body.contract_amount,
            body.currency,
            &body.seniority,
            &body.begin_at,
            &body.end_at,
        )
        .await?;

    database
        .insert_one_client_contractor_pair(
            body.client_ulid,
            body.contractor_ulid,
            Some(contract_ulid),
        )
        .await?;

    Ok(contract_ulid.to_string())
}

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

//let contractor view his contracts
pub async fn contractor_view_contracts(
    claims: Token<UserAccessToken>,
    Json(request): Json<ContractorViewContractsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractDetails>>> {
    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    let response = database.contractor_view_contracts(request).await?;

    Ok(Json(response))
}

pub async fn client_create_contract_preview(
    claims: Token<UserAccessToken>,
    Json(request): Json<ContractsPreviewCreateRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database.client_create_contract_preview(request).await?;

    Ok(())
}

pub async fn client_get_contract_preview(
    claims: Token<UserAccessToken>,
    Query(request): Query<ContractPreviewRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<ContractsPreview>> {
    let database = database.lock().await;
    let response = database
        .client_get_contract_preview(request.preview_ulid)
        .await?;
    if claims.payload.ulid != response.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}

pub async fn client_get_contract_previews(
    _claims: Token<UserAccessToken>,
    Query(request): Query<ContractPreviewsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsPreview>>> {
    let database = database.lock().await;
    let response = database.client_get_contract_previews(request).await?;

    Ok(Json(response))
}

pub async fn client_update_contract_preview(
    claims: Token<UserAccessToken>,
    Json(request): Json<ContractsPreview>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    database.client_update_contract_preview(request).await?;

    Ok(())
}

pub async fn client_delete_contract_preview(
    claims: Token<UserAccessToken>,
    Json(request): Json<ContractsPreviewDeleteRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let contract_preview = database.client_get_contract_preview(request.ulid).await?;
    if claims.payload.ulid != contract_preview.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .client_delete_contract_preview(request.ulid)
        .await?;

    Ok(())
}

pub async fn client_list_contracts_preview(
    _claims: Token<UserAccessToken>,
    Json(request): Json<ListContractsPreviewRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractsPreview>>> {
    let database = database.lock().await;
    let response = database.client_list_contracts_preview(request).await?;

    Ok(Json(response))
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerateContractDto {
    ulid: Uuid,
    client_ulid: Uuid,
    contractor_ulid: Uuid,
    contract_name: String,
    contract_type: String,
    contract_status: String,
    contract_amount: f64,
    currency: String,
    job_title: String,
    seniority: String,
    branch_ulid: Uuid,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    begin_at: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    end_at: sqlx::types::time::OffsetDateTime,
    contract_preview_text: String,
    team_ulid: Option<Uuid>,
    job_scope: String,
}

pub async fn client_generate_contract_from_preview(
    claims: Token<UserAccessToken>,
    Json(request): Json<GenerateContractFromPreviewRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    let contract_preview = database
        .client_get_contract_preview(request.preview_ulid)
        .await?;

    let generated_contract = GenerateContractDto {
        ulid: Uuid::new_v4(),
        client_ulid: claims.payload.ulid,
        contractor_ulid: request.contractor_ulid,
        contract_name: contract_preview.contract_name,
        contract_type: contract_preview.contract_type,
        contract_status: "CREATED".to_string(),
        contract_amount: contract_preview.contract_amount,
        currency: contract_preview.currency,
        job_title: contract_preview.job_title,
        seniority: contract_preview.seniority_level,
        branch_ulid: contract_preview.branch_ulid,
        begin_at: contract_preview.start_date,
        end_at: contract_preview.end_date,
        contract_preview_text: contract_preview.contract_preview_text,
        team_ulid: contract_preview.team_ulid,
        job_scope: contract_preview.job_scope,
    };

    database
        .client_generate_contract_from_preview(generated_contract)
        .await?;

    Ok(())
}

//client invite contractor to sign contract
//send email to contractor
pub async fn client_invite_contractor(
    claims: Token<UserAccessToken>,
    Json(request): Json<SignContractInviteRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.client_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;
    let contractor_details = database
        .get_contractor_email_details(request.contractor_ulid)
        .await?;

    let contract_details = database.get_contract_details(request.contract_ulid).await?;

    let receiver_email = contractor_details
        .email
        .0
        // TODO: Get the name of the person associated to this email address
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
                <title>{}</title>
            </head>
            <body>
                DEAR {},
                {},<br>
                {} is requesting you to sign {} contract <br/>
                contract details are {:?}
            </body>
            </html>
            "##,
            request.title,
            contractor_details.name,
            request.message,
            contract_details.client_name,
            contract_details.job_title,
            contract_details
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
