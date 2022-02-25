//! Types for user data.

use email_address::EmailAddress;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum UserType {
    Individual,
    Entity,
    EorAdmin,
}

impl UserType {
    pub fn db_auth_name(&self) -> &'static str {
        match self {
            UserType::Individual => "auth_individuals",
            UserType::Entity => "auth_entities",
            UserType::EorAdmin => "auth_eor_admins",
        }
    }

    pub fn db_onboard_name(&self, role: Role) -> &'static str {
        match (self, role) {
            (UserType::Individual, Role::Client) => "onboard_individual_clients",
            (UserType::Individual, Role::Contractor) => "onboard_individual_contractors",
            (UserType::Entity, Role::Client) => "onboard_entity_clients",
            (UserType::Entity, Role::Contractor) => "onboard_entity_contractors",
            (UserType::EorAdmin, _) => "onboard_eor_admins",
        }
    }
}

/// Type representing which role a user has.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Role {
    Client,
    Contractor,
}
