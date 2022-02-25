use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const USER_MANAGEMENT_MICROSERVICE: &str = "user-management-microservice";

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub role: Role,
    pub created_at: Option<String>,
    pub email: String,
}

/// NOTE: This is taken from user-management-microservice.
/// Preferably, this should exist in a separate library that each microservices
/// just consume.
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

impl Role {
    pub fn as_db_name(&self) -> &'static str {
        match self {
            Role::ClientIndividual => "client_individuals",
            Role::ClientEntity => "client_entities",
            Role::ContractorIndividual => "contractor_individuals",
            Role::ContractorEntity => "contractor_entities",
            Role::EorAdmin => "eor_admins",
        }
    }
}
