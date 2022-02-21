use axum::extract::{Extension, Form};
use rusty_ulid::Ulid;
use serde::Deserialize;

use crate::auth::{error::Error, token::AccessToken, user::Role, SharedDatabase};

pub async fn bank_details(
    claims: AccessToken,
    Form(details): Form<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let role: Role = claims.role.parse().unwrap();
    if !matches!(role, Role::ContractorIndividual | Role::ContractorEntity) {
        return Err(Error::Forbidden);
    }

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database.onboard_bank_details(ulid, role, details).await
}

pub async fn eor_bank_details(
    claims: AccessToken,
    Form(details): Form<EorBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let role: Role = claims.role.parse().unwrap();
    if !matches!(role, Role::EorAdmin) {
        return Err(Error::Forbidden);
    }

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database.onboard_eor_bank_details(ulid, role, details).await
}

#[derive(Debug, Deserialize)]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

#[derive(Debug, Deserialize)]
pub struct EorBankDetails {
    pub bank_name: String,
    pub account_number: String,
    pub city_address: Option<String>,
    pub postal_code: Option<String>,
    pub tax_id: String,
}
