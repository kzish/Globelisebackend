use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{DateWrapper, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    pubsub::{SharedPubSub, UpdateUserName},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

pub async fn post_onboard_entity_client_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityClientAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let full_name = request.company_name.clone();

    let database = database.lock().await;
    database
        .post_onboard_entity_client_account_details(claims.payload.ulid, request)
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot insert entity client onboard data into the database because \n{:#?}",
                e
            ))
        })?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Client(claims.payload.ulid, full_name))
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot publish entity client user name change event to DAPR because \n{:#?}",
                e
            ))
        })?;

    Ok(())
}

pub async fn get_onboard_entity_client_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_entity_client_account_details(claims.payload.ulid)
            .await?,
    ))
}

pub async fn post_onboard_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<EntityContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }
    let full_name = request.company_name.clone();

    let database = database.lock().await;
    database
        .post_onboard_entity_contractor_account_details(claims.payload.ulid, request)
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot insert entity client onboard data into the database because \n{:#?}",
                e
            ))
        })?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Contractor(claims.payload.ulid, full_name))
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot publish entity client user name change event to DAPR because \n{:#?}",
                e
            ))
        })?;

    Ok(())
}

pub async fn get_onboard_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_entity_contractor_account_details(claims.payload.ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    ))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientAccountDetails {
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

impl FromRow<'_, PgRow> for EntityClientAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let maybe_logo: Option<Vec<u8>> = row.try_get("logo")?;
        Ok(EntityClientAccountDetails {
            company_name: row.try_get("company_name")?,
            country: row.try_get("country")?,
            entity_type: row.try_get("entity_type")?,
            registration_number: row.try_get("registration_number")?,
            tax_id: row.try_get("tax_id")?,
            company_address: row.try_get("company_address")?,
            city: row.try_get("city")?,
            postal_code: row.try_get("postal_code")?,
            time_zone: row.try_get("time_zone")?,
            logo: maybe_logo.map(ImageData),
        })
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorAccountDetails {
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
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

impl FromRow<'_, PgRow> for EntityContractorAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let maybe_logo: Option<Vec<u8>> = row.try_get("logo")?;
        Ok(EntityContractorAccountDetails {
            company_name: row.try_get("company_name")?,
            country: row.try_get("country")?,
            entity_type: row.try_get("entity_type")?,
            registration_number: row.try_get("registration_number")?,
            tax_id: row.try_get("tax_id")?,
            company_address: row.try_get("company_address")?,
            city: row.try_get("city")?,
            postal_code: row.try_get("postal_code")?,
            time_zone: row.try_get("time_zone")?,
            logo: maybe_logo.map(ImageData),
            company_profile: row.try_get("profile_picture")?,
        })
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityPicDetails {
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
pub struct EntityClientDetails {
    pub common_info: EntityDetails,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

impl Database {
    pub async fn post_onboard_entity_client_account_details(
        &self,
        ulid: Uuid,
        details: EntityClientAccountDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO entity_clients_account_details
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $2, country = $3, entity_type = $4, registration_number = $5,
            tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
            logo = $11";
        sqlx::query(query)
            .bind(ulid)
            .bind(details.company_name)
            .bind(details.country)
            .bind(details.entity_type)
            .bind(details.registration_number)
            .bind(details.tax_id)
            .bind(details.company_address)
            .bind(details.city)
            .bind(details.postal_code)
            .bind(details.time_zone)
            .bind(details.logo.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_onboard_entity_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientAccountDetails> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            SELECT
                ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
                city, postal_code, time_zone, logo
            FROM
                entity_clients_account_details 
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?
            .ok_or(GlobeliseError::NotFound)?;

        Ok(result)
    }

    pub async fn post_onboard_entity_contractor_account_details(
        &self,
        ulid: Uuid,
        details: EntityContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO entity_contractors_account_details
            (ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
            city, postal_code, time_zone, logo, company_profile)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE SET 
            company_name = $2, country = $3, entity_type = $4, registration_number = $5,
            tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
            logo = $11, company_profile = $12";
        sqlx::query(query)
            .bind(ulid)
            .bind(details.company_name)
            .bind(details.country)
            .bind(details.entity_type)
            .bind(details.registration_number)
            .bind(details.tax_id)
            .bind(details.company_address)
            .bind(details.city)
            .bind(details.postal_code)
            .bind(details.time_zone)
            .bind(details.logo.map(|b| b.as_ref().to_owned()))
            .bind(details.company_profile)
            .execute(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_onboard_entity_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<EntityContractorAccountDetails>> {
        if self.user(ulid, Some(UserType::Entity)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            SELECT
                ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
                city, postal_code, time_zone, logo, company_profile
            FROM
                entity_contractors_account_details
            WHERE
                ulid = $1";
        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
