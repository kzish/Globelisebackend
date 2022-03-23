use axum::extract::{Extension, Json};
use common_utils::{error::GlobeliseResult, token::Token};
use rusty_ulid::Ulid;
use serde::Deserialize;

use crate::{
    auth::{token::AccessToken, user::UserType},
    database::SharedDatabase,
};

pub async fn bank_details(
    claims: Token<AccessToken>,
    Json(details): Json<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let user_type = claims.payload.user_type.parse::<UserType>().unwrap();

    let ulid = claims.payload.ulid.parse::<Ulid>()?;
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
