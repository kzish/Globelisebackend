use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{ImageData, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::{
    token::UserAccessToken,
    user::{UserRole, UserType},
};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

pub async fn post_onboard_entity_pic_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Path(role): Path<UserRole>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database
        .post_onboard_entity_pic_details(claims.payload.ulid, role, request)
        .await
}

pub async fn get_onboard_entity_pic_details(
    claims: Token<UserAccessToken>,
    Path(role): Path<UserRole>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityPicDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_entity_pic_details(claims.payload.ulid, role)
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    ))
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityPicDetails {
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

impl Database {
    pub async fn post_onboard_entity_pic_details(
        &self,
        ulid: Uuid,
        role: UserRole,
        details: EntityPicDetails,
    ) -> GlobeliseResult<()> {
        let table = match role {
            UserRole::Client => "entity_client_pic_details",
            UserRole::Contractor => "entity_contractor_pic_details",
        };

        let query = format!(
            "
            INSERT INTO {table} (
                ulid, first_name, last_name, dob, dial_code, 
                phone_number, profile_picture
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7
            ) ON CONFLICT(ulid) DO UPDATE SET 
                first_name = $2, last_name = $3, dob = $4, dial_code = $5, 
                phone_number = $6, profile_picture = $7",
        );

        sqlx::query(&query)
            .bind(ulid)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.profile_picture)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_onboard_entity_pic_details(
        &self,
        ulid: Uuid,
        role: UserRole,
    ) -> GlobeliseResult<Option<EntityPicDetails>> {
        let result = match role {
            UserRole::Client => {
                let query = "
                    SELECT
                        ulid, first_name, last_name, dob, dial_code,
                        phone_number, profile_picture
                    FROM
                        entity_contractor_pic_details
                    WHERE
                        ulid = $1";

                sqlx::query_as(query)
                    .bind(ulid)
                    .fetch_optional(&self.0)
                    .await?
            }
            UserRole::Contractor => {
                let query = "
                    SELECT
                        ulid, first_name, last_name, dob, dial_code,
                        phone_number, profile_picture
                    FROM
                        entity_client_pic_details
                    WHERE
                        ulid = $1";

                sqlx::query_as(query)
                    .bind(ulid)
                    .fetch_optional(&self.0)
                    .await?
            }
        };

        Ok(result)
    }
}
