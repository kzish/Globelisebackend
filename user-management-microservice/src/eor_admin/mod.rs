use std::num::NonZeroU32;

use axum::extract::{Extension, Json, Query};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::AccessToken as AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use time::{format_description, OffsetDateTime};

use crate::{
    auth::user::{Role, UserType},
    database::{ulid_from_sql_uuid, SharedDatabase},
    env::{
        GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
        USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
    },
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
