use std::collections::HashMap;
use std::str::FromStr;

use argon2::{self, hash_encoded};
use axum::{
    extract::{Extension, Form, Path, Query},
    http::Uri,
    response::Redirect,
};
use email_address::EmailAddress;
use lettre::{Message as EmailBuilder, SmtpTransport, Transport};
use rand::Rng;
use rusty_ulid::{DecodingError, Ulid};
use serde::Deserialize;
use time::Duration;

use crate::env::{GLOBELISE_SENDER_EMAIL, LISTENING_ADDRESS, SMTP_CREDENTIAL};

use super::{
    error::Error,
    token::one_time::{OneTimeToken, OneTimeTokenAudience},
    user::Role,
    SharedDatabase, SharedState, HASH_CONFIG,
};

/// Logs a user in.
pub async fn lost_password(
    Form(request): Form<LostPasswordRequest>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Redirect, Error> {
    let email_address: EmailAddress = request.email.parse().map_err(|_| Error::BadRequest)?;

    let database = database.lock().await;
    let user_ulid = database
        .user_id(&email_address, role)
        .await?
        .ok_or(Error::BadRequest)?;

    let mut shared_state = shared_state.lock().await;
    let access_token = shared_state
        .open_one_time_session::<ChangePasswordToken>(&database, user_ulid, role)
        .await?;

    let email = EmailBuilder::builder()
        .from(GLOBELISE_SENDER_EMAIL.parse().unwrap())
        .reply_to(GLOBELISE_SENDER_EMAIL.parse().unwrap())
        // TODO: Get the name of the person associated to this email address
        // and prepend it like so `name <email>`
        .to(format!("<{}>", email_address.as_ref()).parse().unwrap())
        .subject("Confirm Request to Reset Password")
        .header(lettre::message::header::ContentType::TEXT_HTML)
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
      <a href="http://localhost:3000/changepasswordpage/{}?token={}">link</a> to reset it.
    </p>
    <p>Otherwise, please report this occurence.</p>
  </body>
</html>"##,
            role, access_token
        ))
        .unwrap();

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    mailer
        .send(&email)
        .map_err(|e| Error::InternalVerbose(e.to_string()))?;

    // TODO: Better ways to create url for redirection?
    let redirect_url = format!("{}/auth/keys", (*LISTENING_ADDRESS));
    let uri = Uri::from_str(redirect_url.as_str()).unwrap();

    Ok(Redirect::to(uri))
}

#[cfg(not(debug_assertions))]
// Use absolute namespace to silence errors about unused imports.
pub async fn lost_password_page() -> axum::response::Response {
    axum::response::IntoResponse::into_response((
        axum::http::StatusCode::NOT_FOUND,
        "Not Found".to_string(),
    ))
}

#[cfg(debug_assertions)]
// Use absolute namespace to silence errors about unused imports.
pub async fn lost_password_page(Path(role): Path<Role>) -> axum::response::Html<String> {
    return axum::response::Html(format!(
        r##"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Globelise Password Recovery</title>
        </head>
        <body>
            <h2>Recover Password</h2>

            <form
            action="http://localhost:3000/lostpassword/{}"
            method="post"
            enctype="application/x-www-form-urlencoded"
            >
            <label for="user_email">Email:</label><br />
            <input
                type="email"
                id="user_email"
                name="user_email"
                placeholder="example@email.com"
            /><br />
            <input type="submit" value="Submit" />
            </form>
        </body>
        </html>           
        "##,
        role
    ));
}

/// Logs a user in.
pub async fn change_password(
    Form(request): Form<ChangePasswordRequest>,
    Path(role): Path<Role>,
    claims: OneTimeToken<ChangePasswordToken>,
    Extension(database): Extension<SharedDatabase>,
) -> Result<Redirect, Error> {
    let ulid: Ulid = claims
        .sub
        .parse()
        .map_err(|e: DecodingError| Error::UnauthorizedVerbose(e.to_string()))?;

    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::Unauthorized);
    }
    if request.password != request.confirm_password {
        return Err(Error::BadRequest);
    }

    let database = database.lock().await;
    if database.user(ulid, Some(role)).await?.is_some() {
        let salt: [u8; 16] = rand::thread_rng().gen();
        let hash = hash_encoded(request.password.as_bytes(), &salt, &HASH_CONFIG)
            .map_err(|_| Error::Internal)?;

        database.update_password(ulid, role, Some(hash)).await?;

        let redirect_url = format!("{}/auth/keys", (*LISTENING_ADDRESS));
        let uri = Uri::from_str(redirect_url.as_str()).unwrap();
        Ok(Redirect::to(uri))
    } else {
        Err(Error::BadRequest)
    }
}

#[cfg(not(debug_assertions))]
// Use absolute namespace to silence errors about unused imports.
pub async fn change_password_page() -> axum::response::Response {
    axum::response::IntoResponse::into_response((
        axum::http::StatusCode::NOT_FOUND,
        "Not Found".to_string(),
    ))
}

#[cfg(debug_assertions)]
// Use absolute namespace to silence errors about unused imports.
pub async fn change_password_page(
    Path(role): Path<Role>,
    Query(params): Query<HashMap<String, String>>,
) -> axum::response::Html<String> {
    let token = params.get("token").unwrap();
    axum::response::Html(format!(
        r##"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Globelise Password Change</title>
        </head>
        <body>
            <h2>Change Password</h2>

            <form
            action="http://localhost:3000/changepassword/{}?token={}"
            method="post"
            enctype="application/x-www-form-urlencoded"
            >
            <label for="new_password">Password:</label><br />
            <input type="password" id="new_password" name="new_password" /><br />
            <label for="confirm_new_password">Confirm Password:</label><br />
            <input
                type="password"
                id="confirm_new_password"
                name="confirm_new_password"
            /><br />
            <input type="submit" value="Submit" />
            </form>
        </body>
        </html>      
        "##,
        role, token
    ))
}

/// Request for requesting password reset.
#[derive(Debug, Deserialize)]
pub struct LostPasswordRequest {
    #[serde(rename(deserialize = "user_email"))]
    email: String,
}

/// Request to change password.
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    #[serde(rename(deserialize = "new_password"))]
    password: String,
    #[serde(rename(deserialize = "confirm_new_password"))]
    confirm_password: String,
}

#[derive(Debug)]
pub struct ChangePasswordToken;

impl OneTimeTokenAudience for ChangePasswordToken {
    fn name() -> &'static str {
        "change_password"
    }

    fn from_str(s: &str) -> Result<(), Error> {
        match s {
            "change_password" => Ok(()),
            _ => Err(Error::Unauthorized),
        }
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
