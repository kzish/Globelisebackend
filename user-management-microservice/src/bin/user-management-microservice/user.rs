use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use common_utils::{
    custom_serde::{OptionOffsetDateWrapper, UserRole, UserType},
    database::{user::OnboardedUserIndex, CommonDatabase},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyOnboardedUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
    pub client_ulid: Option<Uuid>,
    pub contractor_ulid: Option<Uuid>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_after: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_before: Option<sqlx::types::time::OffsetDateTime>,
}

pub async fn user_get_many_users(
    claims: Token<UserAccessToken>,
    Path(user_role): Path<UserRole>,
    Query(query): Query<GetManyOnboardedUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    if !claims.payload.user_roles.contains(&user_role) {
        return Err(GlobeliseError::unauthorized(format!(
            "User is not part of {} role",
            user_role
        )));
    }

    let database = shared_database.lock().await;

    let result = match user_role {
        UserRole::Client => {
            database
                .select_many_contractors_index_for_clients(
                    Some(claims.payload.ulid),
                    query.page,
                    query.per_page,
                    query.query,
                    query.user_type,
                    query.user_role,
                    query.created_after,
                    query.created_before,
                )
                .await?
        }
        UserRole::Contractor => {
            database
                .select_many_clients_index_for_contractors(
                    Some(claims.payload.ulid),
                    query.page,
                    query.per_page,
                    query.query,
                    query.user_type,
                    query.user_role,
                    query.created_after,
                    query.created_before,
                )
                .await?
        }
    };

    Ok(Json(result))
}

pub async fn user_get_one_user(
    claims: Token<UserAccessToken>,
    Path((user_role, user_ulid)): Path<(UserRole, Uuid)>,
    Query(query): Query<GetManyOnboardedUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<OnboardedUserIndex>> {
    if !claims.payload.user_roles.contains(&user_role) {
        return Err(GlobeliseError::unauthorized(format!(
            "User is not part of {} role",
            user_role
        )));
    }

    let database = shared_database.lock().await;

    let result = match user_role {
        UserRole::Client => {
            database
                .select_one_contractors_index_for_clients(
                    Some(claims.payload.ulid),
                    Some(user_ulid),
                    query.query,
                    query.user_type,
                    query.user_role,
                    query.created_after,
                    query.created_before,
                )
                .await?
        }
        UserRole::Contractor => {
            database
                .select_one_clients_index_for_contractors(
                    Some(claims.payload.ulid),
                    Some(user_ulid),
                    query.query,
                    query.user_type,
                    query.user_role,
                    query.created_after,
                    query.created_before,
                )
                .await?
        }
    }
    .ok_or_else(|| GlobeliseError::not_found("Cannot find the user with that query"))?;

    Ok(Json(result))
}

pub async fn admin_get_many_users(
    _: Token<AdminAccessToken>,
    Path(user_role): Path<UserRole>,
    Query(query): Query<GetManyOnboardedUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = shared_database.lock().await;
    let result = match user_role {
        UserRole::Client => {
            database
                .select_many_contractors_index_for_clients(
                    query.client_ulid,
                    query.page,
                    query.per_page,
                    query.query,
                    query.user_type,
                    query.user_role,
                    query.created_after,
                    query.created_before,
                )
                .await?
        }
        UserRole::Contractor => {
            database
                .select_many_clients_index_for_contractors(
                    query.contractor_ulid,
                    query.page,
                    query.per_page,
                    query.query,
                    query.user_type,
                    query.user_role,
                    query.created_after,
                    query.created_before,
                )
                .await?
        }
    };
    Ok(Json(result))
}
