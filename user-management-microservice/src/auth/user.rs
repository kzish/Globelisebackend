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
    ClientIndividual,
    ClientEntity,
    ContractorIndividual,
    ContractorEntity,
    EorAdmin,
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            "client-individual" => Ok(Role::ClientIndividual),
            "client-entity" => Ok(Role::ClientEntity),
            "contractor-individual" => Ok(Role::ContractorIndividual),
            "contractor-entity" => Ok(Role::ContractorEntity),
            "eor-admin" => Ok(Role::EorAdmin),
            _ => Err(Error::Unauthorized("Invalid role")),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::ClientIndividual => write!(f, "client-individual"),
            Role::ClientEntity => write!(f, "client-entity"),
            Role::ContractorIndividual => write!(f, "contractor-individual"),
            Role::ContractorEntity => write!(f, "contractor-entity"),
            Role::EorAdmin => write!(f, "eor-admin"),
        }
    }
}
