use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::UserAccessToken, user::UserType};
use uuid::Uuid;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchAccountDetails {
    pub company_name: String,
    pub country: String,
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

impl<'r> FromRow<'r, PgRow> for BranchAccountDetails {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let maybe_logo: Option<Vec<u8>> = row.try_get("logo")?;
        Ok(BranchAccountDetails {
            company_name: row.try_get("company_name")?,
            country: row.try_get("country")?,
            entity_type: row.try_get("entity_type")?,
            registration_number: row.try_get("registration_number")?,
            tax_id: row.try_get("tax_id")?,
            statutory_contribution_submission_number: row
                .try_get("statutory_contribution_submission_number")?,
            company_address: row.try_get("company_address")?,
            city: row.try_get("city")?,
            postal_code: row.try_get("postal_code")?,
            time_zone: row.try_get("time_zone")?,
            logo: maybe_logo.map(ImageData),
        })
    }
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
        .post_branch_account_details(PostBranchAccountDetailsInput {
            ulid: claims.payload.ulid,
            company_name: request.company_name,
            country: request.country,
            entity_type: request.entity_type,
            registration_number: request.registration_number,
            tax_id: request.tax_id,
            statutory_contribution_submission_number: request
                .statutory_contribution_submission_number,
            company_address: request.company_address,
            city: request.city,
            postal_code: request.postal_code,
            time_zone: request.time_zone,
            logo: request.logo,
        })
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

pub struct PostBranchAccountDetailsInput {
    pub ulid: Uuid,
    pub company_name: String,
    pub country: String,
    pub entity_type: String,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub statutory_contribution_submission_number: Option<String>,
    pub company_address: String,
    pub city: String,
    pub postal_code: String,
    pub time_zone: String,
    pub logo: Option<ImageData>,
}

impl Database {
    pub async fn post_branch_account_details(
        &self,
        PostBranchAccountDetailsInput {
            ulid,
            company_name,
            country,
            entity_type,
            registration_number,
            tax_id,
            statutory_contribution_submission_number,
            company_address,
            city,
            postal_code,
            time_zone,
            logo,
        }: PostBranchAccountDetailsInput,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO entity_clients_branch_account_details (
                ulid, company_name, country, entity_type, registration_number, 
                tax_id, statutory_contribution_submission_number, company_address, city, postal_code, 
                time_zone, logo
            ) VALUES (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12
            ) ON CONFLICT(ulid) DO UPDATE SET 
                company_name = $2, country = $3, entity_type = $4, registration_number = $5,
                tax_id = $6, statutory_contribution_submission_number = $7, company_address = $8, city = $9, postal_code = $10, 
                time_zone = $11, logo = $12
            ";

        sqlx::query(query)
            .bind(ulid)
            .bind(company_name)
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
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_one_branch_account_details(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<Option<BranchAccountDetails>> {
        let query = "
            SELECT
                ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
                city, postal_code, time_zone, logo
            FROM
                entity_clients_branch_account_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid)
            .fetch_optional(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchAccountDetailsRequest {
    branch_ulid: Uuid,
}
