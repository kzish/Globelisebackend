//! Functions and types for handling authorization tokens.

use error::GlobeliseResult;
use serde::{Deserialize, Serialize};
use strum::Display;

pub mod custom_serde;
pub mod database;
pub mod error;
pub mod pubsub;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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

pub fn calc_limit_and_offset(
    per_page: Option<u32>,
    page: Option<u32>,
) -> (Option<u32>, Option<u32>) {
    let limit = per_page;
    let offset = limit.and_then(|v| page.map(|w| (w - 1) * v));
    (limit, offset)
}
