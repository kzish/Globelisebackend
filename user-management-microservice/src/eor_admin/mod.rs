use std::num::NonZeroU32;

use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{DateWrapper, EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_from_sql_uuid,
};
use csv::{ByteRecord, ReaderBuilder};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use time::{format_description, OffsetDateTime};

use crate::{
    auth::user::{Role, UserType},
    database::SharedDatabase,
    env::{
        GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
        USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
    },
    onboard::prefill::{PrefillBankDetails, PrefillIndividualDetails},
};

/// Stores information associated with a user id.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserIndex {
    pub ulid: Ulid,
    pub name: String,
    pub user_role: Role,
    pub user_type: UserType,
    pub email: String,
    pub created_at: String,
}

impl<'r> FromRow<'r, PgRow> for UserIndex {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let ulid = ulid_from_sql_uuid(row.try_get("ulid")?);
        let name = row.try_get("name")?;
        let role_str: String = row.try_get("user_role")?;
        let user_role =
            Role::try_from(role_str.as_str()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let type_str: String = row.try_get("user_type")?;
        let user_type =
            UserType::try_from(type_str.as_str()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let email = row.try_get("email")?;
        let timestamp_msec = ulid.timestamp() as i64 / 1000;
        let format = format_description::parse("[year]-[month]-[day]")
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let created_at = OffsetDateTime::from_unix_timestamp(timestamp_msec)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .format(&format)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        Ok(UserIndex {
            ulid,
            name,
            user_role,
            user_type,
            email,
            created_at,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UserIndexQuery {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<Role>,
}

pub async fn user_index(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<UserIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let database = database.lock().await;
    let result = database.user_index(query).await?;
    Ok(Json(result))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserRequest {
    email: String,
}

pub async fn add_individual_contractor(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(request): Json<AddUserRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let email_address: EmailAddress = request
        .email
        .parse()
        .map_err(|_| GlobeliseError::BadRequest("Not a valid email address"))?;

    let database = database.lock().await;
    if (database.user_id(&email_address).await?).is_some() {
        return Err(GlobeliseError::UnavailableEmail);
    };

    let receiver_email = email_address
        // TODO: Get the name of the person associated to this email address
        .to_display("")
        .parse()
        .map_err(|_| GlobeliseError::BadRequest("Bad request"))?;
    let email = Message::builder()
        .from(GLOBELISE_SENDER_EMAIL.clone())
        // TODO: Remove this because this is supposed to be a no-reply email?
        .reply_to(GLOBELISE_SENDER_EMAIL.clone())
        .to(receiver_email)
        .subject("Invitation to Globelise")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        // TODO: Once designer have a template for this. Use a templating library to populate data.
        .body(format!(
            r##"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Globelise Invitation</title>
            </head>
            <body>
                <p>
               Click the <a href="{}">link</a> to sign up as a Globelise individual contractor.
                </p>
                <p>If you did not expect to receive this email. Please ignore!</p>
            </body>
            </html>
            "##,
            (*USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL),
        ))
        .map_err(|_| {
            GlobeliseError::Internal("Could not create email for changing password".into())
        })?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)
        .map_err(|_| GlobeliseError::Internal("Could not connect to SMTP server".into()))?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    mailer
        .send(&email)
        .map_err(|e| GlobeliseError::Internal(e.to_string()))?;

    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddEmployeesInBulk {
    #[serde_as(as = "Base64")]
    pub uploaded_file: Vec<u8>,
    pub client_ulid: Ulid,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorDetailsForBulkUpload {
    #[serde(rename = "Email")]
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "DOB")]
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    #[serde(rename = "Dial Code")]
    pub dial_code: String,
    #[serde(rename = "Phone Number")]
    pub phone_number: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "ZIP/Postal Code")]
    pub postal_code: String,
    #[serde(rename = "Tax ID")]
    pub tax_id: String,
    #[serde(rename = "Timezone")]
    pub time_zone: String,
    #[serde(rename = "Bank Name")]
    pub bank_name: String,
    #[serde(rename = "Bank Account Number")]
    pub bank_account_number: String,
    #[serde(rename = "Bank Account Owner Name")]
    pub bank_account_owner_name: String,
}

pub async fn eor_admin_add_employees_in_bulk(
    // Only for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<AddEmployeesInBulk>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if let Ok(mut records) = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(request.uploaded_file.as_slice())
        .byte_records()
        .collect::<Result<Vec<ByteRecord>, _>>()
    {
        if !records.is_empty() {
            let header = records.swap_remove(0);
            for record in records {
                let value = record.deserialize::<PrefillIndividualContractorDetailsForBulkUpload>(
                    Some(&header),
                )?;

                let database = database.lock().await;

                database
                    .prefill_onboard_individual_contractors_account_details(
                        PrefillIndividualDetails {
                            email: value.email.clone(),
                            client_ulid: request.client_ulid,
                            first_name: value.first_name,
                            last_name: value.last_name,
                            dob: value.dob,
                            dial_code: value.dial_code,
                            phone_number: value.phone_number,
                            country: value.country,
                            city: value.city,
                            address: value.address,
                            postal_code: value.postal_code,
                            time_zone: value.time_zone,
                            tax_id: Some(value.tax_id),
                        },
                    )
                    .await?;

                database
                    .prefill_onboard_individual_contractors_bank_details(PrefillBankDetails {
                        email: value.email,
                        client_ulid: request.client_ulid,
                        bank_name: value.bank_name,
                        account_name: value.bank_account_owner_name,
                        account_number: value.bank_account_number,
                    })
                    .await?
            }
        }
    }
    Ok(())
}
