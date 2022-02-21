//! Endpoints for user authentication and authorization.

use std::{collections::HashMap, str::FromStr};

use argon2::{self, hash_encoded, verify_encoded, Config};
use axum::{
    extract::{Extension, Form, Path, Query},
    http::Uri,
    response::Redirect,
};
use email_address::EmailAddress;
use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use lettre::{message::Message, SmtpTransport, Transport};
use once_cell::sync::Lazy;
use rand::Rng;
use rusty_ulid::{DecodingError, Ulid};
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

mod database;
mod error;
pub mod google;
pub mod onboarding;
pub mod password;
mod state;
mod token;
mod user;

pub use database::{Database, SharedDatabase};
use error::{Error, RegistrationError};
use token::{create_access_token, RefreshToken};
use user::{Role, User};

pub use state::{SharedState, State};

use crate::{
    auth::token::{one_time::OneTimeToken, ISSSUER, KEYS},
    env::{GLOBELISE_DOMAIN_URL, GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL},
};

use self::{
    password::{ChangePasswordRequest, LostPasswordRequest},
    token::{
        change_password::ChangePasswordToken, lost_password::LostPasswordToken,
        one_time::OneTimeTokenAudience,
    },
};

/// Creates an account.
pub async fn create_account(
    Form(request): Form<CreateAccountRequest>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();
    let confirm_password: String = request.confirm_password.nfc().collect();

    // Frontend validation can be bypassed, so perform basic validation
    // in the backend as well.
    let email = EmailAddress::from_str(&email);

    let is_valid_email = email.is_ok();
    let is_password_at_least_8_chars = request.password.len() >= 8;
    let passwords_match = password == confirm_password;

    if is_valid_email {
        let database = database.lock().await;
        let email = email.unwrap();

        if is_password_at_least_8_chars && passwords_match {
            let salt: [u8; 16] = rand::thread_rng().gen();
            let hash = hash_encoded(password.as_bytes(), &salt, &HASH_CONFIG)
                .map_err(|_| Error::Internal)?;

            let user = User {
                email,
                password_hash: Some(hash),
                google: false,
                outlook: false,
            };
            let ulid = database.create_user(user, role).await?;

            let mut shared_state = shared_state.lock().await;
            let refresh_token = shared_state.open_session(&database, ulid, role).await?;
            return Ok(refresh_token);
        }
    }

    Err(Error::Registration(RegistrationError {
        is_valid_email,
        is_password_at_least_8_chars,
        passwords_match,
    }))
}

