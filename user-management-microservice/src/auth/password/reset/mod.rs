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

use crate::{
    auth::token::one_time::create_one_time_token,
    env::{
        GLOBELISE_DOMAIN_URL, GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, PASSWORD_RESET_URL,
        SMTP_CREDENTIAL,
    },
    error::Error,
};

use crate::auth::{
    token::one_time::{OneTimeToken, OneTimeTokenBearer, OneTimeTokenParam},
    user::UserType,
    SharedDatabase, SharedState, HASH_CONFIG,
};

mod token;

use token::{ChangePasswordToken, LostPasswordToken};

/// Send email to the user with the steps to recover their password.
pub async fn send_email(
    Form(request): Form<LostPasswordRequest>,
    Path(user_type): Path<UserType>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<(), Error> {
    let email_address: EmailAddress = request
        .email
        .parse()
        .map_err(|_| Error::BadRequest("Not a valid email address"))?;

    let database = database.lock().await;
    let (user_ulid, is_valid_attempt) = match database.user_id(&email_address, user_type).await {
        Ok(Some(ulid)) => (ulid, true),
        _ => (Ulid::generate(), false),
    };

    let mut shared_state = shared_state.lock().await;
    let (one_time_token, created_valid_token) = match shared_state
        .open_one_time_session::<LostPasswordToken>(&database, user_ulid, user_type)
        .await
    {
        Ok(token) => (token, true),
        Err(_) => {
            let (fake_token, _) = create_one_time_token::<LostPasswordToken>(user_ulid, user_type)?;
            (fake_token, false)
        }
    };

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
                <a href="{}/auth/password/reset/initiate/{}?token={}">link</a> to reset it.
                </p>
                <p>Otherwise, please report this occurence.</p>
            </body>
            </html>
            "##,
            (*GLOBELISE_DOMAIN_URL),
            user_type,
            one_time_token
        ))
        .map_err(|_| Error::Internal("Could not create email for changing password".into()))?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)
        .map_err(|_| Error::Internal("Could not connect to SMTP server".into()))?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    if is_valid_attempt && created_valid_token {
        mailer
            .send(&email)
            .map_err(|e| Error::Internal(e.to_string()))?;
    }

    Ok(())
}

/// Respond to user clicking the reset password link in their email.
pub async fn initiate(
    OneTimeTokenParam(claims): OneTimeTokenParam<OneTimeToken<LostPasswordToken>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Redirect, Error> {
    let ulid: Ulid = claims.sub.parse().unwrap();
    let user_type: UserType = claims.user_type.parse().unwrap();

    let mut shared_state = shared_state.lock().await;
    let database = database.lock().await;
    let change_password_token = shared_state
        .open_one_time_session::<ChangePasswordToken>(&database, ulid, user_type)
        .await?;

    let redirect_url = format!("{}?token={}", (*PASSWORD_RESET_URL), change_password_token);
    let uri = Uri::from_str(redirect_url.as_str()).unwrap();
    Ok(Redirect::to(uri))
}

/// Replace the password for a user with the requested one.
pub async fn execute(
    Form(request): Form<ChangePasswordRequest>,
    OneTimeTokenBearer(claims): OneTimeTokenBearer<OneTimeToken<ChangePasswordToken>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<(), Error> {
    let ulid: Ulid = claims.sub.parse().unwrap();
    let user_type: UserType = claims.user_type.parse().unwrap();

    if request.new_password != request.confirm_new_password {
        return Err(Error::BadRequest("Passwords do not match"));
    }

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;

    // NOTE: This is not atomic, so this check is quite pointless.
    // Either rely completely on SQL or use some kind of transaction commit.
    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash = hash_encoded(request.new_password.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(|_| Error::Internal("Failed to hash password".into()))?;

    database
        .update_password(ulid, user_type, Some(hash))
        .await?;
    shared_state.revoke_all_sessions(ulid).await?;

    Ok(())
}

/// Request for requesting password reset.
#[derive(Debug, Deserialize)]
pub struct LostPasswordRequest {
    pub email: String,
}

/// Request to change password.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChangePasswordRequest {
    pub new_password: String,
    pub confirm_new_password: String,
}
