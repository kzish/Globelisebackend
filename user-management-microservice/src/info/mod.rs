use std::{collections::HashMap, str::FromStr};

use axum::extract::{Extension, Json, Query};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use time::OffsetDateTime;

use crate::{
    auth::{token::AccessToken, user::Role},
    database::{ulid_from_sql_uuid, SharedDatabase},
    error::Error,
};

/// Year, month and day
#[derive(Debug, Deserialize, Serialize)]
pub struct Ymd {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl Ymd {
    pub fn from_offset_datetime(value: OffsetDateTime) -> Self {
        Ymd {
            year: value.year() as u32,
            month: value.month() as u32,
            day: value.day() as u32,
        }
    }
}

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub email: String,
    pub created_at: Ymd,
}

impl UserIndex {
    pub fn from_pg_row(row: PgRow) -> Result<Self, Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let name = row.try_get("name")?;
        let role = Role::from_str(row.try_get("role")?)?;
        let email = row.try_get("email")?;
        let timestamp_msec = ulid.timestamp();
        let created_at =
            Ymd::from_offset_datetime(OffsetDateTime::from_unix_timestamp(timestamp_msec as i64)?);
        Ok(UserIndex {
            ulid,
            name,
            role,
            email,
            created_at,
        })
    }
}

pub async fn eor_admin_user_index(
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
    let search_text = query.get("search_text").map(|v| v.to_owned());
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
        .eor_admin_user_index(page, per_page, search_text, role)
        .await?;
    Ok(Json(result))
}
