use argon2::{hash_encoded, Config};
use axum::extract::{ContentLengthLimit, Extension, Json, Query};
use common_utils::{
    custom_serde::{
        EmailWrapper, OptionOffsetDateWrapper, UserRole, UserType, FORM_DATA_LENGTH_LIMIT,
    },
    database::{
        user::{OnboardedUserIndex, UserIndex},
        CommonDatabase,
    },
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use uuid::Uuid;

use crate::env::{FRONTEND_URL, GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL};

pub mod bank_transfer;
pub mod cost_center;
pub mod entity_contractor_branch_pair;
pub mod individual_contractor_branch_pair;
pub mod pay_items;
pub mod sap;
pub mod search_employee_contractors;
pub mod teams;
use once_cell::sync::Lazy;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddUserRequest {
    pub email: EmailWrapper,
    pub client_ulid: Uuid,
    pub debug: Option<bool>,
}

pub async fn add_individual_contractor(
    // Only for validation
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<AddUserRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .is_some()
    {
        return Err(GlobeliseError::UnavailableEmail);
    };
    let default_password_raw =
        std::env::var("DEFAULT_USER_PASSWORD").expect("default password not set");
    let salt: [u8; 16] = rand::thread_rng().gen();

    let default_password_hash = hash_encoded(default_password_raw.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(GlobeliseError::internal)?;

    let contractor_ulid = database
        .insert_one_user(
            &body.email,
            Some(&default_password_hash),
            false,
            false,
            false,
            true,
            false,
            true,
        )
        .await?;

    // If  in debug mode, skip sending emails
    if let Some(true) = body.debug {
        return Ok(());
    }

    //create client contractor pair
    database
        .create_client_contractor_pair(body.client_ulid, contractor_ulid)
        .await?;

    let receiver_email = body
        .email
        .0
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
               Click the <a href="{}/signup?as=contractor&type=individual">link</a> to sign up as a Globelise individual contractor.
                </p>
                <br/> your password is {}. Please change your password immediatly.
                <p>If you did not expect to receive this email. Please ignore!</p>
            </body>
            </html>
            "##,
            (*FRONTEND_URL),
            (default_password_raw),
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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub user_type: Option<UserType>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_after: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_before: Option<sqlx::types::time::OffsetDateTime>,
}

pub async fn admin_get_many_user_index(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<GetManyUserIndexQuery>,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let database = database.lock().await;
    let result = database
        .select_many_user_index(
            query.page,
            query.per_page,
            query.query,
            query.user_type,
            query.created_after,
            query.created_before,
        )
        .await?;
    Ok(Json(result))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyOnboardedUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_after: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_before: Option<sqlx::types::time::OffsetDateTime>,
}

/// Lists all the users plus some information about them.
pub async fn admin_get_many_onboarded_user_index(
    _: Token<AdminAccessToken>,
    Query(query): Query<GetManyOnboardedUserIndexQuery>,
    Extension(shared_database): Extension<CommonDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = shared_database.lock().await;
    let result = database
        .select_many_onboarded_user_index(
            query.page,
            query.per_page,
            query.query,
            query.user_type,
            query.user_role,
            query.created_after,
            query.created_before,
        )
        .await?;
    Ok(Json(result))
}

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});
