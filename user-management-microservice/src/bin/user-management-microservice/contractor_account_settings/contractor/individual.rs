use crate::database::{Database, SharedDatabase};
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
pub struct IndividualContractorAccountDetails {
    pub ulid: Uuid,
    pub first_name: String,
    pub last_name: String,
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

    // #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub passport_expiry_date: String,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub dob: sqlx::types::time::OffsetDateTime,

    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub profile_picture: Option<ImageData>,

    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub cv: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorAccountDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorBankDetails {
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
pub struct IndividualContractorBankDetailsRequest {
    pub ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorEmployementInformation {
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
pub struct IndividualContractorEmployementInformationRequest {
    pub contractor_uuid: Uuid,
    pub team_uuid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct IndividualContractorPayrollInformation {
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
pub struct IndividualContractorPayrollInformationRequest {
    pub contractor_ulid: Uuid,
    pub client_ulid: Uuid,
}

//
//######### methods #########
//

//IndividualContractorAccountDetails
pub async fn get_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorAccountDetails>> {
    let database = database.lock().await;

    let response = database
        .get_individual_contractor_account_details(claims.payload.ulid)
        .await?;

    Ok(Json(response))
}
//IndividualContractorAccountDetails
pub async fn update_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorAccountDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_individual_contractor_account_details(request)
        .await?;

    Ok(())
}
//IndividualContractorAccountDetails
pub async fn delete_individual_contractor_account_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_individual_contractor_account_details(claims.payload.ulid)
        .await?;

    Ok(())
}

//IndividualContractorBankDetails
pub async fn get_individual_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorBankDetails>> {
    let database = database.lock().await;

    let response = database
        .get_individual_contractor_bank_details(claims.payload.ulid)
        .await?;

    Ok(Json(response))
}
//IndividualContractorBankDetails
pub async fn update_individual_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorBankDetails>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_individual_contractor_bank_details(request)
        .await?;

    Ok(())
}
//IndividualContractorBankDetails
pub async fn delete_individual_contractor_bank_details(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_individual_contractor_bank_details(claims.payload.ulid)
        .await?;

    Ok(())
}

//IndividualContractorEmployementInformation
pub async fn get_individual_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorEmployementInformation>> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_uuid {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_individual_contractor_employment_information(request)
        .await?;

    Ok(Json(response))
}
//IndividualContractorEmployementInformation
pub async fn update_individual_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorEmployementInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_uuid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_individual_contractor_employment_information(request)
        .await?;

    Ok(())
}
//IndividualContractorEmployementInformation
pub async fn delete_individual_contractor_employment_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorEmployementInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_uuid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_individual_contractor_employment_information(request)
        .await?;

    Ok(())
}

//IndividualContractorPayrollInformation
pub async fn get_individual_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<IndividualContractorPayrollInformation>> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }
    let response = database
        .get_individual_contractor_payroll_information(request)
        .await?;

    Ok(Json(response))
}
//IndividualContractorPayrollInformation
pub async fn update_individual_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorPayrollInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .update_individual_contractor_payroll_information(request)
        .await?;

    Ok(())
}
//IndividualContractorPayrollInformation
pub async fn delete_individual_contractor_payroll_information(
    claims: Token<UserAccessToken>,
    Json(request): Json<IndividualContractorPayrollInformationRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if claims.payload.ulid != request.contractor_ulid {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .delete_individual_contractor_payroll_information(request)
        .await?;

    Ok(())
}
//
//######### DB #########
//

impl Database {
    //IndividualContractorAccountDetails
    pub async fn get_individual_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<IndividualContractorAccountDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                individual_contractor_account_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //IndividualContractorAccountDetails
    pub async fn update_individual_contractor_account_details(
        &self, //kz
        request: IndividualContractorAccountDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                individual_contractor_account_details(
                    ulid,
                    first_name,
                    last_name,
                    dial_code,
                    phone_number,
                    country,
                    city,
                    address,
                    postal_code,
                    tax_id,
                    time_zone,
                    gender,
                    marital_status,
                    nationality,
                    email_address,
                    national_id,
                    passport_number,
                    work_permit,
                    added_related_pay_item_id,
                    total_dependants,
                    passport_expiry_date,
                    dob,
                    profile_picture,
                    cv
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
                ON CONFLICT(ulid) DO UPDATE
            SET
                first_name = $2,
                last_name = $3,
                dial_code = $4,
                phone_number = $5,
                country = $6,
                city = $7,
                address = $8,
                postal_code = $9,
                tax_id = $10,
                time_zone = $11,
                gender = $12,
                marital_status = $13,
                nationality = $14,
                email_address = $15,
                national_id = $16,
                passport_number = $17,
                work_permit = $18,
                added_related_pay_item_id = $19,
                total_dependants = $20,
                passport_expiry_date = $21,
                dob = $22,
                profile_picture = $23,
                cv = $24",
        )
        .bind(request.ulid)
        .bind(request.first_name)
        .bind(request.last_name)
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
        .bind(request.dob)
        .bind(request.profile_picture.map(|b| b.as_ref().to_owned()))
        .bind(request.cv)
        .execute(&self.0)
        .await?;

        Ok(())
    }
    //IndividualContractorAccountDetails
    pub async fn delete_individual_contractor_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                individual_contractor_account_details
            WHERE 
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //IndividualContractorBankDetails
    pub async fn get_individual_contractor_bank_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<IndividualContractorBankDetails> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                individual_contractor_bank_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
    //IndividualContractorBankDetails
    pub async fn update_individual_contractor_bank_details(
        &self,
        request: IndividualContractorBankDetails,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                individual_contractor_bank_details(
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
    //IndividualContractorBankDetails
    pub async fn delete_individual_contractor_bank_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM
                individual_contractor_bank_details
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    //IndividualContractorEmployementInformation
    pub async fn get_individual_contractor_employment_information(
        &self,
        request: IndividualContractorEmployementInformationRequest,
    ) -> GlobeliseResult<IndividualContractorEmployementInformation> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                individual_contractor_employment_information
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
    //IndividualContractorEmployementInformation
    pub async fn update_individual_contractor_employment_information(
        &self,
        request: IndividualContractorEmployementInformation,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                individual_contractor_employment_information(
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
    //IndividualContractorEmployementInformation
    pub async fn delete_individual_contractor_employment_information(
        &self,
        request: IndividualContractorEmployementInformationRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                individual_contractor_employment_information
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

    //IndividualContractorPayrollInformation
    pub async fn get_individual_contractor_payroll_information(
        &self,
        request: IndividualContractorPayrollInformationRequest,
    ) -> GlobeliseResult<IndividualContractorPayrollInformation> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                individual_contractor_payroll_information
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
    //IndividualContractorPayrollInformation
    pub async fn update_individual_contractor_payroll_information(
        &self,
        request: IndividualContractorPayrollInformation,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            INSERT INTO 
                individual_contractor_payroll_information(
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
    //IndividualContractorPayrollInformation
    pub async fn delete_individual_contractor_payroll_information(
        &self,
        request: IndividualContractorPayrollInformationRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
            DELETE FROM 
                individual_contractor_payroll_information
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
}
