//! Functions and types for handling authorization tokens.

use error::GlobeliseResult;
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use strum::Display;

pub mod custom_serde;
pub mod error;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "kebab-case")]
pub enum DaprAppId {
    UserManagementMicroservice,
    EorAdminMicroservice,
    ContractorManagementMicroservice,
}

impl DaprAppId {
    pub fn as_str(&self) -> &'static str {
        match self {
            DaprAppId::UserManagementMicroservice => "user-management-microservice",
            DaprAppId::EorAdminMicroservice => "eor-admin-microservice",
            DaprAppId::ContractorManagementMicroservice => "contractor-management-microservice",
        }
    }

    pub fn microservice_domain_url(&self) -> GlobeliseResult<String> {
        Ok((match self {
            DaprAppId::UserManagementMicroservice => {
                std::env::var("USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL")
            }
            DaprAppId::EorAdminMicroservice => std::env::var("EOR_ADMIN_MICROSERVICE_DOMAIN_URL"),
            DaprAppId::ContractorManagementMicroservice => {
                std::env::var("CONTRACTOR_MANAGEMENT_MICROSERVICE_DOMAIN_URL")
            }
        })?)
    }
}

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
