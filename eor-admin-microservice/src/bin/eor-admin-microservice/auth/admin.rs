//! Types for admin data.

use common_utils::error::GlobeliseResult;
use email_address::EmailAddress;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

/// Stores information associated with a admin id.
#[derive(Debug, Deserialize, Serialize)]
pub struct Admin {
    pub email: EmailAddress,
    pub password_hash: Option<String>,
    pub google: bool,
    pub outlook: bool,
}

impl Admin {
    pub fn has_authentication(&self) -> bool {
        self.password_hash.is_some() || self.google || self.outlook
    }

    pub fn from_pg_row(row: PgRow) -> GlobeliseResult<Self> {
        Ok(Admin {
            email: row.try_get::<String, _>("email")?.parse()?,
            password_hash: row.try_get("password")?,
            google: row.try_get("is_google")?,
            outlook: row.try_get("is_outlook")?,
        })
    }
}
