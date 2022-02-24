use std::{collections::HashMap, str::FromStr};

use axum::extract::{Extension, Json, Query};
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
    pub created_at: Option<String>,
    pub email: String,
}

pub async fn user_index(
    // NOTE: Only used to check that _some_ access token is provided
    _: AccessToken,
    Query(query): Query<HashMap<String, String>>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<Json<Vec<UserIndex>>, Error> {
    let page = query
        .get("page")
        .map(|v| {
            v.parse()
                .map_err(|_| Error::BadRequest("Cannot parse page param as i32"))
        })
        .unwrap_or_else(|| Ok(0))?;
    let per_page = query
        .get("per_page")
        .map(|v| {
            v.parse()
                .map_err(|_| Error::BadRequest("Cannot parse page param as i32"))
        })
        .unwrap_or_else(|| Ok(25))?;
    let search_text = query
        .get("search_text")
        .map(|v| Ok(Some(v.to_owned())))
        .unwrap_or_else(|| Ok(None))?;
    let user_role = query.get("user_role").cloned().unwrap_or_default();
    let user_type = query.get("user_type").cloned().unwrap_or_default();
    let role = match (user_type.as_ref(), user_role.as_ref()) {
        ("client", "individual") => {
            vec![Role::ClientIndividual]
        }
        ("client", "entity") => {
            vec![Role::ClientEntity]
        }
        ("client", _) => {
            vec![Role::ClientIndividual, Role::ClientEntity]
        }
        ("contractor", "individual") => {
            vec![Role::ContractorIndividual]
        }
        ("contractor", "entity") => {
            vec![Role::ContractorEntity]
        }
        ("contractor", _) => {
            vec![Role::ContractorIndividual, Role::ContractorEntity]
        }
        _ => vec![
            Role::ClientIndividual,
            Role::ClientEntity,
            Role::ContractorIndividual,
            Role::ContractorEntity,
        ],
    };
    let database = database.lock().await;
    let result = database
        .user_index(page, per_page, search_text, role)
        .await?;
    Ok(Json(result))
}
