use crate::database::Database;
use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{EmailWrapper, ImageData, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};

use email_address::EmailAddress;
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};

use crate::database::SharedDatabase;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientPicDetails {
    #[serde_as(as = "TryFromInto<EmailWrapper>")]
    pub email: EmailAddress,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}

pub async fn entity_client_post_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database.prefill_entity_client_pic_details(body).await?;
    Ok(())
}

pub async fn entity_client_get_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    database.prefill_entity_client_pic_details(body).await?;
    Ok(())
}

impl Database {
    pub async fn prefill_entity_client_pic_details(
        &self,
        details: PrefillEntityClientPicDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO prefilled_entity_clients_pic_details
            (email, first_name, last_name, dob, dial_code, phone_number, profile_picture)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(email) DO UPDATE SET 
            first_name = $2, last_name = $3, dob = $4, dial_code = $5, phone_number = $6,
            profile_picture = $7";

        sqlx::query(query)
            .bind(details.email.to_string())
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }
}
