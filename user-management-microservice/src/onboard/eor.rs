use axum::extract::{ContentLengthLimit, Extension, Multipart};
use rusty_ulid::Ulid;

use crate::{
    auth::{
        token::AccessToken,
        user::{Role, UserType},
    },
    database::SharedDatabase,
    error::Error,
};

use super::{individual::IndividualDetails, multipart::FORM_DATA_LENGTH_LIMIT};

pub async fn account_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let user_type: UserType = claims.user_type.parse().unwrap();
    if !matches!(user_type, UserType::EorAdmin) {
        return Err(Error::Forbidden);
    }

    let details = IndividualDetails::from_multipart(multipart).await?;

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database
        .onboard_individual_details(ulid, user_type, Role::Client, details)
        .await
}
