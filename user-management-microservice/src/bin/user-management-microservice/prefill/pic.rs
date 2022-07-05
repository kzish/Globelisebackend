use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{EmailWrapper, ImageData, OffsetDateWrapper, UserRole, FORM_DATA_LENGTH_LIMIT},
    database::CommonDatabase,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientPicDetails {
    pub email: EmailWrapper,
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

pub async fn admin_post_one_entity_client(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<CommonDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let ulid = database
        .find_one_user(None, Some(&body.email), None)
        .await?
        .ok_or_else(|| GlobeliseError::not_found("Cannot find a user with this email"))?
        .ulid;

    if database
        .select_one_onboard_entity_pic_details(ulid, UserRole::Client)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_entity_pic_details(
                &ulid,
                &UserRole::Client,
                &body.first_name,
                &body.last_name,
                &body.dob,
                &body.dial_code,
                &body.phone_number,
                body.profile_picture.as_ref(),
            )
            .await?;
    }

    Ok(())
}
