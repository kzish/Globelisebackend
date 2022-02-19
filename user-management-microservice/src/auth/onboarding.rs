use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use axum::{
    body::Bytes,
    extract::{multipart::Field, ContentLengthLimit, Extension, Multipart},
};
use mime_detective::MimeDetective;
use rusty_ulid::Ulid;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use time::{format_description, Date};

use super::{error::Error, token::AccessToken, user::Role, SharedDatabase};

pub async fn individual_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
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
            let format = format_description::parse("[year]-[month]-[day]").unwrap();
            Date::parse(
                &text_fields.remove(&IndividualDetailNames::Dob).unwrap(),
                &format,
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
    let role: Role = claims.role.parse().unwrap();
    let database = database.lock().await;
    database
        .onboard_individual_details(ulid, Some(role), details)
        .await
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

    /*
    let mime = MimeDetective::new()
        .map_err(|_| Error::Internal)?
        .detect_buffer(&data)
        .map_err(|_| Error::Internal)?;
    if mime != content_type {
        return Err(Error::UnsupportedMediaType);
    }
    */

    Ok(data)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct IndividualDetails {
    first_name: String,
    last_name: String,
    dob: Date,
    dial_code: String,
    phone_number: String,
    country: String,
    address: String,
    city: String,
    postal_code: String,
    tax_id: Option<String>,
    time_zone: String,
    profile_picture: Option<Bytes>,
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

#[derive(Debug, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
enum BankDetails {
    BankName,
    AccountName,
    AccountNumber,
}

trait MultipartFormFields {
    fn is_required(&self) -> bool;
    fn is_image(&self) -> bool;
}

/// Maximum length of a `multipart/form-data` request.
const FORM_DATA_LENGTH_LIMIT: u64 = 6 * 1024 * 1024;
/// Maximum size of an uploaded image.
const IMAGE_SIZE_LIMIT: usize = 5 * 1024 * 1024;
