use std::hash::Hash;

use axum::{
    body::Bytes,
    extract::{ContentLengthLimit, Extension, Multipart},
};
use rusty_ulid::Ulid;
use strum::{EnumIter, EnumString};

use crate::{
    auth::{token::AccessToken, user::Role},
    database::SharedDatabase,
    error::Error,
};

use super::multipart::{extract_multipart_form_data, MultipartFormFields, FORM_DATA_LENGTH_LIMIT};

pub async fn account_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let role: Role = claims.role.parse().unwrap();
    if !matches!(
        role,
        Role::IndividualClient | Role::IndividualContractor | Role::EorAdmin
    ) {
        return Err(Error::Forbidden);
    }

    let (mut text_fields, mut byte_fields) =
        extract_multipart_form_data::<IndividualDetailNames>(multipart).await?;

    let details = IndividualDetails {
        first_name: text_fields
            .remove(&IndividualDetailNames::FirstName)
            .unwrap(),
        last_name: text_fields
            .remove(&IndividualDetailNames::LastName)
            .unwrap(),
        dob: {
            sqlx::types::time::Date::parse(
                &text_fields.remove(&IndividualDetailNames::Dob).unwrap(),
                "%F",
            )
            .map_err(|_| Error::BadRequest("Date must use YYYY-MM-DD format"))?
        },
        dial_code: text_fields
            .remove(&IndividualDetailNames::DialCode)
            .unwrap(),
        phone_number: text_fields
            .remove(&IndividualDetailNames::PhoneNumber)
            .unwrap(),
        country: text_fields.remove(&IndividualDetailNames::Country).unwrap(),
        address: text_fields.remove(&IndividualDetailNames::Address).unwrap(),
        city: text_fields.remove(&IndividualDetailNames::City).unwrap(),
        postal_code: text_fields
            .remove(&IndividualDetailNames::PostalCode)
            .unwrap(),
        tax_id: text_fields.remove(&IndividualDetailNames::TaxId),
        time_zone: text_fields
            .remove(&IndividualDetailNames::TimeZone)
            .unwrap(),
        profile_picture: byte_fields.remove(&IndividualDetailNames::ProfilePicture),
    };

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database
        .onboard_individual_details(ulid, role, details)
        .await
}

#[derive(Debug)]
pub struct IndividualDetails {
    pub first_name: String,
    pub last_name: String,
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    pub tax_id: Option<String>,
    pub time_zone: String,
    pub profile_picture: Option<Bytes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum IndividualDetailNames {
    FirstName,
    LastName,
    Dob,
    DialCode,
    PhoneNumber,
    Country,
    City,
    Address,
    PostalCode,
    TaxId,
    TimeZone,
    ProfilePicture,
}

impl MultipartFormFields for IndividualDetailNames {
    fn is_required(&self) -> bool {
        !matches!(
            self,
            IndividualDetailNames::TaxId | IndividualDetailNames::ProfilePicture,
        )
    }

    fn is_image(&self) -> bool {
        matches!(self, IndividualDetailNames::ProfilePicture)
    }
}
