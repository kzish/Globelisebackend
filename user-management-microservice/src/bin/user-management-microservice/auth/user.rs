//! Types for user data.

use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: EmailAddress,
    pub password_hash: Option<String>,
    pub google: bool,
    pub outlook: bool,
}
