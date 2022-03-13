use axum::extract::{Extension, Json};
use rusty_ulid::Ulid;
use serde::Deserialize;

use crate::{
    auth::{token::AccessToken, user::UserType},
    database::SharedDatabase,
    error::Error,
};

pub async fn bank_details(
    claims: AccessToken,
    Json(details): Json<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let user_type: UserType = claims.user_type.parse().unwrap();

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database
        .onboard_bank_details(ulid, user_type, details)
        .await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}
