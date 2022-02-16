//! Endpoints for user authentication and authorization.

use std::str::FromStr;

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::{
    extract::{Extension, Form, OriginalUri, Path},
    http::{
        uri::{self, Builder as UriBuilder},
        Uri,
    },
    response::{Redirect, Response},
};
use email_address::EmailAddress;
use lettre::{
    transport::smtp::authentication::Credentials as SmtpCredentials, Message as EmailBuilder,
    SmtpTransport, Transport,
};
use once_cell::sync::Lazy;
use rand::Rng;
use rusty_ulid::Ulid;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

mod error;
pub mod google;
mod state;
mod token;
mod user;

use error::{Error, RegistrationError};
pub use state::{SharedState, State};
use token::{create_access_token, RefreshToken};
use user::{Role, User};

/// Creates an account.
pub async fn create_account(
    Form(request): Form<CreateAccountRequest>,
    Path(role): Path<Role>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();
    let confirm_password: String = request.confirm_password.nfc().collect();

    // Frontend validation can be bypassed, so perform basic validation
    // in the backend as well.
    let email = EmailAddress::from_str(&email);
    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::Unauthorized);
    }

    let is_valid_email = email.is_ok();
    let mut is_email_available = false;
    let is_password_at_least_8_chars = request.password.len() >= 8;
    let passwords_match = password == confirm_password;

    if is_valid_email {
        let mut shared_state = shared_state.lock().await;
        let email = email.unwrap();
        is_email_available = shared_state.user_id(&email, role).await?.is_none();

        if is_email_available && is_password_at_least_8_chars && passwords_match {
            let salt: [u8; 16] = rand::thread_rng().gen();
            let hash = hash_encoded(password.as_bytes(), &salt, &HASH_CONFIG)
                .map_err(|_| Error::Internal)?;

            let ulid = Ulid::generate();
            let user = User {
                email,
                password_hash: Some(hash),
                google: false,
                outlook: false,
            };
            shared_state.create_user(ulid, user, role).await?;

            let refresh_token = shared_state.open_session(ulid, role).await?;
            return Ok(refresh_token);
        }
    }

    Err(Error::Registration(RegistrationError {
        is_valid_email,
        is_email_available,
        is_password_at_least_8_chars,
        passwords_match,
    }))
}

/// Logs a user in.
pub async fn login(
    Form(request): Form<LoginRequest>,
    Path(role): Path<Role>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    let email: EmailAddress = request.email.parse().map_err(|_| Error::BadRequest)?;
    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::Unauthorized);
    }

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    let mut shared_state = shared_state.lock().await;
    if let Some(ulid) = shared_state.user_id(&email, role).await? {
        if let Some(User {
            password_hash: Some(hash),
            ..
        }) = shared_state.user(ulid, role).await?
        {
            if let Ok(true) = verify_encoded(&hash, request.password.as_bytes()) {
                let refresh_token = shared_state.open_session(ulid, role).await?;
                return Ok(refresh_token);
            }
        }
    }

    Err(Error::Unauthorized)
}

/// Logs a user in.
pub async fn lost_password(Form(request): Form<LostPasswordRequest>) -> Result<Redirect, Error> {
    let email_address: EmailAddress = request.email.parse().map_err(|_| Error::BadRequest)?;

    let sender_email =
        std::env::var("GLOBELISE_SENDER_EMAIL").expect("GLOBELISE_SENDER_EMAIL must be set");
    let smtp_username =
        std::env::var("GLOBELISE_SMTP_USERNAME").expect("GLOBELISE_SMTP_USERNAME must be set");
    let smtp_password =
        std::env::var("GLOBELISE_SMTP_PASSWORD").expect("GLOBELISE_SMTP_PASSWORD must be set");
    let domain_url =
        std::env::var("GLOBELISE_DOMAIN_URL").expect("GLOBELISE_DOMAIN_URL must be set");

    let email = EmailBuilder::builder()
        .from(sender_email.parse().unwrap())
        .reply_to(sender_email.parse().unwrap())
        // TODO: Get the name of the person associated to this email address
        // and prepend it like so `name <email>`
        .to(format!("<{}>", email_address.as_ref()).parse().unwrap())
        .subject("Confirm Request to Reset Password")
        .body(format!(
            "Are you sure you want to reset the password? Please follow this link to reset it",
        ))
        .unwrap();

    let creds = SmtpCredentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer
        .send(&email)
        .map_err(|e| Error::InternalVerbose(e.to_string()))?;

    // TODO: Better ways to create url for redirection?
    let redirect_url = format!("{}/auth/keys", domain_url);
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
pub async fn lost_password_page() -> axum::response::Html<String> {
    axum::response::Html(
        r##"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Globelise Password Recovery</title>
        </head>
        <body>
            <h2>Recover Password</h2>

            <form
            action="lostpassword"
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
        "##
        .to_string(),
    )
}

/// Logs a user in.
pub async fn change_password(
    Form(request): Form<ChangePasswordRequest>,
    Path(role): Path<Role>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    let email: EmailAddress = request.email.parse().map_err(|_| Error::BadRequest)?;
    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::Admin) {
        return Err(Error::Unauthorized);
    }

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    let mut shared_state = shared_state.lock().await;
    if let Some(ulid) = shared_state.user_id(&email, role).await? {
        if let Some(User {
            password_hash: Some(hash),
            ..
        }) = shared_state.user(ulid, role).await?
        {
            if let Ok(true) = verify_encoded(&hash, request.password.as_bytes()) {
                let refresh_token = shared_state.open_session(ulid, role).await?;
                return Ok(refresh_token);
            }
        }
    }

    Err(Error::Unauthorized)
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
pub async fn change_password_page() -> axum::response::Html<String> {
    axum::response::Html(
        r##"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Globelise Password Recovery</title>
        </head>
        <body>
            <h2>Recover Password</h2>

            <form
            action="lostpassword"
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
        "##
        .to_string(),
    )
}

/// Gets a new access token.
pub async fn renew_access_token(
    claims: RefreshToken,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    let ulid: Ulid = claims
        .sub
        .parse()
        .map_err(|_| Error::Conversion("uuid parse error".into()))?;
    let role: Role = claims
        .role
        .parse()
        .map_err(|_| Error::Conversion("role parse error".into()))?;

    let mut shared_state = shared_state.lock().await;
    if let Some(User { email, .. }) = shared_state.user(ulid, role).await? {
        let access_token = create_access_token(ulid, email, role)?;
        Ok(access_token)
    } else {
        Err(Error::Unauthorized)
    }
}

/// Gets the public key for decoding tokens.
pub async fn public_key() -> String {
    (*token::PUBLIC_KEY).clone()
}

/// Request for creating a user.
#[derive(Deserialize)]
pub struct CreateAccountRequest {
    email: String,
    password: String,
    confirm_password: String,
}

/// Request for logging a user in.
#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
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
    email: String,
    password: String,
    confirm_password: String,
}

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});
