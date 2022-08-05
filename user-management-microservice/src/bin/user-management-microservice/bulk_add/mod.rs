use argon2::hash_encoded;
use argon2::{self, Config};
use axum::{
    extract::{ContentLengthLimit, Extension, Json},
    http::{header::CONTENT_TYPE, HeaderValue},
    response::IntoResponse,
};
use calamine::Reader;
use common_utils::{
    custom_serde::{
        Country, Currency, EmailWrapper, OffsetDateWrapper, OptionOffsetDateWrapper, UserType,
        FORM_DATA_LENGTH_LIMIT,
    },
    database::{
        onboard::{bank::ContractorUserDetails, individual::IndividualContractorAccountDetails},
        CommonDatabase, Database,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use csv::{ReaderBuilder, StringRecord};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use std::{io::Cursor, str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::env::{FRONTEND_URL, GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillIndividualContractorDetailsForBulkUpload {
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "Gender")]
    pub gender: String,
    #[serde(rename = "Marital Status")]
    pub marital_status: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Date of Birth")]
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    #[serde(rename = "Dial Code")]
    pub dial_code: String,
    #[serde(rename = "Phone Number")]
    pub phone_number: String,
    #[serde(rename = "Email Address")]
    pub email: EmailWrapper,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Country")]
    pub country: Country,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "Postal Code")]
    pub postal_code: String,
    #[serde(rename = "National ID")]
    pub national_id: Option<String>,
    #[serde(rename = "Passport Number")]
    pub passport_number: Option<String>,
    #[serde(rename = "Passport Expiry Date")]
    pub passport_expiry_date: Option<String>,
    #[serde(rename = "Work Permit")]
    pub work_permit: Option<String>,
    #[serde(rename = "Tax ID")]
    pub tax_id: Option<String>,
    #[serde(rename = "Contribution ID #1")]
    pub contribution_id_1: Option<String>,
    #[serde(rename = "Contribution ID #2")]
    pub contribution_id_2: Option<String>,
    #[serde(rename = "Total Dependants")]
    pub total_dependants: Option<i64>,
    #[serde(rename = "Timezone")]
    pub time_zone: String,
    #[serde(rename = "Employee ID")]
    pub employee_id: Option<String>,
    #[serde(rename = "Designation")]
    pub designation: Option<String>,
    #[serde(rename = "Start Date")]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    #[serde(default)]
    pub start_date: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(rename = "End Date")]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    #[serde(default)]
    pub end_date: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(rename = "Employment Status")]
    pub employment_status: Option<String>,
    #[serde(rename = "Bank Name")]
    pub bank_name: String,
    #[serde(rename = "Bank Account Owner Name")]
    pub bank_account_name: String,
    #[serde(rename = "Bank Account Number")]
    pub bank_account_number: String,
    #[serde(rename = "Bank Code")]
    pub bank_code: String,
    #[serde(rename = "Branch Code")]
    pub bank_branch_code: String,
    #[serde(rename = "Currency")]
    pub currency: Currency,
    #[serde(rename = "Basic Salary")]
    pub basic_salary: Option<f64>,
    #[serde(rename = "Additional Item 1")]
    pub additional_item_1: String,
    #[serde(rename = "Additional Item 2")]
    pub additional_item_2: Option<String>,
    #[serde(rename = "Deduction 1")]
    pub deduction_1: Option<String>,
    #[serde(rename = "Deduction 2")]
    pub deduction_2: Option<String>,
    #[serde(rename = "Other Pay Item 1")]
    pub other_pay_item_1: Option<String>,
    #[serde(rename = "Other Pay Item 2")]
    pub other_pay_item_2: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostOneAddBulkEmployee {
    pub file_name: String,
    #[serde_as(as = "Base64")]
    pub file_data: Vec<u8>,
    pub client_ulid: Uuid,
    pub debug: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetPrefillIndividualContractorDetailsForBulkUpload {
    pub email: EmailWrapper,
}

pub async fn post_one(
    // Only for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PostOneAddBulkEmployee>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let mut rows_to_enter = vec![];

    let file_name = std::path::PathBuf::from_str(&body.file_name)?;

    enum FileType {
        Csv,
        Excel,
    }

    let file_extension = match file_name.extension().and_then(|s| s.to_str()) {
        Some("xls") | Some("xlsx") | Some("xlsb") | Some("ods") => Ok(FileType::Excel),
        Some("csv") => Ok(FileType::Csv),
        _ => Err(GlobeliseError::bad_request(
            "Cannot determine the file type from the file name",
        )),
    }?;

    match file_extension {
        FileType::Csv => {
            let mut records = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(body.file_data.as_slice())
                .records()
                .collect::<Result<Vec<StringRecord>, _>>()?;

            if !records.is_empty() {
                // Get/remove the first row because its the header.
                let header = records.swap_remove(0);
                for record in records {
                    let value = record
                        .deserialize::<PrefillIndividualContractorDetailsForBulkUpload>(Some(
                            &header,
                        ))
                        .map_err(|_| {
                            GlobeliseError::bad_request(
                                "Please provide a CSV file that follows the template",
                            )
                        })?;
                    rows_to_enter.push(value);
                }
            }
        }
        FileType::Excel => {
            let excel_workbook =
                calamine::open_workbook_auto_from_rs(Cursor::new(body.file_data.as_slice()))?
                    .worksheets();

            let first_worksheet = excel_workbook.first().ok_or_else(|| {
                GlobeliseError::bad_request(
                    "Please provide an excel file with at least 1 worksheet",
                )
            })?;

            let row_deserializer = first_worksheet.1.deserialize()?;

            let rows = row_deserializer
                .into_iter()
                .collect::<Result<Vec<PrefillIndividualContractorDetailsForBulkUpload>, _>>()?;

            rows_to_enter.extend(rows);
        }
    };

    let mut errors = vec![];

    for value in rows_to_enter {
        if let Err(err) = process_row(value, database.clone(), body.debug, body.client_ulid).await {
            errors.push(err);
        }
    }

    Ok(())
}

async fn process_row(
    value: PrefillIndividualContractorDetailsForBulkUpload,
    database: Arc<Mutex<Database>>,
    debug: Option<bool>,
    client_ulid: Uuid,
) -> GlobeliseResult<()> {
    let receiver_email = value
        .email
        // TODO: Get the name of the person associated to this email address
        .0
        .to_display("")
        .parse::<Mailbox>()?;

    let database = database.lock().await;

    let mut user_created = false;

    let user_ulid = if let Some(user) = database
        .find_one_user(None, Some(&value.email), None)
        .await?
    {
        user.ulid
    } else {
        user_created = true;
        //TODO add default password and create also a benefits user
        let default_password_string =
            std::env::var("DEFAULT_USER_PASSWORD").expect("default password not set");
        let salt: [u8; 16] = rand::thread_rng().gen();
        let default_password_hash =
            hash_encoded(default_password_string.as_bytes(), &salt, &HASH_CONFIG)
                .map_err(GlobeliseError::internal)?;
        database
            .insert_one_user(
                &value.email,
                Some(&default_password_hash),
                false,
                false,
                false,
                true,
                false,
                true,
            )
            .await?
    };

    if database
        .select_one_onboard_individual_contractor_account_details(user_ulid)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_individual_contractor_account_details(
                user_ulid,
                &IndividualContractorAccountDetails {
                    first_name: value.first_name,
                    last_name: value.last_name,
                    dob: value.dob,
                    dial_code: value.dial_code,
                    phone_number: value.phone_number,
                    country: value.country,
                    city: value.city,
                    address: value.address,
                    postal_code: value.postal_code,
                    tax_id: value.tax_id,
                    time_zone: value.time_zone,
                    profile_picture: None,
                    cv: None,
                    gender: value.gender,
                    marital_status: value.marital_status,
                    nationality: Some(value.nationality),
                    email_address: Some(value.email),
                    national_id: value.national_id,
                    passport_number: value.passport_number,
                    passport_expiry_date: value.passport_expiry_date,
                    work_permit: value.work_permit,
                    added_related_pay_item_id: None,
                    total_dependants: value.total_dependants,
                },
            )
            .await?;
    }

    if database
        .select_one_onboard_user_bank_detail(user_ulid, UserType::Individual)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_user_bank_details(
                user_ulid,
                UserType::Individual,
                &ContractorUserDetails {
                    bank_name: value.bank_name,
                    bank_account_name: value.bank_account_name,
                    bank_account_number: value.bank_account_number,
                    bank_code: value.bank_code,
                    branch_code: value.bank_branch_code,
                },
            )
            .await?;
    }
    //link this contractor to this client
    database
        .create_client_contractor_pair(client_ulid, user_ulid)
        .await?;

    if let Some(true) = debug {
        return Ok(());
    }

    // Send email to the contractor

    if user_created {
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
           Click the <a href="{}/signup?as=contractor&type=individual">link</a> to sign up as a Globelise individual contractor.
            </p>
            <p>If you did not expect to receive this email. Please ignore!</p>
        </body>
        </html>
        "##,
                (*FRONTEND_URL),
            ))?;

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)?
            .credentials(SMTP_CREDENTIAL.clone())
            .build();

        // Send the email
        mailer.send(&email)?;
    }

    Ok(())
}

pub async fn download(_: Token<AdminAccessToken>) -> impl IntoResponse {
    let bytes = include_bytes!("add_bulk_employees.xlsx").to_vec();
    (
        [(
            CONTENT_TYPE,
            HeaderValue::from_static(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ),
        )],
        bytes,
    )
}

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});
