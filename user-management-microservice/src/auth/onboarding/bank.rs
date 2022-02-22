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

#[derive(Debug, Deserialize)]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}
