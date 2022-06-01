use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{Country, ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::FromRow;
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchAccountDetails {
    pub branch_name: String,
    pub country: Country,
    pub entity_type: String,
    #[serde(default)]
    pub registration_number: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    pub statutory_contribution_submission_number: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    pub logo: Option<ImageData>,
}

pub async fn post_branch_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<BranchAccountDetails>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database
        .post_branch_account_details(
            claims.payload.ulid,
            request.branch_name,
            request.country,
            request.entity_type,
            request.registration_number,
            request.tax_id,
            request.statutory_contribution_submission_number,
            request.company_address,
            request.city,
            request.postal_code,
            request.time_zone,
            request.logo,
        )
        .await?;

    Ok(())
}

pub async fn get_branch_account_details(
    claims: Token<UserAccessToken>,
    ContentLengthLimit(Json(request)): ContentLengthLimit<
        Json<BranchAccountDetailsRequest>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BranchAccountDetails>> {
    if !matches!(claims.payload.user_type, UserType::Entity) {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    if !database
        .client_owns_branch(claims.payload.ulid, request.branch_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(
        database
            .get_one_branch_account_details(request.branch_ulid)
            .await?
            .ok_or(GlobeliseError::NotFound)?,
    ))
}

impl Database {
    #[allow(clippy::too_many_arguments)]
    pub async fn post_branch_account_details(
        &self,
        ulid: Uuid,
        branch_name: String,
        country: Country,
        entity_type: String,
        registration_number: Option<String>,
        tax_id: Option<String>,
        statutory_contribution_submission_number: Option<String>,
        company_address: String,
        city: String,
        postal_code: String,
        time_zone: String,
        logo: Option<ImageData>,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO entity_clients_branch_account_details (
                ulid, branch_name, country, entity_type, registration_number, 
                tax_id, statutory_contribution_submission_number, company_address, city, postal_code, 
                time_zone, logo
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12
            ) ON CONFLICT(ulid) DO UPDATE SET 
                branch_name = $2, country = $3, entity_type = $4, registration_number = $5,
                tax_id = $6, statutory_contribution_submission_number = $7, company_address = $8, city = $9, postal_code = $10, 
                time_zone = $11, logo = $12
            ";

        sqlx::query(query)
            .bind(ulid)
            .bind(branch_name)
            .bind(country)
            .bind(entity_type)
            .bind(registration_number)
            .bind(tax_id)
            .bind(statutory_contribution_submission_number)
            .bind(company_address)
            .bind(city)
            .bind(postal_code)
            .bind(time_zone)
            .bind(logo.map(|b| b.as_ref().to_owned()))
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_one_branch_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<BranchAccountDetails>> {
        let query = "
            SELECT
                ulid, branch_name, country, entity_type, registration_number, tax_id, company_address,
                city, postal_code, time_zone, logo
            FROM
                entity_clients_branch_account_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await?;

        Ok(result)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchAccountDetailsRequest {
    branch_ulid: Uuid,
}
