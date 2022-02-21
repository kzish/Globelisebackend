use std::hash::Hash;

use axum::{
    body::Bytes,
    extract::{ContentLengthLimit, Extension, Multipart},
};
use rusty_ulid::Ulid;
use strum::{EnumIter, EnumString};

use crate::auth::{error::Error, token::AccessToken, user::Role, SharedDatabase};

use super::multipart::{extract_multipart_form_data, MultipartFormFields, FORM_DATA_LENGTH_LIMIT};

pub async fn account_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let role: Role = claims.role.parse().unwrap();
    if !matches!(role, Role::EorAdmin) {
        return Err(Error::Forbidden);
    }

    let (mut text_fields, mut byte_fields) =
        extract_multipart_form_data::<EorDetailNames>(multipart).await?;

    let details = EorDetails {
        first_name: text_fields.remove(&EorDetailNames::FirstName).unwrap(),
        last_name: text_fields.remove(&EorDetailNames::LastName).unwrap(),
        dob: {
            sqlx::types::time::Date::parse(&text_fields.remove(&EorDetailNames::Dob).unwrap(), "%F")
                .map_err(|_| Error::BadRequest)?
        },
        dial_code: text_fields.remove(&EorDetailNames::DialCode).unwrap(),
        phone_number: text_fields.remove(&EorDetailNames::PhoneNumber).unwrap(),
        country: text_fields.remove(&EorDetailNames::Country).unwrap(),
        time_zone: text_fields.remove(&EorDetailNames::TimeZone).unwrap(),
        profile_picture: byte_fields.remove(&EorDetailNames::ProfilePicture),
    };

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database.onboard_eor_details(ulid, role, details).await
}

#[derive(Debug)]
pub struct EorDetails {
    pub first_name: String,
    pub last_name: String,
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub time_zone: String,
    pub profile_picture: Option<Bytes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
enum EorDetailNames {
    FirstName,
    LastName,
    Dob,
    DialCode,
    PhoneNumber,
    Country,
    TimeZone,
    ProfilePicture,
}

impl MultipartFormFields for EorDetailNames {
    fn is_required(&self) -> bool {
        !matches!(self, EorDetailNames::ProfilePicture)
    }

    fn is_image(&self) -> bool {
        matches!(self, EorDetailNames::ProfilePicture)
    }
}
