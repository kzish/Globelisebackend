use common_utils::{custom_serde::DateWrapper, ulid_from_sql_uuid};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};

use super::user::{Role, UserType};

/// Stores information associated with a user id.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub user_role: Role,
    pub user_type: UserType,
    pub email: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub created_at: sqlx::types::time::Date,
}

impl<'r> FromRow<'r, PgRow> for UserIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let name = row.try_get("name")?;
        let role_str: String = row.try_get("user_role")?;
        let user_role =
            Role::try_from(role_str.as_str()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let type_str: String = row.try_get("user_type")?;
        let user_type =
            UserType::try_from(type_str.as_str()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let email = row.try_get("email")?;
        let created_at = row.try_get("created_at")?;
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
