//! Types for user data.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum UserType {
    Individual,
    Entity,
}

impl UserType {
    pub fn db_auth_name(&self) -> &'static str {
        match self {
            UserType::Individual => "auth_individuals",
            UserType::Entity => "auth_entities",
        }
    }

    pub fn db_onboard_details_prefix(&self, role: Role) -> String {
        UserTypeAndRole::from((*self, role)).to_string() + "s"
    }
}

/// Type representing which role a user has.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Role {
    Client,
    Contractor,
}

/// Possible user type and role combinations.
#[derive(Debug, Display)]
#[strum(serialize_all = "snake_case")]
enum UserTypeAndRole {
    IndividualClient,
    IndividualContractor,
    EntityClient,
    EntityContractor,
}

impl From<(UserType, Role)> for UserTypeAndRole {
    fn from(value: (UserType, Role)) -> Self {
        match value {
            (UserType::Individual, Role::Client) => UserTypeAndRole::IndividualClient,
            (UserType::Individual, Role::Contractor) => UserTypeAndRole::IndividualContractor,
            (UserType::Entity, Role::Client) => UserTypeAndRole::EntityClient,
            (UserType::Entity, Role::Contractor) => UserTypeAndRole::EntityContractor,
        }
    }
}
