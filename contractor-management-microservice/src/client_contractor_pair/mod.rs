use axum::{
    extract::{ContentLengthLimit, Extension, Query},
    Json,
};
use common_utils::{
    custom_serde::{UserRole, UserType, FORM_DATA_LENGTH_LIMIT},
    database::{
        client_contractor_pair::ClientContractorPairIndex, user::OnboardedUserIndex, CommonDatabase,
    },
    error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::common::PaginatedQuery;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyClientContractorPairIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub client_ulid: Option<Uuid>,
    pub contractor_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyOnboardedUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostOneClientContractorPair {
    client_ulid: Uuid,
    contractor_ulid: Uuid,
    contract_ulid: Option<Uuid>,
}

// Deprecate this in favor of just using onboarded user index in user management microservice
pub async fn admin_get_many_onboarded_user_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetManyOnboardedUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = shared_database.lock().await;
    let result = database
        .select_many_onboarded_user_index(
            query.page,
            query.per_page,
            query.query,
            query.user_type,
            query.user_role,
            None,
            None,
        )
        .await?;
    Ok(Json(result))
}

pub async fn admin_get_many_client_contractor_pair_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetManyClientContractorPairIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<ClientContractorPairIndex>>> {
    let database = shared_database.lock().await;
    let result = database
        .select_many_client_contractor_pair_index(
            query.page,
            query.per_page,
            query.query,
            query.client_ulid,
            query.contractor_ulid,
        )
        .await?;
    Ok(Json(result))
}

pub async fn admin_post_one_client_contractor_pair(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostOneClientContractorPair>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = shared_database.lock().await;

    database
        .insert_one_client_contractor_pair(
            body.client_ulid,
            body.contractor_ulid,
            body.contract_ulid,
        )
        .await?;

    Ok(())
}

pub async fn user_get_many_clients_for_contractors(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<ClientContractorPairIndex>>> {
    let database = database.lock().await;
    let result = database
        .select_many_client_contractor_pair_index(
            query.page,
            query.per_page,
            query.query,
            Some(access_token.payload.ulid),
            query.contractor_ulid,
        )
        .await?;
    Ok(Json(result))
}

pub async fn user_get_many_contractors_for_clients(
    access_token: Token<UserAccessToken>,
    Query(query): Query<PaginatedQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<ClientContractorPairIndex>>> {
    let database = database.lock().await;
    let result = database
        .select_many_client_contractor_pair_index(
            query.page,
            query.per_page,
            query.query,
            query.client_ulid,
            Some(access_token.payload.ulid),
        )
        .await?;
    Ok(Json(result))
}
