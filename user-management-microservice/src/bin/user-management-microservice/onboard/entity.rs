use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{Country, ImageData, OffsetDateWrapper, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
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
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

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
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

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
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientAccountDetails {
    pub company_name: String,
    pub country: Country,
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
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorAccountDetails {
    pub company_name: String,
    pub country: Country,
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

#[serde_as]
#[derive(Debug, Deserialize)]
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

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityDetails {
    pub company_name: String,
    pub country: Country,
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
        if self
            .find_one_user(ulid, Some(UserType::Entity))
            .await?
            .is_none()
        {
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
            .await?;

        Ok(())
    }

    pub async fn get_onboard_entity_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientAccountDetails> {
        if self
            .find_one_user(ulid, Some(UserType::Entity))
            .await?
            .is_none()
        {
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
        if self
            .find_one_user(ulid, Some(UserType::Entity))
            .await?
            .is_none()
        {
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
            .await?;

        Ok(())
    }

    pub async fn get_onboard_entity_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<EntityContractorAccountDetails>> {
        if self
            .find_one_user(ulid, Some(UserType::Entity))
            .await?
            .is_none()
        {
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
