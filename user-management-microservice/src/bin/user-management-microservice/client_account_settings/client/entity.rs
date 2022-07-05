use crate::database::{Database, SharedDatabase};
use axum::extract::Query;
use axum::{extract::Extension, Json};
use common_utils::custom_serde::{Country, ImageData};
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
pub struct EntityClientPicDetails {
    pub ulid: Uuid,
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
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientPicDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientAccountDetails {
    pub ulid: Uuid,
    pub company_name: String,
    pub country: Country,
    pub entity_type: String,
    pub registration_number: String,
    pub tax_id: String,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientAccountDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientBranchAccountDetails {
    pub ulid: Uuid,
    pub branch_name: String,
    pub country: Country,
    pub entity_type: String,
    pub registration_number: String,
    pub tax_id: String,
    pub statutory_contribution_submission_number: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientBranchAccountDetailsRequest {
    pub branch_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientBranchBankDetails {
    pub ulid: Uuid,
    pub currency: String,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub swift_code: String,
    pub bank_key: String,
    pub iban: String,
    pub bank_code: String,
    pub branch_code: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientBranchBankDetailsRequest {
    pub branch_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientBranchPayrollDetails {
    pub ulid: Uuid,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub cutoff_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub payment_date: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientBranchPayrollDetailsRequest {
    pub branch_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityClientPaymentDetails {
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
pub struct EntityClientPaymentDetailsRequest {
    pub ulid: Uuid,
}

//
//######### methods #########
//

//EntityClientPicDetails
pub async fn get_entity_client_pic_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientPicDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_pic_details(claims.payload.ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientPicDetails
pub async fn update_entity_client_pic_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientPicDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database.update_entity_client_pic_details(request).await?;

    Ok(())
}
//EntityClientPicDetails
pub async fn delete_entity_client_pic_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientPicDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database.delete_entity_client_pic_details(request).await?;

    Ok(())
}

//EntityClientAccountDetails
pub async fn get_entity_client_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_account_details(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != response.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}
//EntityClientAccountDetails
pub async fn update_entity_client_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_client_account_details(request)
        .await?;

    Ok(())
}
//EntityClientAccountDetails
pub async fn delete_entity_client_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_entity_client_account_details(request)
        .await?;

    Ok(())
}

//EntityClientBranchAccountDetails
pub async fn get_entity_client_branch_account_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityClientBranchAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientBranchAccountDetails>> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_entity_client_branch_account_details(request.branch_ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientBranchAccountDetails
pub async fn update_entity_client_branch_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientBranchAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_client_branch_account_details(request)
        .await?;

    Ok(())
}
//EntityClientBranchAccountDetails
pub async fn delete_entity_client_branch_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientBranchAccountDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .delete_entity_client_branch_account_details(request)
        .await?;

    Ok(())
}

//EntityClientBranchBankDetails
pub async fn get_entity_client_branch_bank_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityClientBranchBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientBranchBankDetails>> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_entity_client_branch_bank_details(request.branch_ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientBranchBankDetails
pub async fn update_entity_client_branch_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientBranchBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_client_branch_bank_details(request)
        .await?;

    Ok(())
}
//EntityClientBranchBankDetails
pub async fn delete_entity_client_branch_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientBranchBankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_entity_client_branch_bank_details(request)
        .await?;

    Ok(())
}

//EntityClientBranchPayrollDetails
pub async fn get_entity_client_branch_payroll_details(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityClientBranchPayrollDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientBranchPayrollDetails>> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_entity_client_branch_payroll_details(request.branch_ulid)
        .await?;

    Ok(Json(response))
}
//EntityClientBranchPayrollDetails
pub async fn update_entity_client_branch_payroll_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientBranchPayrollDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_client_branch_payroll_details(request)
        .await?;

    Ok(())
}
//EntityClientBranchPayrollDetails
pub async fn delete_entity_client_branch_payroll_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientBranchPayrollDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_entity_client_branch_payroll_details(request)
        .await?;

    Ok(())
}

//EntityClientPaymentDetails
pub async fn get_entity_client_payment_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityClientPaymentDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_client_payment_details(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != response.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}
//EntityClientPaymentDetails
pub async fn update_entity_client_payment_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientPaymentDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_client_payment_details(request)
        .await?;

    Ok(())
}
//EntityClientPaymentDetails
pub async fn delete_entity_client_payment_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityClientPaymentDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_entity_client_payment_details(request)
        .await?;

    Ok(())
}
//
//######### DB #########
//

