use crate::database::{Database, SharedDatabase};
use axum::extract::Query;
use axum::{extract::Extension, Json};
use common_utils::custom_serde::{EmailWrapper, ImageData};
use common_utils::error::GlobeliseError;
use common_utils::{custom_serde::OffsetDateWrapper, error::GlobeliseResult, token::Token};
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
pub struct EntityContractorAccountDetails {
    pub ulid: Uuid,
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    pub email_address: Option<EmailWrapper>,
    pub added_related_pay_item_id: Option<Uuid>,
    pub total_dependants: Option<i64>,

    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,

    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub company_profile: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorAccountDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorBankDetails {
    pub ulid: Uuid,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorBankDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorEmployementInformation {
    pub contractor_uuid: Uuid,
    pub team_uuid: Uuid,
    pub designation: String,
    pub employment_status: bool,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub start_date: sqlx::types::time::OffsetDateTime,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub end_date: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorPayrollInformation {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
    pub monthly_basic_salary_amount: f64,
    pub monthly_added_pay_items_for_addition_section: f64,
    pub monthly_added_pay_items_for_deduction_section: f64,
    pub monthly_added_pay_items_for_statement_only_section: f64,
    pub monthly_added_pay_items_for_employers_contribution_section: f64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorPayrollInformationRequest {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorPicDetails {
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
pub struct EntityContractorPicDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct EntityContractorEmployementInformationRequest {
    pub contractor_uuid: Uuid,
    pub team_uuid: Uuid,
}

//
//######### methods #########
//

//EntityContractorAccountDetails
pub async fn get_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_contractor_account_details(claims.payload.ulid)
        .await?;

    Ok(Json(response))
}
//EntityContractorAccountDetails
pub async fn update_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_contractor_account_details(request)
        .await?;

    Ok(())
}
//EntityContractorAccountDetails
pub async fn delete_entity_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_contractor_account_details(claims.payload.ulid)
        .await?;

    Ok(())
}

//EntityContractorEmployementInformation
pub async fn get_entity_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorEmployementInformation>> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_uuid {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_entity_contractor_employment_information(request)
        .await?;

    Ok(Json(response))
}
//EntityContractorEmployementInformation
pub async fn update_entity_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorEmployementInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_uuid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_entity_contractor_employment_information(request)
        .await?;

    Ok(())
}
//EntityContractorEmployementInformation
pub async fn delete_entity_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_uuid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_entity_contractor_employment_information(request)
        .await?;

    Ok(())
}

//EntityContractorPayrollInformation
pub async fn get_entity_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Query(request): Query<EntityContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorPayrollInformation>> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_entity_contractor_payroll_information(request)
        .await?;

    Ok(Json(response))
}
//EntityContractorPayrollInformation
pub async fn update_entity_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPayrollInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_entity_contractor_payroll_information(request)
        .await?;

    Ok(())
}
//EntityContractorPayrollInformation
pub async fn delete_entity_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .delete_entity_contractor_payroll_information(request)
        .await?;

    Ok(())
}

//EntityContractorPicDetails
pub async fn get_entity_contractor_pic_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorPicDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_contractor_pic_details(claims.payload.ulid)
        .await?;

    Ok(Json(response))
}
//EntityContractorPicDetails
pub async fn update_entity_contractor_pic_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorPicDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_entity_contractor_pic_details(request)
        .await?;

    Ok(())
}
//EntityContractorPicDetails
pub async fn delete_entity_contractor_pic_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_contractor_pic_details(claims.payload.ulid)
        .await?;

    Ok(())
}

//EntityContractorBankDetails
pub async fn get_entity_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EntityContractorBankDetails>> {
    let database = database.lock().await;

    let response = database
        .get_entity_contractor_bank_details(claims.payload.ulid)
        .await?;

    Ok(Json(response))
}
//EntityContractorBankDetails
pub async fn update_entity_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<EntityContractorBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;
    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }
    database
        .update_entity_contractor_bank_details(request)
        .await?;

    Ok(())
}
//EntityContractorBankDetails
pub async fn delete_entity_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_entity_contractor_bank_details(claims.payload.ulid)
        .await?;

    Ok(())
}

//
//######### DB #########
//

