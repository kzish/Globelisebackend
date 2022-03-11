use axum::extract::{Extension, Form};
use common_utils::token::Token;
use rusty_ulid::Ulid;
use serde::Deserialize;

use crate::{
    auth::{token::AccessToken, user::UserType},
    database::SharedDatabase,
    error::Error,
};

pub async fn bank_details(
    claims: Token<AccessToken>,
    Form(details): Form<BankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let user_type = claims.payload.user_type.parse::<UserType>().unwrap();

    let ulid = claims.payload.ulid.parse::<Ulid>().unwrap();
    let database = database.lock().await;
    database
        .onboard_bank_details(ulid, user_type, details)
        .await
}

#[derive(Debug, Deserialize)]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}
