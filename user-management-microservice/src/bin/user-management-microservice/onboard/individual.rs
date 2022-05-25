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

pub async fn post_onboard_individual_client_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<IndividualClientAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let ulid = claims.payload.ulid;
    let full_name = format!("{} {}", body.first_name, body.last_name);

    let database = database.lock().await;
    database
        .post_onboard_individual_client_account_details(ulid, body)
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot insert individual client onboard data into the database because \n{:#?}",
                e
            ))
        })?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Client(ulid, full_name))
        .await
        .map_err(|e| {
            GlobeliseError::internal(format!(
                "Cannot publish individual client user name change event to DAPR because \n{:#?}",
                e
            ))
        })?;

    Ok(())
}

pub async fn get_onboard_individual_client_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualClientAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_individual_client_account_details(claims.payload.ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    ))
}

pub async fn post_onboard_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<IndividualContractorAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
    Extension(pubsub): Extension<SharedPubSub>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }
    let ulid = claims.payload.ulid;
    let full_name = format!("{} {}", request.first_name, request.last_name);

    let database = database.lock().await;
    database
        .post_onboard_individual_contractor_account_details(ulid, request)
        .await?;

    let pubsub = pubsub.lock().await;
    pubsub
        .publish_event(UpdateUserName::Contractor(ulid, full_name))
        .await?;

    Ok(())
}

pub async fn get_onboard_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Individual) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    Ok(Json(
        database
            .get_onboard_individual_contractor_account_details(claims.payload.ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    ))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientAccountDetails {
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}

impl FromRow<'_, PgRow> for IndividualClientAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let meybe_profile_picture: Option<Vec<u8>> = row.try_get("profile_picture")?;
        Ok(IndividualClientAccountDetails {
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            dob: row.try_get("dob")?,
            dial_code: row.try_get("dial_code")?,
            phone_number: row.try_get("phone_number")?,
            country: row.try_get("country")?,
            city: row.try_get("city")?,
            address: row.try_get("address")?,
            postal_code: row.try_get("postal_code")?,
            tax_id: row.try_get("tax_id")?,
            time_zone: row.try_get("time_zone")?,
            profile_picture: meybe_profile_picture.map(ImageData),
        })
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorAccountDetails {
    pub first_name: String,
    pub last_name: String,
    #[serde_as(as = "TryFromInto<DateWrapper>")]
    pub dob: sqlx::types::time::Date,
    pub dial_code: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub cv: Option<Vec<u8>>,
}

impl FromRow<'_, PgRow> for IndividualContractorAccountDetails {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let meybe_profile_picture: Option<Vec<u8>> = row.try_get("profile_picture")?;
        Ok(IndividualContractorAccountDetails {
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            dob: row.try_get("dob")?,
            dial_code: row.try_get("dial_code")?,
            phone_number: row.try_get("phone_number")?,
            country: row.try_get("country")?,
            city: row.try_get("city")?,
            address: row.try_get("address")?,
            postal_code: row.try_get("postal_code")?,
            tax_id: row.try_get("tax_id")?,
            time_zone: row.try_get("time_zone")?,
            profile_picture: meybe_profile_picture.map(ImageData),
            cv: row.try_get("cv")?,
        })
    }
}

impl Database {
    pub async fn post_onboard_individual_client_account_details(
        &self,
        ulid: Uuid,
        details: IndividualClientAccountDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO individual_clients_account_details
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $2, last_name = $3, dob = $4, dial_code = $5, phone_number = $6,
            country = $7, city = $8, address = $9, postal_code = $10, tax_id = $11,
            time_zone = $12, profile_picture = $13";
        sqlx::query(query)
            .bind(ulid)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.country)
            .bind(details.city)
            .bind(details.address)
            .bind(details.postal_code)
            .bind(details.tax_id)
            .bind(details.time_zone)
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_onboard_individual_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<IndividualClientAccountDetails>> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            SELECT 
                ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
                postal_code, tax_id, time_zone, profile_picture
            FROM
                individual_clients_account_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }

    pub async fn post_onboard_individual_contractor_account_details(
        &self,
        ulid: Uuid,
        details: IndividualContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            INSERT INTO individual_contractors_account_details
            (ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
            postal_code, tax_id, time_zone, profile_picture, cv) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT(ulid) DO UPDATE SET 
            first_name = $2, last_name = $3, dob = $4, dial_code = $5, phone_number = $6,
            country = $7, city = $8, address = $9, postal_code = $10, tax_id = $11,
            time_zone = $12, profile_picture = $13, cv = $14";

        sqlx::query(query)
            .bind(ulid)
            .bind(details.first_name)
            .bind(details.last_name)
            .bind(details.dob)
            .bind(details.dial_code)
            .bind(details.phone_number)
            .bind(details.country)
            .bind(details.city)
            .bind(details.address)
            .bind(details.postal_code)
            .bind(details.tax_id)
            .bind(details.time_zone)
            .bind(details.profile_picture.map(|b| b.as_ref().to_owned()))
            .bind(details.cv)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_onboard_individual_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<IndividualContractorAccountDetails>> {
        if self.user(ulid, Some(UserType::Individual)).await?.is_none() {
            return Err(GlobeliseError::Forbidden);
        }

        let query = "
            SELECT
                ulid, first_name, last_name, dob, dial_code, phone_number, country, city, address,
                postal_code, tax_id, time_zone, profile_picture, cv
            FROM
                individual_contractors_account_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}
