use axum::{extract::Path, Extension, Json};
use common_utils::{error::GlobeliseResult, token::Token};
use user_management_microservice_sdk::{token::AccessToken, user::Role};

use crate::database::SharedDatabase;

pub mod bank;
pub mod entity;
pub mod individual;
pub mod prefill;

pub async fn fully_onboarded(
    claims: Token<AccessToken>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<bool>> {
    let database = database.lock().await;
    Ok(Json(
        database
            .get_is_user_fully_onboarded(claims.payload.ulid, claims.payload.user_type, role)
            .await?,
    ))
}
