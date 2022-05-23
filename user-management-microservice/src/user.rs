//! Types for user data.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgTypeInfo, PgValueRef};
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

    pub fn as_str(&self) -> &'static str {
        match self {
            UserType::Individual => "individual",
            UserType::Entity => "entity",
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for UserType {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("text")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserType {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value: &'r str = sqlx::decode::Decode::decode(value)?;
        Ok(UserType::from_str(value)?)
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

impl sqlx::Type<sqlx::Postgres> for Role {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("text")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Role {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value: &'r str = sqlx::decode::Decode::decode(value)?;
        Ok(Role::from_str(value)?)
    }
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