impl Database {
    //EntityClientPicDetails
    pub async fn get_entity_client_pic_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientPicDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_client_pic_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityClientPicDetails
    pub async fn update_entity_client_pic_details(
        &self,
        request: EntityClientPicDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                entity_client_pic_details(
                    ulid, 
                    first_name, 
                    last_name, 
                    dob, 
                    dial_code, 
                    phone_number, 
                    profile_picture
                )
                VALUES($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT(ulid) DO UPDATE
            SET
                first_name = $2,
                last_name = $3,
                dob = $4,
                dial_code = $5,
                phone_number = $6,
                profile_picture = $7",
        )
        .bind(request.ulid)
        .bind(request.first_name)
        .bind(request.last_name)
        .bind(request.dob)
        .bind(request.dial_code)
        .bind(request.phone_number)
        .bind(request.profile_picture.map(|b| b.as_ref().to_owned()))
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityClientPicDetails
    pub async fn delete_entity_client_pic_details(
        &self,
        request: EntityClientPicDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                entity_client_pic_details
            WHERE
            ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityClientAccountDetails
    pub async fn get_entity_client_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientAccountDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_client_account_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityClientAccountDetails
    pub async fn update_entity_client_account_details(
        &self,
        request: EntityClientAccountDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
            entity_client_account_details(
                ulid, 
                company_name,
                country,
                entity_type,
                registration_number,
                tax_id,
                company_address,
                city,
                postal_code,
                time_zone,
                logo)
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT(ulid) DO UPDATE
            SET
                company_name = $2,
                country = $3,
                entity_type = $4,
                registration_number = $5,
                tax_id = $6,
                company_address = $7,
                city = $8,
                postal_code = $9,
                time_zone = $10,
                logo = $11",
        )
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
        .bind(request.logo.map(|b| b.as_ref().to_owned()))
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityClientAccountDetails
    pub async fn delete_entity_client_account_details(
        &self,
        request: EntityClientAccountDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                entity_client_account_details
            WHERE
                ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityClientBranchAccountDetails
    pub async fn get_entity_client_branch_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientBranchAccountDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_client_branch_account_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityClientBranchAccountDetails
    pub async fn update_entity_client_branch_account_details(
        &self,
        request: EntityClientBranchAccountDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO 
            entity_client_branch_account_details(
                ulid,
                branch_name,
                country,
                entity_type,
                registration_number,
                tax_id,
                statutory_contribution_submission_number,
                company_address,
                city,
                postal_code,
                time_zone,
                logo
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT(ulid) DO UPDATE
            SET
                branch_name = $2,
                country = $3,
                entity_type = $4,
                registration_number = $5,
                tax_id = $6,
                statutory_contribution_submission_number = $7,
                company_address = $8,
                city = $9,
                postal_code = $10,
                time_zone = $11,
                logo = $12",
        )
        .bind(request.ulid)
        .bind(request.branch_name)
        .bind(request.country)
        .bind(request.entity_type)
        .bind(request.registration_number)
        .bind(request.tax_id)
        .bind(request.statutory_contribution_submission_number)
        .bind(request.company_address)
        .bind(request.city)
        .bind(request.postal_code)
        .bind(request.time_zone)
        .bind(request.logo)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityClientBranchAccountDetails
    pub async fn delete_entity_client_branch_account_details(
        &self,
        request: EntityClientBranchAccountDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                entity_client_branch_account_details
            WHERE 
                ulid = $1",
        )
        .bind(request.branch_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityClientBranchBankDetails
    pub async fn get_entity_client_branch_bank_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientBranchBankDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_client_branch_bank_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityClientBranchBankDetails
    pub async fn update_entity_client_branch_bank_details(
        &self,
        request: EntityClientBranchBankDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO 
            entity_client_branch_bank_details(
                ulid,
                currency,
                bank_name,
                bank_account_name,
                bank_account_number,
                swift_code,
                bank_key,
                iban,
                bank_code,
                branch_code
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT(ulid) DO UPDATE
            SET
                currency = $2,
                bank_name = $3,
                bank_account_name = $4,
                bank_account_number = $5,
                swift_code = $6,
                bank_key = $7,
                iban = $8,
                bank_code = $9,
                branch_code = $10",
        )
        .bind(request.ulid)
        .bind(request.currency)
        .bind(request.bank_name)
        .bind(request.bank_account_name)
        .bind(request.bank_account_number)
        .bind(request.swift_code)
        .bind(request.bank_key)
        .bind(request.iban)
        .bind(request.bank_code)
        .bind(request.branch_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityClientBranchBankDetails
    pub async fn delete_entity_client_branch_bank_details(
        &self,
        request: EntityClientBranchBankDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM 
                entity_client_branch_bank_details
            WHERE
                ulid = $1",
        )
        .bind(request.branch_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityClientBranchPayrollDetails
    pub async fn get_entity_client_branch_payroll_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientBranchPayrollDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_client_branch_payroll_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityClientBranchPayrollDetails
    pub async fn update_entity_client_branch_payroll_details(
        &self,
        request: EntityClientBranchPayrollDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO 
            entity_client_branch_payroll_details(
                ulid,
                cutoff_date,
                payment_date
            )
            VALUES($1, $2, $3)
            ON CONFLICT(ulid) DO UPDATE
            SET
                cutoff_date = $2,
                payment_date = $3",
        )
        .bind(request.ulid)
        .bind(request.cutoff_date)
        .bind(request.payment_date)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityClientBranchPayrollDetails
    pub async fn delete_entity_client_branch_payroll_details(
        &self,
        request: EntityClientBranchPayrollDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                entity_client_branch_payroll_details
                WHERE 
                ulid = $1",
        )
        .bind(request.branch_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityClientPaymentDetails
    pub async fn get_entity_client_payment_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityClientPaymentDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_client_payment_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityClientPaymentDetails
    pub async fn update_entity_client_payment_details(
        &self,
        request: EntityClientPaymentDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO 
            entity_client_payment_details (
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
    //EntityClientPaymentDetails
    pub async fn delete_entity_client_payment_details(
        &self,
        request: EntityClientPaymentDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                entity_client_payment_details 
            WHERE 
                ulid = $1",
        )
        .bind(request.ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
