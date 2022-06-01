//! Types for admin data.

use common_utils::custom_serde::EmailWrapper;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Stores information associated with a admin id.
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Admin {
    pub email: EmailWrapper,
    pub password: Option<String>,
    pub is_google: bool,
    pub is_outlook: bool,
}

impl Admin {
    pub fn has_authentication(&self) -> bool {
        self.password.is_some() || self.is_google || self.is_outlook
    }
}
