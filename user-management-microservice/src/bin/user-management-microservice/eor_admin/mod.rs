use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{DateWrapper, EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    pubsub::{AddClientContractorPair, SharedPubSub},
    token::Token,
};
use csv::{ReaderBuilder, StringRecord};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AccessToken as AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use user_management_microservice_sdk::{
    user::{Role, UserType},
    user_index::{OnboardedUserIndex, UserIndex},
};

use crate::{
    auth::user::User,
    database::SharedDatabase,
    env::{
        GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
        USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
    },
    onboard::prefill::{PrefillBankDetails, PrefillIndividualDetails},
};

pub mod client_contractor_pair;
pub mod individual_contractor_branch_pair;
pub mod pay_items;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardedUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<Role>,
}

pub async fn eor_admin_onboarded_user_index(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<OnboardedUserIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = database.lock().await;
    let result = database.onboarded_user_index(query).await?;
    Ok(Json(result))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddUserRequest {
    email: String,
    debug: Option<bool>,
}

pub async fn add_individual_contractor(
    // Only for validation
    _: Token<AdminAccessToken>,
    Json(request): Json<AddUserRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let email_address: EmailAddress = request.email.parse().map_err(GlobeliseError::bad_request)?;

    let database = database.lock().await;

    if (database.user_id(&email_address).await?).is_some() {
        return Err(GlobeliseError::UnavailableEmail);
    };

    // If  in debug mode, skip sending emails
    if let Some(true) = request.debug {
        return Ok(());
    }

    let receiver_email = email_address
        // TODO: Get the name of the person associated to this email address
        .to_display("")
        .parse()?;
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
        ))?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    mailer.send(&email)?;

    Ok(())
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddEmployeesInBulk {
    #[serde_as(as = "Base64")]
    pub uploaded_file: Vec<u8>,
    pub client_ulid: Ulid,
    pub debug: Option<bool>,
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
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if let Ok(mut records) = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(request.uploaded_file.as_slice())
        .records()
        .collect::<Result<Vec<StringRecord>, _>>()
    {
        if !records.is_empty() {
            // Get/remove the first row because its the header.
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
                        email: value.email.clone(),
                        client_ulid: request.client_ulid,
                        bank_name: value.bank_name,
                        account_name: value.bank_account_owner_name,
                        account_number: value.bank_account_number,
                    })
                    .await?;

                // Create contractor ULIDs
                let contractor_ulid = database
                    .create_user(
                        User {
                            email: value.email.clone(),
                            password_hash: None,
                            google: false,
                            outlook: false,
                        },
                        UserType::Individual,
                    )
                    .await?;

                // Create client/contractor pairs
                database
                    .create_client_contractor_pairs(request.client_ulid, contractor_ulid)
                    .await?;

                // Publish event to DAPR
                let pubsub = pubsub.lock().await;
                pubsub
                    .publish_event(AddClientContractorPair {
                        client_ulid: request.client_ulid,
                        contractor_ulid,
                    })
                    .await?;

                if let Some(true) = request.debug {
                    return Ok(());
                }

                // Send email to the contractor
                let receiver_email = value
                    .email
                    // TODO: Get the name of the person associated to this email address
                    .to_display("")
                    .parse()?;
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
                ))?;

                // Open a remote connection to gmail
                let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)?
                    .credentials(SMTP_CREDENTIAL.clone())
                    .build();

                // Send the email
                mailer.send(&email)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
}

pub async fn eor_admin_user_index(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<UserIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let database = database.lock().await;
    let result = database.user_index(query).await?;
    Ok(Json(result))
}
