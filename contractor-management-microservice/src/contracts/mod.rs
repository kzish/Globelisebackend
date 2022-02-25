use std::collections::HashMap;

use axum::{
    extract::{Extension, Query},
    Json,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};

use crate::{
    auth::token::AccessToken, database::SharedDatabase, error::Error,
    microservices::user_management::Role, state::SharedState,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndexResult {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub contract_count: i64,
    pub created_at: Option<String>,
    pub email: String,
}

// Respond to user clicking the reset password link in their email.
pub async fn user_index(
    AccessToken(access_token): AccessToken,
    Query(query): Query<HashMap<String, String>>,
    Extension(shared_state): Extension<SharedState>,
    Extension(shared_database): Extension<SharedDatabase>,
) -> Result<Json<Vec<UserIndexResult>>, Error> {
    let mut shared_state = shared_state.lock().await;
    let database = shared_database.lock().await;
    let users = shared_state
        // NOTE: Not sure if this is entirely safe.
        .index(
            query
                .into_iter()
                .chain(std::iter::once((format!("token"), access_token))),
        )
        .await?;
    let mut result = Vec::with_capacity(users.len());
    for v in users {
        let count = database.count_number_of_contracts(&v.ulid, &v.role).await?;
        result.push(UserIndexResult {
            ulid: v.ulid,
            name: v.name,
            role: v.role,
            contract_count: count,
            created_at: v.created_at,
            email: v.email,
        });
    }
    Ok(Json(result))
}