/// Logs a user in.
pub async fn login(
    Form(request): Form<LoginRequest>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<String, Error> {
    // Credentials should be normalized for maximum compatibility.
    let email: String = request.email.trim().nfc().collect();
    let password: String = request.password.nfc().collect();

    let email: EmailAddress = email.parse().map_err(|_| Error::BadRequest)?;

    // NOTE: A timing attack can detect registered emails.
    // Mitigating this is not strictly necessary, as attackers can still find out
    // if an email is registered by using the sign-up page.
    let database = database.lock().await;
    if let Some(ulid) = database.user_id(&email, role).await? {
        if let Some((
            User {
                password_hash: Some(hash),
                ..
            },
            _,
        )) = database.user(ulid, Some(role)).await?
        {
            if let Ok(true) = verify_encoded(&hash, password.as_bytes()) {
                let mut shared_state = shared_state.lock().await;
                let refresh_token = shared_state.open_session(&database, ulid, role).await?;
                return Ok(refresh_token);
            }
        }
    }

    Err(Error::Unauthorized)
}

/// Send email to the user with the steps to recover their password.
pub async fn lost_password(
    Form(request): Form<LostPasswordRequest>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<(), Error> {
    let email_address: EmailAddress = request.email.parse().map_err(|_| Error::BadRequest)?;

    let database = database.lock().await;
    let user_ulid = database
        .user_id(&email_address, role)
        .await?
        .ok_or(Error::BadRequest)?;

    let mut shared_state = shared_state.lock().await;
    let access_token = shared_state
        .open_one_time_session::<LostPasswordToken>(&database, user_ulid, role)
        .await?;

    let receiver_email = email_address
        // TODO: Get the name of the person associated to this email address
        .to_display("")
        .parse()
        .map_err(|_| Error::BadRequest)?;
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
        .map_err(|_| Error::Internal)?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)
        .map_err(|_| Error::Internal)?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    mailer
        .send(&email)
        .map_err(|e| Error::InternalVerbose(e.to_string()))?;

    Ok(())
}

// Respond to user clicking the reset password link in their email.
pub async fn change_password_redirect(
    Path(role): Path<Role>,
    Query(params): Query<HashMap<String, String>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<Redirect, Error> {
    // TODO: Reimplement using FromRequest which does the validation etc.
    let token = params.get("token").ok_or(Error::BadRequest)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[LostPasswordToken::name()]);
    validation.set_issuer(&[ISSSUER]);
    validation.set_required_spec_claims(&["aud", "iss", "exp"]);
    let validation = validation;

    let TokenData { claims, .. } =
        decode::<OneTimeToken<LostPasswordToken>>(token, &KEYS.decoding, &validation)
            .map_err(|e| Error::UnauthorizedVerbose(e.to_string()))?;
    let ulid: Ulid = claims
        .sub
        .parse()
        .map_err(|e: DecodingError| Error::UnauthorizedVerbose(e.to_string()))?;

    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::EorAdmin) {
        return Err(Error::Unauthorized);
    }

    // Make sure the user actually exists.
    let mut shared_state = shared_state.lock().await;
    let database = database.lock().await;

    // Do not authorize if the token has already been used.
    if !shared_state
        .is_one_time_token_valid::<LostPasswordToken>(ulid, token.as_bytes())
        .await?
    {
        return Err(Error::UnauthorizedVerbose(
            "Invalid lost password token used".to_string(),
        ));
    }

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
    Query(params): Query<HashMap<String, String>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<(), Error> {
    // TODO: Reimplement using FromRequest which does the validation etc.
    let token = params.get("token").ok_or(Error::BadRequest)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[ChangePasswordToken::name()]);
    validation.set_issuer(&[ISSSUER]);
    validation.set_required_spec_claims(&["aud", "iss", "exp"]);
    let validation = validation;

    let TokenData { claims, .. } =
        decode::<OneTimeToken<ChangePasswordToken>>(token, &KEYS.decoding, &validation)
            .map_err(|_| Error::Unauthorized)?;
    let ulid: Ulid = claims
        .sub
        .parse()
        .map_err(|e: DecodingError| Error::UnauthorizedVerbose(e.to_string()))?;

    // NOTE: Admin sign up disabled until we figure out how to restrict access.
    if matches!(role, Role::EorAdmin) {
        return Err(Error::Unauthorized);
    }
    if request.password != request.confirm_password {
        return Err(Error::BadRequest);
    }

    // Make sure the user actually exists.
    let mut shared_state = shared_state.lock().await;
    let database = database.lock().await;

    // Do not authorize if the token has already been used.
    if !shared_state
        .is_one_time_token_valid::<ChangePasswordToken>(ulid, token.as_bytes())
        .await?
    {
        return Err(Error::Unauthorized);
    }

    // NOTE: This is not atomic, so this check is quite pointless.
    // Either rely completely on SQL or use some kind of transaction commit.
    if database.user(ulid, Some(role)).await?.is_some() {
        let salt: [u8; 16] = rand::thread_rng().gen();
        let hash = hash_encoded(request.password.as_bytes(), &salt, &HASH_CONFIG)
            .map_err(|_| Error::Internal)?;

        database.update_password(ulid, role, Some(hash)).await?;

        Ok(())
    } else {
        Err(Error::BadRequest)
    }
}

/// Gets a new access token.
pub async fn renew_access_token(
    claims: RefreshToken,
    Extension(database): Extension<SharedDatabase>,
) -> Result<String, Error> {
    let ulid: Ulid = claims
        .sub
        .parse()
        .map_err(|_| Error::Conversion("uuid parse error".into()))?;
    let role: Role = claims
        .role
        .parse()
        .map_err(|_| Error::Conversion("role parse error".into()))?;

    let database = database.lock().await;
    if let Some((User { email, .. }, _)) = database.user(ulid, Some(role)).await? {
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

/// The parameters used for hashing.
// TODO: Calibrate hash parameters for production server.
pub static HASH_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: argon2::Variant::Argon2id,
    ..Default::default()
});
