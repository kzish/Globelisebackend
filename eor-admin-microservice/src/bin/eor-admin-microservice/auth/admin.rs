//! Types for admin data.

use email_address::EmailAddress;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

/// Stores information associated with a admin id.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Admin {
    pub email: EmailAddress,
    pub password_hash: Option<String>,
    pub google: bool,
    pub outlook: bool,
}

impl<'r> FromRow<'r, PgRow> for Admin {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            email: row
                .try_get::<String, _>("email")?
                .parse::<EmailAddress>()
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            password_hash: row.try_get("password")?,
            google: row.try_get("is_google")?,
            outlook: row.try_get("is_outlook")?,
        })
    }
}

impl Admin {
    pub fn has_authentication(&self) -> bool {
        self.password_hash.is_some() || self.google || self.outlook
    }
}
