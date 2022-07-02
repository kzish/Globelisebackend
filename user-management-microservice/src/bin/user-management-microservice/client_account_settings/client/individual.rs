use crate::database::{Database, SharedDatabase};
use axum::{extract::Extension, Json};
use common_utils::custom_serde::ImageData;
use common_utils::{
    custom_serde::OffsetDateWrapper,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::{serde_as, TryFromInto};
use sqlx::{types::Uuid, FromRow};
use user_management_microservice_sdk::token::UserAccessToken;
//
//######### models #########
//
#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientAccountDetails {
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
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientAccountDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientAccountDetailsDeleteRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientPaymentDetails {
    pub ulid: Uuid,
    pub currency: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientPaymentDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualClientPaymentDetailsDeleteRequest {
    pub ulid: Uuid,
}

//
//######### methods #########
//

//IndividualClientAccountDetails
pub async fn get_individual_client_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualClientAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_individual_client_account_details(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != response.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}
//IndividualClientAccountDetails
pub async fn update_individual_client_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualClientAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_individual_client_account_details(request)
        .await?;

    Ok(())
}
//IndividualClientAccountDetails
pub async fn delete_individual_client_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualClientAccountDetailsDeleteRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_individual_client_account_details(request)
        .await?;

    Ok(())
}

//IndividualClientPaymentDetails
pub async fn get_individual_client_payment_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualClientPaymentDetails>> {
    let database = database.lock().await;

    let response = database
        .get_individual_client_payment_details(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != response.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}
//IndividualClientPaymentDetails
pub async fn update_individual_client_payment_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualClientPaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_individual_client_payment_details(request)
        .await?;

    Ok(())
}
//IndividualClientPaymentDetails
pub async fn delete_individual_client_payment_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualClientPaymentDetailsDeleteRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_individual_client_payment_details(request)
        .await?;

    Ok(())
}

//######### DB #########
//

impl Database {
    //IndividualClientAccountDetails
    pub async fn get_individual_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<IndividualClientAccountDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                individual_client_account_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //IndividualClientAccountDetails
    pub async fn update_individual_client_account_details(
        &self,
        request: IndividualClientAccountDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                individual_client_account_details(
                    ulid,
                    first_name,
                    last_name,
                    dob,
                    dial_code,
                    phone_number,
                    country,
                    city,
                    address,
                    postal_code,
                    tax_id,
                    time_zone,
                    profile_picture
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                ON CONFLICT(ulid) DO UPDATE
            SET
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
                profile_picture = $13",
        )
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
        .bind(request.profile_picture.map(|b| b.as_ref().to_owned()))
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //IndividualClientAccountDetails
    pub async fn delete_individual_client_account_details(
        &self,
        request: IndividualClientAccountDetailsDeleteRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                individual_client_account_details
            WHERE
                ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //IndividualClientPaymentDetails
    pub async fn get_individual_client_payment_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<IndividualClientPaymentDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                individual_client_payment_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //IndividualClientPaymentDetails
    pub async fn update_individual_client_payment_details(
        &self,
        request: IndividualClientPaymentDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                individual_client_payment_details(
                    ulid,
                    currency,
                    payment_date,
                    cutoff_date
                )
                VALUES($1, $2, $3, $4)
                ON CONFLICT(ulid) DO UPDATE
            SET
                currency = $2,
                payment_date = $3,
                cutoff_date = $4",
        )
        .bind(request.ulid)
        .bind(request.currency)
        .bind(request.payment_date)
        .bind(request.cutoff_date)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //IndividualClientPaymentDetails
    pub async fn delete_individual_client_payment_details(
        &self,
        request: IndividualClientPaymentDetailsDeleteRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                individual_client_payment_details
            WHERE
                ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
