use axum::extract::{ContentLengthLimit, Extension, Json, Path};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{
    token::AccessToken,
    user::{Role, UserType},
};

use crate::database::{Database, SharedDatabase};

pub async fn post_onboard_entity_pic_details(
    claims: Token<AccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityPicDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Path(role): Path<Role>,
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
    claims: Token<AccessToken>,
    Path(role): Path<Role>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityPicDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_entity_pic_details(claims.payload.ulid, role)
            .await?,
    ))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityPicDetails {
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    #[serde_as(as = "Option<Base64>")]
    pub profile_picture: Option<ImageData>,
}

impl FromRow<'_, PgRow> for EntityPicDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let maybe_profile_picture: Option<Vec<u8>> = row.try_get("profile_picture")?;
        Ok(EntityPicDetails {
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            dob: row.try_get("dob")?,
            dial_code: row.try_get("dial_code")?,
            phone_number: row.try_get("phone_number")?,
            profile_picture: maybe_profile_picture.map(ImageData),
        })
    }
}

impl Database {
    pub async fn post_onboard_entity_pic_details(
        &self,
        ulid: Ulid,
        role: Role,
        details: EntityPicDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let target_table = UserType::Entity.db_onboard_details_prefix(role) + "_pic_details";
        let query = format!(
            "
            INSERT INTO {target_table}
            (ulid, first_name, last_name, dob, dial_code, phone_number, profile_picture)
            VALUES ($7, $1, $2, $3, $4, $5, $6)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $1, last_name = $2, dob = $3, dial_code = $4, phone_number = $5,
            profile_picture = $6",
        );

        sqlx::query(&query)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(ulid_to_sql_uuid(ulid))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_onboard_entity_pic_details(
        &self,
        ulid: Ulid,
        role: Role,
    ) -> GlobeliseResult<EntityPicDetails> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let result = match role {
            Role::Client => {
                let query = "
                    SELECT
                        ulid, first_name, last_name, dob, dial_code,
                        phone_number, profile_picture
                    FROM
                        entity_contractors_pic_details
                    WHERE
                        ulid = $1";

                sqlx::query_as(query)
                    .bind(ulid_to_sql_uuid(ulid))
                    .fetch_one(&self.0)
                    .await
                    .map_err(|e| GlobeliseError::Database(e.to_string()))?
            }
            Role::Contractor => {
                let query = "
                    SELECT
                        ulid, first_name, last_name, dob, dial_code,
                        phone_number, profile_picture
                    FROM
                        entity_clients_pic_details
                    WHERE
                        ulid = $1";

                sqlx::query_as(query)
                    .bind(ulid_to_sql_uuid(ulid))
                    .fetch_one(&self.0)
                    .await
                    .map_err(|e| GlobeliseError::Database(e.to_string()))?
            }
        };

        Ok(result)
    }
}
