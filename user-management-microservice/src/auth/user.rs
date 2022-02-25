//! Types for user data.

use std::{fmt, str::FromStr};

use email_address::EmailAddress;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::error::Error;

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: EmailAddress,
    pub password_hash: Option<String>,
    pub google: bool,
    pub outlook: bool,
}

impl User {
    pub fn has_authentication(&self) -> bool {
        self.password_hash.is_some() || self.google || self.outlook
    }
}

/// Type representing which role a user has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    IndividualClient,
    EntityClient,
    IndividualContractor,
    EntityContractor,
    EorAdmin,
}

impl Role {
    pub fn as_db_name(&self) -> &'static str {
        match self {
            Role::IndividualClient => "client_individuals",
            Role::EntityClient => "client_entities",
            Role::IndividualContractor => "contractor_individuals",
            Role::EntityContractor => "contractor_entities",
            Role::EorAdmin => "eor_admins",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::IndividualClient => "client-individual",
            Role::EntityClient => "client-entity",
            Role::IndividualContractor => "contractor-individual",
            Role::EntityContractor => "contractor-entity",
            Role::EorAdmin => "eor-admin",
        }
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            "client-individual" => Ok(Role::IndividualClient),
            "client-entity" => Ok(Role::EntityClient),
            "contractor-individual" => Ok(Role::IndividualContractor),
            "contractor-entity" => Ok(Role::EntityContractor),
            "eor-admin" => Ok(Role::EorAdmin),
            _ => Err(Error::Unauthorized("Invalid role")),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
