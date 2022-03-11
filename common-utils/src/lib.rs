//! Functions and types for handling authorization tokens.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub mod error;
pub mod token;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum UserType {
    Individual,
    Entity,
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
