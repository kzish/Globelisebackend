use std::hash::Hash;

use axum::{
    body::Bytes,
    extract::{ContentLengthLimit, Extension, Multipart, Path},
};
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use strum::{EnumIter, EnumString};

use crate::{
    auth::{
        token::{AccessToken, AdminAccessToken},
        user::{Role, UserType},
    },
    database::SharedDatabase,
    error::Error,
};

use super::multipart::{extract_multipart_form_data, MultipartFormFields, FORM_DATA_LENGTH_LIMIT};

pub async fn account_details(
    claims: AccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let user_type: UserType = claims.user_type.parse().unwrap();
    if !matches!(user_type, UserType::Individual) {
        return Err(Error::Forbidden);
    }

    let details = IndividualDetails::from_multipart(multipart).await?;

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

impl IndividualDetails {
    pub async fn from_multipart(multipart: Multipart) -> Result<Self, Error> {
        let (mut text_fields, mut byte_fields) =
            extract_multipart_form_data::<IndividualDetailNames>(multipart).await?;

        Ok(Self {
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
        })
    }
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

pub async fn prefill_individual_contractor_account_details(
    // Only needed for validation
    _: AdminAccessToken,
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, FORM_DATA_LENGTH_LIMIT>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let prefill_details = PrefillIndividualDetails::from_multipart(multipart).await?;

    if !EmailAddress::is_valid(&prefill_details.email) {
        return Err(Error::BadRequest("Please provide a valid email address"));
    }

    let database = database.lock().await;

    let (email, details) = prefill_details.split();

    database
        .prefill_onboard_individual_contractors(email, details)
        .await
}

#[derive(Debug)]
pub struct PrefillIndividualDetails {
    pub email: String,
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

impl PrefillIndividualDetails {
    pub fn split(self) -> (String, IndividualDetails) {
        (
            self.email,
            IndividualDetails {
                first_name: self.first_name,
                last_name: self.last_name,
                dob: self.dob,
                dial_code: self.dial_code,
                phone_number: self.phone_number,
                country: self.country,
                city: self.city,
                address: self.address,
                postal_code: self.postal_code,
                tax_id: self.tax_id,
                time_zone: self.time_zone,
                profile_picture: self.profile_picture,
            },
        )
    }

    pub async fn from_multipart(multipart: Multipart) -> Result<Self, Error> {
        let (mut text_fields, mut byte_fields) =
            extract_multipart_form_data::<PrefillIndividualDetailNames>(multipart).await?;

        Ok(Self {
            email: text_fields
                .remove(&PrefillIndividualDetailNames::Email)
                .unwrap(),
            first_name: text_fields
                .remove(&PrefillIndividualDetailNames::FirstName)
                .unwrap(),
            last_name: text_fields
                .remove(&PrefillIndividualDetailNames::LastName)
                .unwrap(),
            dob: {
                sqlx::types::time::Date::parse(
                    &text_fields
                        .remove(&PrefillIndividualDetailNames::Dob)
                        .unwrap(),
                    "%F",
                )
                .map_err(|_| Error::BadRequest("Date must use YYYY-MM-DD format"))?
            },
            dial_code: text_fields
                .remove(&PrefillIndividualDetailNames::DialCode)
                .unwrap(),
            phone_number: text_fields
                .remove(&PrefillIndividualDetailNames::PhoneNumber)
                .unwrap(),
            country: text_fields
                .remove(&PrefillIndividualDetailNames::Country)
                .unwrap(),
            address: text_fields
                .remove(&PrefillIndividualDetailNames::Address)
                .unwrap(),
            city: text_fields
                .remove(&PrefillIndividualDetailNames::City)
                .unwrap(),
            postal_code: text_fields
                .remove(&PrefillIndividualDetailNames::PostalCode)
                .unwrap(),
            tax_id: text_fields.remove(&PrefillIndividualDetailNames::TaxId),
            time_zone: text_fields
                .remove(&PrefillIndividualDetailNames::TimeZone)
                .unwrap(),
            profile_picture: byte_fields.remove(&PrefillIndividualDetailNames::ProfilePicture),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum PrefillIndividualDetailNames {
    Email,
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

impl MultipartFormFields for PrefillIndividualDetailNames {
    fn is_required(&self) -> bool {
        !matches!(
            self,
            PrefillIndividualDetailNames::TaxId | PrefillIndividualDetailNames::ProfilePicture,
        )
    }

    fn is_image(&self) -> bool {
        matches!(self, PrefillIndividualDetailNames::ProfilePicture)
    }
}
