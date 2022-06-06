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
    pub fn db_onboard_details_prefix(&self, role: UserRole) -> String {
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

impl sqlx::encode::Encode<'_, sqlx::Postgres> for UserType {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::encode(val, buf)
    }
    fn size_hint(&self) -> std::primitive::usize {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::size_hint(&val)
    }
}

/// Type representing which role a user has.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum UserRole {
    Client,
    Contractor,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Client => "client",
            UserRole::Contractor => "contractor",
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for UserRole {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("text")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserRole {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value: &'r str = sqlx::decode::Decode::decode(value)?;
        Ok(UserRole::from_str(value)?)
    }
}

impl sqlx::encode::Encode<'_, sqlx::Postgres> for UserRole {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::encode(val, buf)
    }
    fn size_hint(&self) -> std::primitive::usize {
        let val = self.as_str();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::size_hint(&val)
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

impl From<(UserType, UserRole)> for UserTypeAndRole {
    fn from(value: (UserType, UserRole)) -> Self {
        match value {
            (UserType::Individual, UserRole::Client) => UserTypeAndRole::IndividualClient,
            (UserType::Individual, UserRole::Contractor) => UserTypeAndRole::IndividualContractor,
            (UserType::Entity, UserRole::Client) => UserTypeAndRole::EntityClient,
            (UserType::Entity, UserRole::Contractor) => UserTypeAndRole::EntityContractor,
        }
    }
}
