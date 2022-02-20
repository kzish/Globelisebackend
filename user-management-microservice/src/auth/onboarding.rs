use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use axum::{
    body::Bytes,
    extract::{multipart::Field, ContentLengthLimit, Extension, Form, Multipart},
};
use rusty_ulid::Ulid;
use serde::Deserialize;
use strum::{EnumIter, EnumString, IntoEnumIterator};

use super::{error::Error, token::AccessToken, user::Role, SharedDatabase};

pub async fn individual_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let role: Role = claims.role.parse().unwrap();
    if !matches!(role, Role::ClientIndividual | Role::ContractorIndividual) {
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
            .map_err(|_| Error::BadRequest)?
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

async fn extract_multipart_form_data<T>(
    mut multipart: Multipart,
) -> Result<(HashMap<T, String>, HashMap<T, Bytes>), Error>
where
    T: Copy + Eq + Hash + MultipartFormFields + IntoEnumIterator + FromStr,
{
    let mut text_fields = HashMap::<T, String>::new();
    let mut byte_fields = HashMap::<T, Bytes>::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::BadRequest)?
    {
        let name = field.name().ok_or(Error::BadRequest)?;
        let name: T = name.parse().map_err(|_| Error::BadRequest)?;

        if name.is_image() {
            let data = validate_image_field(field).await?;
            if text_fields.contains_key(&name) || byte_fields.insert(name, data).is_some() {
                return Err(Error::BadRequest);
            }
        } else {
            let data = field.text().await.map_err(|_| Error::BadRequest)?;
            if byte_fields.contains_key(&name) || text_fields.insert(name, data).is_some() {
                return Err(Error::BadRequest);
            }
        }
    }

    let fields_found: HashSet<T> = text_fields
        .keys()
        .chain(byte_fields.keys())
        .copied()
        .collect();
    for field in T::iter().filter(|f| f.is_required()) {
        if !fields_found.contains(&field) {
            return Err(Error::BadRequest);
        }
    }

    Ok((text_fields, byte_fields))
}

async fn validate_image_field(field: Field<'_>) -> Result<Bytes, Error> {
    let filename = field.file_name().ok_or(Error::BadRequest)?;
    let mut file_extension = std::path::Path::new(filename)
        .extension()
        .ok_or(Error::UnsupportedMediaType)?
        .to_owned();
    file_extension.make_ascii_lowercase();
    let file_extension = file_extension;
    if !(file_extension == "png" || file_extension == "jpeg" || file_extension == "jpg") {
        return Err(Error::UnsupportedMediaType);
    }

    let content_type = field.content_type().ok_or(Error::BadRequest)?.to_owned();
    match (content_type.type_(), content_type.subtype()) {
        (mime::IMAGE, mime::PNG) => {
            if file_extension != "png" {
                return Err(Error::UnsupportedMediaType);
            }
        }
        (mime::IMAGE, mime::JPEG) => {
            if !(file_extension == "jpeg" || file_extension == "jpg") {
                return Err(Error::UnsupportedMediaType);
            }
        }
        _ => return Err(Error::UnsupportedMediaType),
    }

    let data = field.bytes().await.map_err(|_| Error::BadRequest)?;
    if data.len() > IMAGE_SIZE_LIMIT {
        return Err(Error::PayloadTooLarge);
    }

    let cookie = magic::Cookie::open(magic::CookieFlags::MIME_TYPE | magic::CookieFlags::ERROR)
        .map_err(|_| Error::Internal)?;
    cookie.load::<&str>(&[]).map_err(|_| Error::Internal)?;
    let detected_mime = cookie.buffer(&data).map_err(|_| Error::Internal)?;
    if detected_mime != content_type.to_string() {
        return Err(Error::UnsupportedMediaType);
    }

    Ok(data)
}

#[derive(Debug)]
pub struct IndividualDetails {
    pub first_name: String,
    pub last_name: String,
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub tax_id: Option<String>,
    pub time_zone: String,
    pub profile_picture: Option<Bytes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
enum IndividualDetailNames {
    FirstName,
    LastName,
    Dob,
    DialCode,
    PhoneNumber,
    Country,
    Address,
    City,
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

#[derive(Debug, Deserialize)]
pub struct BankDetails {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
}

trait MultipartFormFields {
    fn is_required(&self) -> bool;
    fn is_image(&self) -> bool;
}

/// Maximum length of a `multipart/form-data` request.
const FORM_DATA_LENGTH_LIMIT: u64 = 6 * 1024 * 1024;
/// Maximum size of an uploaded image.
const IMAGE_SIZE_LIMIT: usize = 5 * 1024 * 1024;
