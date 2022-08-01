use axum::{extract::Query, Extension, Json};
use common_utils::{custom_serde::EmailWrapper, error::GlobeliseResult, token::Token};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::SharedDatabase;

mod database;

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub client_ulid: Option<Uuid>,
    pub branch_ulid: Option<Uuid>,
    pub cost_center_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractorsResponse {
    pub ulid: Uuid,        //contractor ulid
    pub client_ulid: Uuid, //client ulid - client associated with this contractor
    pub name: String,      //contractor name
    pub email: Option<EmailWrapper>,
    pub user_role: Option<String>,
    pub user_type: Option<String>,
    pub contract_count: i64,
    pub branch_name: Option<String>,
    pub branch_ulid: Option<Uuid>,
    pub cost_center_name: Option<String>,
    pub cost_center_ulid: Option<Uuid>,
}

pub async fn client_get_contractors(
    claims: Token<UserAccessToken>,
    Query(mut request): Query<ContractorsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorsResponse>>> {
    let database = database.lock().await;
    request.client_ulid = Some(claims.payload.ulid); //ensure client ulid is always present in client request
    let response = database.client_get_contractors(request).await?;

    Ok(Json(response))
}

pub async fn eor_admin_get_contractors(
    _: Token<AdminAccessToken>,
    Query(request): Query<ContractorsQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ContractorsResponse>>> {
    let database = database.lock().await;
    let response = database.client_get_contractors(request).await?;

    Ok(Json(response))
}
