use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use rusty_ulid::Ulid;
use serde::Deserialize;
use serde_with::{base64::Base64, serde_as, TryFromInto};

use crate::{
    auth::{
        token::AccessToken,
        user::{Role, UserType},
    },
    database::SharedDatabase,
};

use super::util::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT};

pub async fn account_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let user_type: UserType = claims.payload.user_type.parse::<UserType>().unwrap();
    if !matches!(user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let ulid: Ulid = claims.payload.ulid.parse().unwrap();

    let database = database.lock().await;
    database.onboard_entity_details(ulid, role, request).await
}

pub async fn pic_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<Json<PicDetails>, FORM_DATA_LENGTH_LIMIT>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let user_type = claims.payload.user_type.parse::<UserType>().unwrap();
    if !matches!(user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let ulid: Ulid = claims.payload.ulid.parse().unwrap();

    let database = database.lock().await;
    database.onboard_pic_details(ulid, role, request).await
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityDetails {
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    #[serde(default)]
    pub registration_number: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PicDetails {
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}
