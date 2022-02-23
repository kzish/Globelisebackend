use axum::extract::{Extension, Path};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        token::AccessToken,
        user::{Role, User},
    },
    database::SharedDatabase,
    error::Error,
};

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub email: String,
}

pub async fn user_index(
    // NOTE: Only used to check that _some_ access token is provided
    _claims: AccessToken,
    Extension(database): Extension<SharedDatabase>,
) -> Result<String, Error> {
    let database = database.lock().await;
    let result = database.user_index().await?;
    let result = serde_json::to_string(&result).unwrap();
    Ok(result)
}
