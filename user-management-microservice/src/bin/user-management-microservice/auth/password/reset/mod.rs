use argon2::{self, hash_encoded};
use axum::{
    extract::{ContentLengthLimit, Extension, Json},
    response::Redirect,
};
use common_utils::{
    custom_serde::{EmailWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
};
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;
use user_management_microservice_sdk::user::UserType;
use uuid::Uuid;

use crate::{
    auth::token::one_time::create_one_time_token,
    env::{
        FRONTEND_URL, GLOBELISE_SENDER_EMAIL, GLOBELISE_SMTP_URL, SMTP_CREDENTIAL,
        USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL,
    },
};

use crate::auth::{
    token::one_time::{OneTimeToken, OneTimeTokenBearer, OneTimeTokenParam},
    SharedDatabase, SharedState, HASH_CONFIG,
};

mod token;

use token::{ChangePasswordToken, LostPasswordToken};

/// Send email to the user with the steps to recover their password.
pub async fn send_email(
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<LostPasswordRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    let (user_ulid, user_type, is_valid_attempt) =
        match database.find_one_user(None, Some(&body.email), None).await {
            Ok(Some(user)) => {
                let user_type = user.user_type()?;
                (user.ulid, user_type, true)
            }
            _ => (Uuid::new_v4(), UserType::Individual, false),
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

    let receiver_email = body
        .email
        .0
        // TODO: Get the name of the person associated to this email address
        .to_display("")
        .parse()
        .map_err(GlobeliseError::bad_request)?;
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
                <a href="{}/auth/password/reset/initiate?token={}">link</a> to reset it.
                </p>
                <p>Otherwise, please report this occurence.</p>
            </body>
            </html>
            "##,
            (*USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL),
            one_time_token,
        ))
        .map_err(GlobeliseError::internal)?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&GLOBELISE_SMTP_URL)
        .map_err(GlobeliseError::internal)?
        .credentials(SMTP_CREDENTIAL.clone())
        .build();

    // Send the email
    if is_valid_attempt && created_valid_token {
        mailer.send(&email).map_err(GlobeliseError::internal)?;
    }

    Ok(())
}

/// Respond to user clicking the reset password link in their email.
pub async fn initiate(
    OneTimeTokenParam(claims): OneTimeTokenParam<OneTimeToken<LostPasswordToken>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<Redirect> {
    let mut shared_state = shared_state.lock().await;
    let database = database.lock().await;
    let change_password_token = shared_state
        .open_one_time_session::<ChangePasswordToken>(&database, claims.sub, claims.user_type)
        .await?;

    let redirect_url = format!(
        "{}/reset-password?token={}",
        (*FRONTEND_URL),
        change_password_token
    );
    Ok(Redirect::to(&redirect_url))
}

/// Replace the password for a user with the requested one.
pub async fn execute(
    Json(request): Json<ChangePasswordRequest>,
    OneTimeTokenBearer(claims): OneTimeTokenBearer<OneTimeToken<ChangePasswordToken>>,
    Extension(database): Extension<SharedDatabase>,
    Extension(shared_state): Extension<SharedState>,
) -> GlobeliseResult<()> {
    let new_password: String = request.new_password.nfc().collect();
    let confirm_new_password: String = request.new_password.nfc().collect();

    if new_password != confirm_new_password {
        return Err(GlobeliseError::bad_request("Passwords do not match"));
    }

    let database = database.lock().await;
    let mut shared_state = shared_state.lock().await;

    // NOTE: This is not atomic, so this check is quite pointless.
    // Either rely completely on SQL or use some kind of transaction commit.
    let salt: [u8; 16] = rand::thread_rng().gen();
    let hash = hash_encoded(new_password.as_bytes(), &salt, &HASH_CONFIG)
        .map_err(GlobeliseError::internal)?;

    database
        .update_user_password_hash(claims.sub, claims.user_type, Some(hash))
        .await?;
    shared_state.revoke_all_sessions(claims.sub).await?;

    Ok(())
}

/// Request for requesting password reset.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LostPasswordRequest {
    pub email: EmailWrapper,
}

/// Request to change password.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChangePasswordRequest {
    pub new_password: String,
    pub confirm_new_password: String,
}
