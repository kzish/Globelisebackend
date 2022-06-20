use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use common_utils::{
    custom_serde::{OptionOffsetDateWrapper, UserRole, UserType},
    database::{user::OnboardedUserIndex, CommonDatabase},
    error::GlobeliseResult,
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use user_management_microservice_sdk::token::UserAccessToken;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyOnboardedUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_after: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_before: Option<sqlx::types::time::OffsetDateTime>,
}

/// Lists all the users plus some information about them.
pub async fn user_get_many_users(
    claims: Token<UserAccessToken>,
    Path(user_role): Path<UserRole>,
    Query(query): Query<GetManyOnboardedUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
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
