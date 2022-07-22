use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{Currency, EmailWrapper, OffsetDateWrapper, UserType, FORM_DATA_LENGTH_LIMIT},
    database::CommonDatabase,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::FromRow;

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PrefillEntityClientPaymentDetails {
    pub email: EmailWrapper,
    pub currency: Currency,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

pub async fn admin_post_one_entity_client(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PrefillEntityClientPaymentDetails>,
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
        .select_one_onboard_client_payment_details(ulid, UserType::Entity)
        .await?
        .is_none()
    {
        database
            .insert_one_onboard_client_payment_details(
                ulid,
                UserType::Entity,
                body.currency,
                &body.payment_date,
                &body.cutoff_date,
            )
            .await?;
    }

    Ok(())
}
