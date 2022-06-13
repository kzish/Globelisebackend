use axum::{extract::Extension, Json};
use common_utils::{
    custom_serde::{EmailWrapper, ImageData, OffsetDateWrapper},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, TryFromInto};
use sqlx::FromRow;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorProfileSettingsRequest {
    pub ulid: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    pub tax_id: String,
    pub time_zone: String,
    pub gender: String,
    pub marital_status: String,
    pub nationality: String,
    pub email_address: EmailWrapper,
    pub national_id: String,
    pub passport_number: String,
    pub work_permit: bool,
    pub added_related_pay_item_id: Uuid,
    pub total_dependants: i64,
    pub passport_expiry_date: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
    pub cv: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorProfileSettingsResponse {
    pub ulid: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    pub tax_id: String,
    pub time_zone: String,
    pub gender: String,
    pub marital_status: String,
    pub nationality: String,
    pub email_address: EmailWrapper,
    pub national_id: String,
    pub passport_number: String,
    pub work_permit: bool,
    pub added_related_pay_item_id: Uuid,
    pub total_dependants: i64,
    pub passport_expiry_date: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
    pub cv: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorProfileSettingsRequest {
    pub ulid: Uuid,
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    pub registration_number: String,
    pub tax_id: String,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    pub email_address: EmailWrapper,
    pub added_related_pay_item_id: Uuid,
    pub total_dependants: i64,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorProfileSettingsResponse {
    pub ulid: Uuid,
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    pub registration_number: String,
    pub tax_id: String,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    pub email_address: EmailWrapper,
    pub added_related_pay_item_id: Uuid,
    pub total_dependants: i64,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

pub async fn get_profile_settings_entity(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorProfileSettingsResponse>> {
    let database = database.lock().await;

    let profile = database
        .get_profile_settings_entity(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != profile.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(profile))
}

pub async fn post_profile_settings_entity(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorProfileSettingsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database.post_profile_settings_entity(request).await?;

    Ok(())
}

pub async fn get_profile_settings_individual(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorProfileSettingsResponse>> {
    let database = database.lock().await;

    let profile = database
        .get_profile_settings_individual(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != profile.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(profile))
}

pub async fn post_profile_settings_individual(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorProfileSettingsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database.post_profile_settings_individual(request).await?;

    Ok(())
}

impl Database {
    pub async fn get_profile_settings_entity(
        &self,
        uuid: Uuid,
    ) -> GlobeliseResult<EntityContractorProfileSettingsResponse> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                entity_contractor_account_details 
            WHERE ulid = $1",
        )
        .bind(uuid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_profile_settings_entity(
        &self,
        request: EntityContractorProfileSettingsRequest,
    ) -> GlobeliseResult<()> {
        let query = "INSERT INTO 
                            entity_contractor_account_details 
                                    (ulid, company_name, country, entity_type, registration_number, tax_id, company_address, city,
                                     postal_code, time_zone, email_address, added_related_pay_item_id, total_dependants, logo, company_profile) 
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                            ON CONFLICT(ulid) DO UPDATE SET 
                            company_name = $2, 
                            country = $3, 
                            entity_type = $4, 
                            registration_number = $5, 
                            tax_id = $6,
                            company_address = $7, 
                            city = $8, 
                            postal_code = $9, 
                            time_zone = $10,
                            email_address = $11,
                            added_related_pay_item_id = $12,
                            total_dependants = $13,
                            logo = $14, 
                            company_profile = $15";

        sqlx::query(query)
            .bind(request.ulid)
            .bind(request.company_name)
            .bind(request.country)
            .bind(request.entity_type)
            .bind(request.registration_number)
            .bind(request.tax_id)
            .bind(request.company_address)
            .bind(request.city)
            .bind(request.postal_code)
            .bind(request.time_zone)
            .bind(request.email_address)
            .bind(request.added_related_pay_item_id)
            .bind(request.total_dependants)
            .bind(request.logo.map(|b| b.as_ref().to_owned()))
            .bind(request.company_profile)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_profile_settings_individual(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<IndividualContractorProfileSettingsResponse> {
        let response = sqlx::query_as(
            "SELECT *
                FROM 
                    individual_contractor_account_details 
                WHERE ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_profile_settings_individual(
        &self,
        request: IndividualContractorProfileSettingsRequest,
    ) -> GlobeliseResult<()> {
        let query = "INSERT INTO 
                            individual_contractor_account_details 
                                    (ulid, first_name, last_name, dob, dial_code, phone_number, country, 
                                    city, address, postal_code, tax_id, time_zone, gender, marital_status,
                                    nationality, email_address, national_id, passport_number, work_permit,
                                    added_related_pay_item_id, total_dependants, passport_expiry_date,
                                    profile_picture, cv) 
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
                            ON CONFLICT(ulid) DO UPDATE SET 
                            first_name = $2, 
                            last_name = $3, 
                            dob = $4, 
                            dial_code = $5, 
                            phone_number = $6,
                            country = $7, 
                            city = $8, 
                            address = $9, 
                            postal_code = $10, 
                            tax_id = $11,
                            time_zone = $12, 
                            gender = $13,
                            marital_status = $14,
                            nationality = $15,
                            email_address = $16,
                            national_id = $17,
                            passport_number = $18,
                            work_permit = $19,
                            added_related_pay_item_id = $20,
                            total_dependants = $21,
                            passport_expiry_date = $22,
                            profile_picture = $23,
                            cv = $24";
        sqlx::query(query)
            .bind(request.ulid)
            .bind(request.first_name)
            .bind(request.last_name)
            .bind(request.dob)
            .bind(request.dial_code)
            .bind(request.phone_number)
            .bind(request.country)
            .bind(request.city)
            .bind(request.address)
            .bind(request.postal_code)
            .bind(request.tax_id)
            .bind(request.time_zone)
            .bind(request.gender)
            .bind(request.marital_status)
            .bind(request.nationality)
            .bind(request.email_address)
            .bind(request.national_id)
            .bind(request.passport_number)
            .bind(request.work_permit)
            .bind(request.added_related_pay_item_id)
            .bind(request.total_dependants)
            .bind(request.passport_expiry_date)
            .bind(request.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(request.cv)
            .execute(&self.0)
            .await?;

        Ok(())
    }
}
