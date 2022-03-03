use std::{collections::HashMap, str::FromStr};

use axum::extract::{Extension, Json, Query};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use time::{format_description, OffsetDateTime};

use crate::{
    auth::{
        token::AdminAccessToken,
        user::{Role, UserType},
    },
    database::{ulid_from_sql_uuid, SharedDatabase},
    error::Error,
};

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub user_role: Role,
    pub user_type: UserType,
    pub email: String,
    pub created_at: String,
}

impl UserIndex {
    pub fn from_pg_row(row: PgRow) -> Result<Self, Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let name = row.try_get("name")?;
        let role_str: String = row.try_get("user_role")?;
        let user_role = Role::try_from(role_str.as_str())?;
        let type_str: String = row.try_get("user_type")?;
        let user_type = UserType::try_from(type_str.as_str())?;
        let email = row.try_get("email")?;
        let timestamp_msec = ulid.timestamp() as i64 / 1000;
        let format = format_description::parse("[year]-[month]-[day]")?;
        let created_at = OffsetDateTime::from_unix_timestamp(timestamp_msec)?.format(&format)?;
        Ok(UserIndex {
            ulid,
            name,
            user_role,
            user_type,
            email,
            created_at,
        })
    }
}

pub async fn user_index(
    // Only for validation
    _: AdminAccessToken,
    Query(query): Query<HashMap<String, String>>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<Json<Vec<UserIndex>>, Error> {
    let page = query
        .get("page")
        .map(|v| v.parse::<i64>())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid page param passed"))?;
    let per_page = query
        .get("per_page")
        .map(|v| v.parse::<i64>())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid per_page param passed"))?;
    let search_text = query.get("search_text").map(|v| v.to_owned());
    let user_type = query
        .get("user_type")
        .map(|r| UserType::from_str(r))
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid user_type param passed"))?;
    let user_role = query
        .get("user_role")
        .map(|r| Role::from_str(r))
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid user_role param passed"))?;
    let database = database.lock().await;
    let result = database
        .user_index(page, per_page, search_text, user_type, user_role)
        .await?;
    Ok(Json(result))
}
