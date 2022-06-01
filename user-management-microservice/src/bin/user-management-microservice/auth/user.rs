//! Types for user data.

use common_utils::custom_serde::EmailWrapper;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Stores information associated with a user id.
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub email: EmailWrapper,
    pub password: Option<String>,
    pub is_google: bool,
    pub is_outlook: bool,
    pub is_entity: bool,
    pub is_individual: bool,
    pub is_client: bool,
    pub is_contractor: bool,
}
