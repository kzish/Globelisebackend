use axum::extract::{Extension, Json, Query};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::OptionOffsetDateWrapper,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use user_management_microservice_sdk::{
    user::{UserRole, UserType},
    user_index::{OnboardedUserIndex, UserIndex},
};

use crate::{
    database::{Database, SharedDatabase},
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
pub mod sap;
pub mod search_employee_contractors;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnboardedUserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    pub user_role: Option<UserRole>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_after: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_before: Option<sqlx::types::time::OffsetDateTime>,
}

pub async fn eor_admin_onboarded_user_index(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<OnboardedUserIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<OnboardedUserIndex>>> {
    let database = database.lock().await;
    let result = database
        .onboarded_user_index(
            query.page,
            query.per_page,
            query.search_text,
            query.user_type,
            query.user_role,
            query.created_after,
            query.created_before,
        )
        .await?;
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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserIndexQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
    pub user_type: Option<UserType>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_after: Option<sqlx::types::time::OffsetDateTime>,
    #[serde(default)]
    #[serde_as(as = "TryFromInto<OptionOffsetDateWrapper>")]
    pub created_before: Option<sqlx::types::time::OffsetDateTime>,
}

pub async fn eor_admin_user_index(
    // Only for validation
    _: Token<AdminAccessToken>,
    Query(query): Query<UserIndexQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<UserIndex>>> {
    let database = database.lock().await;
    let result = database
        .user_index(
            query.page,
            query.per_page,
            query.search_text,
            query.user_type,
            query.created_after,
            query.created_before,
        )
        .await?;
    Ok(Json(result))
}

impl Database {
    /// Index users (client and contractors)
    ///
    /// Currently, the search functionality only works on the name.
    /// For entities, this is the company's name.
    /// For individuals, this is a concat of their first and last name.
    #[allow(clippy::too_many_arguments)]
    pub async fn onboarded_user_index(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        search_text: Option<String>,
        user_type: Option<UserType>,
        user_role: Option<UserRole>,
        created_after: Option<sqlx::types::time::OffsetDateTime>,
        created_before: Option<sqlx::types::time::OffsetDateTime>,
    ) -> GlobeliseResult<Vec<OnboardedUserIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT
                ulid,
                name,
                email,
                user_role,
                user_type,
                created_at
            FROM 
                onboarded_user_index 
            WHERE
                ($1 IS NULL OR name ~* $1) AND
                ($2 IS NULL OR user_role = $2) AND
                ($3 IS NULL OR user_type = $3) AND
                ($4 IS NULL OR created_at > $4) AND
                ($5 IS NULL OR created_at < $5)
            LIMIT
                $6
            OFFSET
                $7",
        )
        .bind(search_text)
        .bind(user_role)
        .bind(user_type)
        .bind(created_after)
        .bind(created_before)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;
        Ok(result)
    }

    /// Index users ULID and email
    ///
    /// This does not require users to be fully onboarded.
    pub async fn user_index(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        search_text: Option<String>,
        user_type: Option<UserType>,
        created_after: Option<sqlx::types::time::OffsetDateTime>,
        created_before: Option<sqlx::types::time::OffsetDateTime>,
    ) -> GlobeliseResult<Vec<UserIndex>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
            SELECT 
                ulid,
                email,
                user_type,
                is_google,
                is_outlook,
                created_at
            FROM 
                users_index 
            WHERE
                ($1 IS NULL OR email ~* $1) AND
                ($2 IS NULL OR user_type = $2) AND
                ($3 IS NULL OR created_at > $3) AND
                ($4 IS NULL OR created_at < $4)
            LIMIT
                $5
            OFFSET
                $6",
        )
        .bind(search_text)
        .bind(user_type)
        .bind(created_after)
        .bind(created_before)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }
}
