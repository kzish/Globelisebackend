//! Types for user data.

use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: EmailAddress,
    pub password_hash: Option<String>,
    pub is_google: bool,
    pub is_outlook: bool,
    pub is_entity: bool,
    pub is_individual: bool,
    pub is_client: bool,
    pub is_contractor: bool,
}
