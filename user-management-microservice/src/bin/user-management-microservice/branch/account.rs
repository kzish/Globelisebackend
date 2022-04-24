use axum::extract::{ContentLengthLimit, Extension, Json};
use common_utils::{
    custom_serde::{ImageData, FORM_DATA_LENGTH_LIMIT},
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
    ulid_to_sql_uuid,
};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::{postgres::PgRow, FromRow, Row};
use user_management_microservice_sdk::{token::AccessToken, user::UserType};

use crate::database::{Database, SharedDatabase};

pub async fn post_branch_account_details(
    claims: Token<AccessToken>,
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
        .post_branch_account_details(claims.payload.ulid, request)
        .await?;

    Ok(())
}

pub async fn get_branch_account_details(
    claims: Token<AccessToken>,
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
            .await?,
    ))
}

impl Database {
    pub async fn post_branch_account_details(
        &self,
        ulid: Ulid,
        details: BranchAccountDetails,
    ) -> GlobeliseResult<()> {
        let query = "
            INSERT INTO entity_clients_branch_account_details (
                ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
                city, postal_code, time_zone, logo
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) ON CONFLICT(ulid) DO UPDATE SET 
                company_name = $2, country = $3, entity_type = $4, registration_number = $5,
                tax_id = $6, company_address = $7, city = $8, postal_code = $9, time_zone = $10,
                logo = $11
            ";

        sqlx::query(query)
            .bind(ulid_to_sql_uuid(ulid))
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

    pub async fn get_one_branch_account_details(
        &self,
        ulid: Ulid,
    ) -> GlobeliseResult<BranchAccountDetails> {
        let query = "
            SELECT
                ulid, company_name, country, entity_type, registration_number, tax_id, company_address,
                city, postal_code, time_zone, logo
            FROM
                entity_clients_branch_account_details
            WHERE
                ulid = $1";

        let result = sqlx::query_as(query)
            .bind(ulid_to_sql_uuid(ulid))
            .fetch_one(&self.0)
            .await
            .map_err(|e| GlobeliseError::Database(e.to_string()))?;

        Ok(result)
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchAccountDetailsRequest {
    branch_ulid: Ulid,
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
