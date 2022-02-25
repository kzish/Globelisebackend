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
    if !matches!(role, Role::EntityClient | Role::EntityContractor) {
        return Err(Error::Forbidden);
    }

    let (mut text_fields, mut byte_fields) =
        extract_multipart_form_data::<EntityDetailNames>(multipart).await?;

    let details = EntityDetails {
        company_name: text_fields.remove(&EntityDetailNames::CompanyName).unwrap(),
        country: text_fields.remove(&EntityDetailNames::Country).unwrap(),
        entity_type: text_fields.remove(&EntityDetailNames::EntityType).unwrap(),
        registration_number: text_fields.remove(&EntityDetailNames::RegistrationNumber),
        tax_id: text_fields.remove(&EntityDetailNames::TaxId),
        company_address: text_fields
            .remove(&EntityDetailNames::CompanyAddress)
            .unwrap(),
        city: text_fields.remove(&EntityDetailNames::City).unwrap(),
        postal_code: text_fields.remove(&EntityDetailNames::PostalCode).unwrap(),
        time_zone: text_fields.remove(&EntityDetailNames::TimeZone).unwrap(),
        logo: byte_fields.remove(&EntityDetailNames::Logo),
    };

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database.onboard_entity_details(ulid, role, details).await
}

pub async fn pic_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let role: Role = claims.role.parse().unwrap();
    if !matches!(role, Role::EntityClient | Role::EntityContractor) {
        return Err(Error::Forbidden);
    }

    let (mut text_fields, mut byte_fields) =
        extract_multipart_form_data::<PicDetailNames>(multipart).await?;

    let details = PicDetails {
        first_name: text_fields.remove(&PicDetailNames::FirstName).unwrap(),
        last_name: text_fields.remove(&PicDetailNames::LastName).unwrap(),
        dob: {
            sqlx::types::time::Date::parse(&text_fields.remove(&PicDetailNames::Dob).unwrap(), "%F")
                .map_err(|_| Error::BadRequest("Date must use YYYY-MM-DD format"))?
        },
        dial_code: text_fields.remove(&PicDetailNames::DialCode).unwrap(),
        phone_number: text_fields.remove(&PicDetailNames::PhoneNumber).unwrap(),
        profile_picture: byte_fields.remove(&PicDetailNames::ProfilePicture),
    };

    let ulid: Ulid = claims.sub.parse().unwrap();
    let database = database.lock().await;
    database.onboard_pic_details(ulid, role, details).await
}

pub struct EntityDetails {
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    pub logo: Option<Bytes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum EntityDetailNames {
    CompanyName,
    Country,
    EntityType,
    RegistrationNumber,
    TaxId,
    CompanyAddress,
    City,
    PostalCode,
    TimeZone,
    Logo,
}

impl MultipartFormFields for EntityDetailNames {
    fn is_required(&self) -> bool {
        !matches!(
            self,
            EntityDetailNames::RegistrationNumber
                | EntityDetailNames::TaxId
                | EntityDetailNames::Logo
        )
    }

    fn is_image(&self) -> bool {
        matches!(self, EntityDetailNames::Logo)
    }
}

pub struct PicDetails {
    pub first_name: String,
    pub last_name: String,
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub profile_picture: Option<Bytes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum PicDetailNames {
    FirstName,
    LastName,
    Dob,
    DialCode,
    PhoneNumber,
    ProfilePicture,
}

impl MultipartFormFields for PicDetailNames {
    fn is_required(&self) -> bool {
        !matches!(self, PicDetailNames::ProfilePicture)
    }

    fn is_image(&self) -> bool {
        matches!(self, PicDetailNames::ProfilePicture)
    }
}
