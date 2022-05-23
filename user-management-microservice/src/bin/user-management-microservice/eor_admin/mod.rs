use axum::extract::{Extension, Json, Query};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use user_management_microservice_sdk::{
    user::{Role, UserType},
    user_index::{OnboardedUserIndex, UserIndex},
};

use crate::{
    database::SharedDatabase,
    env::{
        GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
        USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
    },
};

pub mod client_contractor_pair;
pub mod entity_contractor_branch_pair;
pub mod individual_contractor_branch_pair;
pub mod pay_items;
pub mod prefill;
pub mod search_employee_contractors;

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
