use axum::extract::{Extension, Json};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{token::AccessToken, user::Role},
    database::SharedDatabase,
    error::Error,
};

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub dob: Option<String>,
    pub email: String,
}

pub async fn user_index(
    // NOTE: Only used to check that _some_ access token is provided
    _: AccessToken,
    Extension(database): Extension<SharedDatabase>,
) -> Result<Json<Vec<UserIndex>>, Error> {
    let database = database.lock().await;
    let result = database.user_index().await?;
    Ok(Json(result))
}
