use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    custom_serde::{ImageData, OffsetDateWrapper, UserRole},
    database::Database,
    error::GlobeliseResult,
};

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
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_one_onboard_entity_pic_details(
        &self,
        ulid: Uuid,
        role: UserRole,
        first_name: String,
        last_name: String,
        dob: sqlx::types::time::OffsetDateTime,
        dial_code: String,
        phone_number: String,
        profile_picture: Option<ImageData>,
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
            .bind(first_name)
            .bind(last_name)
            .bind(dob)
            .bind(dial_code)
            .bind(phone_number)
            .bind(profile_picture)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn select_one_onboard_entity_pic_details(
        &self,
        ulid: Uuid,
        role: UserRole,
    ) -> GlobeliseResult<Option<EntityPicDetails>> {
        let table = match role {
            UserRole::Client => "entity_client_pic_details",
            UserRole::Contractor => "entity_contractor_pic_details",
        };

        let query = format!(
            "
        SELECT
            *
        FROM
            {table}
        WHERE
            ulid = $1"
        );

        let result = sqlx::query_as(&query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
