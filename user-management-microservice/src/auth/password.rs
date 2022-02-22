use std::str::FromStr;

use argon2::{self, hash_encoded};
use axum::{
    extract::{Extension, Form, Path},
    http::Uri,
    response::Redirect,
};
use email_address::EmailAddress;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use rusty_ulid::Ulid;
use serde::Deserialize;

use crate::env::{
    GLOBELISE_DOMAIN_URL, GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
};

use super::{
    error::Error,
    token::{
        change_password::ChangePasswordToken, lost_password::LostPasswordToken,
        one_time::OneTimeToken,
    },
    user::Role,
    SharedDatabase, SharedState, HASH_CONFIG,
};

/// Send email to the user with the steps to recover their password.
pub async fn lost_password(
    Form(request): Form<LostPasswordRequest>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<(), Error> {
    let email_address: EmailAddress = request
        .email
        .parse()
        .map_err(|_| Error::BadRequest("Bad request"))?;

    // TODO: Do not reveal correct email and role.
    let database = database.lock().await;
    let user_ulid = database
        .user_id(&email_address, role)
        .await?
        .ok_or(Error::BadRequest("Bad request"))?;

    let mut shared_state = shared_state.lock().await;
    let access_token = shared_state
        .open_one_time_session::<LostPasswordToken>(&database, user_ulid, role)
        .await?;

    let receiver_email = email_address
        // TODO: Get the name of the person associated to this email address
        .to_display("")
        .parse()
        .map_err(|_| Error::BadRequest("Bad request"))?;
    let email = Message::builder()
        .from(GLOBELISE_SENDER_EMAIL.clone())
        .reply_to(GLOBELISE_SENDER_EMAIL.clone())
        .to(receiver_email)
        .subject("Confirm Request to Reset Password")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        // TODO: Once designer have a template for this. Use a templating library to populate data.
        .body(format!(
            r##"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Change Password Request</title>
            </head>
            <body>
                <p>
                If you requested to change your password, please follow this
                <a href="{}/changepasswordredirect/{}?token={}">link</a> to reset it.
                </p>
                <p>Otherwise, please report this occurence.</p>
            </body>
            </html>
"##,
            (*GLOBELISE_DOMAIN_URL),
            role,
            access_token
        ))
        .map_err(|_| Error::Internal("Could not create email for changing password".into()))?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)
        .map_err(|_| Error::Internal("Could not connect to SMTP server".into()))?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    mailer
        .send(&email)
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(())
}

// Respond to user clicking the reset password link in their email.
pub async fn change_password_redirect(
    Path(role): Path<Role>,
    claims: OneTimeToken<LostPasswordToken>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Redirect, Error> {
    let ulid: Ulid = claims.sub.parse().unwrap();

    // Make sure the user actually exists.
    let mut shared_state = shared_state.lock().await;
    let database = database.lock().await;

    let change_password_token = shared_state
        .open_one_time_session::<ChangePasswordToken>(&database, ulid, role)
        .await?;

    let redirect_url = format!(
        "{}/changepasswordpage/{}?token={}",
        (*GLOBELISE_DOMAIN_URL),
        role,
        change_password_token
    );
    let uri = Uri::from_str(redirect_url.as_str()).unwrap();
    Ok(Redirect::to(uri))
}

/// Replace the password for a user with the requested one.
pub async fn change_password(
    Form(request): Form<ChangePasswordRequest>,
    Path(role): Path<Role>,
    claims: OneTimeToken<ChangePasswordToken>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<(), Error> {
    let ulid: Ulid = claims.sub.parse().unwrap();

    if request.password != request.confirm_password {
        return Err(Error::BadRequest("Bad request"));
    }

    let database = database.lock().await;

    // NOTE: This is not atomic, so this check is quite pointless.
    // Either rely completely on SQL or use some kind of transaction commit.
    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash = hash_encoded(request.password.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(|_| Error::Internal("Failed to hash password".into()))?;

    database.update_password(ulid, role, Some(hash)).await?;

    Ok(())
}

/// Request for requesting password reset.
#[derive(Debug, Deserialize)]
pub struct LostPasswordRequest {
    #[serde(rename(deserialize = "user_email"))]
    pub email: String,
}

/// Request to change password.
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    #[serde(rename(deserialize = "new_password"))]
    pub password: String,
    #[serde(rename(deserialize = "confirm_new_password"))]
    pub confirm_password: String,
}