impl Database {
    //EntityContractorAccountDetails
    pub async fn get_entity_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityContractorAccountDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_contractor_account_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityContractorAccountDetails
    pub async fn update_entity_contractor_account_details(
        &self,
        request: EntityContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                entity_contractor_account_details(
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
                    email_address,
                    added_related_pay_item_id,
                    total_dependants,
                    logo,
                    company_profile
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
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
                email_address = $11,
                added_related_pay_item_id = $12,
                total_dependants = $13,
                logo = $14,
                company_profile = $15",
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
        .bind(request.email_address)
        .bind(request.added_related_pay_item_id)
        .bind(request.total_dependants)
        .bind(request.logo.map(|b| b.as_ref().to_owned()))
        .bind(request.company_profile)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityContractorAccountDetails
    pub async fn delete_entity_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                entity_contractor_account_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityContractorEmployementInformation
    pub async fn get_entity_contractor_employment_information(
        &self,
        request: EntityContractorEmployementInformationRequest,
    ) -> GlobeliseResult<EntityContractorEmployementInformation> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_contractor_employment_information
            WHERE
                contractor_uuid = $1
            AND
                team_uuid = $2",
        )
        .bind(request.contractor_uuid)
        .bind(request.team_uuid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityContractorEmployementInformation
    pub async fn update_entity_contractor_employment_information(
        &self,
        request: EntityContractorEmployementInformation,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                entity_contractor_employment_information(
                    contractor_uuid,
                    team_uuid,
                    designation,
                    employment_status,
                    start_date,
                    end_date
                )
                VALUES($1, $2, $3, $4, $5, $6)
                ON CONFLICT(contractor_uuid, team_uuid) DO UPDATE
            SET
                designation = $3,
                employment_status = $4,
                start_date = $5,
                end_date = $6",
        )
        .bind(request.contractor_uuid)
        .bind(request.team_uuid)
        .bind(request.designation)
        .bind(request.employment_status)
        .bind(request.start_date)
        .bind(request.end_date)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityContractorEmployementInformation
    pub async fn delete_entity_contractor_employment_information(
        &self,
        request: EntityContractorEmployementInformationRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                entity_contractor_employment_information
            WHERE
                contractor_uuid = $1
            AND
                team_uuid = $2",
        )
        .bind(request.contractor_uuid)
        .bind(request.team_uuid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityContractorPayrollInformation
    pub async fn get_entity_contractor_payroll_information(
        &self,
        request: EntityContractorPayrollInformationRequest,
    ) -> GlobeliseResult<EntityContractorPayrollInformation> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_contractor_payroll_information
            WHERE
                contractor_ulid = $1
            AND
                client_ulid = $2",
        )
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityContractorPayrollInformation
    pub async fn update_entity_contractor_payroll_information(
        &self,
        request: EntityContractorPayrollInformation,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                entity_contractor_payroll_information(
                    contractor_ulid,
                    client_ulid,
                    monthly_basic_salary_amount,
                    monthly_added_pay_items_for_addition_section,
                    monthly_added_pay_items_for_deduction_section,
                    monthly_added_pay_items_for_statement_only_section,
                    monthly_added_pay_items_for_employers_contribution_section
                )
                VALUES($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT(contractor_ulid, client_ulid) DO UPDATE
            SET
                monthly_basic_salary_amount = $3,
                monthly_added_pay_items_for_addition_section = $4,
                monthly_added_pay_items_for_deduction_section = $5,
                monthly_added_pay_items_for_statement_only_section = $6,
                monthly_added_pay_items_for_employers_contribution_section = $7",
        )
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .bind(request.monthly_basic_salary_amount)
        .bind(request.monthly_added_pay_items_for_addition_section)
        .bind(request.monthly_added_pay_items_for_deduction_section)
        .bind(request.monthly_added_pay_items_for_statement_only_section)
        .bind(request.monthly_added_pay_items_for_employers_contribution_section)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityContractorPayrollInformation
    pub async fn delete_entity_contractor_payroll_information(
        &self,
        request: EntityContractorPayrollInformationRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                entity_contractor_payroll_information
            WHERE
                contractor_ulid = $1
            AND
                client_ulid = $2",
        )
        .bind(request.contractor_ulid)
        .bind(request.client_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityContractorPicDetails
    pub async fn get_entity_contractor_pic_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityContractorPicDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_contractor_pic_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityContractorPicDetails
    pub async fn update_entity_contractor_pic_details(
        &self,
        request: EntityContractorPicDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                entity_contractor_pic_details(
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
        .bind(request.profile_picture)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityContractorPicDetails
    pub async fn delete_entity_contractor_pic_details(&self, ulid: Uuid) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                entity_contractor_pic_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //EntityContractorBankDetails
    pub async fn get_entity_contractor_bank_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<EntityContractorBankDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                entity_contractor_bank_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //EntityContractorBankDetails
    pub async fn update_entity_contractor_bank_details(
        &self,
        request: EntityContractorBankDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                entity_contractor_bank_details(
                    ulid,
                    bank_name,
                    bank_account_name,
                    bank_account_number,
                    bank_code,
                    branch_code
                )
                VALUES($1, $2, $3, $4, $5, $6)
                ON CONFLICT(ulid) DO UPDATE
            SET
                bank_name = $2,
                bank_account_name = $3,
                bank_account_number = $4,
                bank_code = $5,
                branch_code = $6",
        )
        .bind(request.ulid)
        .bind(request.bank_name)
        .bind(request.bank_account_name)
        .bind(request.bank_account_number)
        .bind(request.bank_code)
        .bind(request.branch_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //EntityContractorBankDetails
    pub async fn delete_entity_contractor_bank_details(&self, ulid: Uuid) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                entity_contractor_bank_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
