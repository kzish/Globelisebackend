use axum::{
    extract::{Extension, Path},
    Json,
};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::OffsetDateWrapper,
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use sqlx::{types::Uuid, FromRow};
use user_management_microservice_sdk::token::UserAccessToken;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListClientContractorEmploymentInformationRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListClientContractorEmploymentInformationResponse {
    pub contractor_uuid: Uuid,
    pub team_uuid: Uuid,
    pub designation: String,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub start_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub end_date: sqlx::types::time::OffsetDateTime,
    pub employment_status: String,
    pub contractor_type: String,
    pub client_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmploymentInformation {
    pub contractor_uuid: Uuid,
    pub team_uuid: Option<Uuid>,
    pub designation: String,
    pub employment_status: bool,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub start_date: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub end_date: sqlx::types::time::OffsetDateTime,
}

pub async fn get_employment_information_individual(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EmploymentInformation>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_employment_information_individual(contractor_ulid)
        .await?;

    Ok(Json(response))
}

pub async fn post_employment_information_individual(
    claims: Token<UserAccessToken>,
    Json(request): Json<EmploymentInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.contractor_uuid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database
        .post_employment_information_individual(request)
        .await?;

    Ok(())
}

pub async fn get_employment_information_entity(
    claims: Token<UserAccessToken>,
    Path(contractor_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<EmploymentInformation>> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database
        .get_employment_information_entity(contractor_ulid)
        .await?;

    Ok(Json(response))
}

pub async fn post_employment_information_entity(
    claims: Token<UserAccessToken>,
    Json(request): Json<EmploymentInformation>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.contractor_uuid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.post_employment_information_entity(request).await?;

    Ok(())
}

impl Database {
    // Index contract of a given contractor
    pub async fn get_employment_information_all(
        &self,
        request: ListClientContractorEmploymentInformationRequest,
    ) -> GlobeliseResult<Vec<ListClientContractorEmploymentInformationResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let index = sqlx::query_as(
            "
            SELECT
                *
            FROM
                contractor_employment_information
            WHERE
                client_ulid = $3
            LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(request.client_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(index)
    }

    pub async fn get_employment_information_individual(
        &self,
        uuid: Uuid,
    ) -> GlobeliseResult<EmploymentInformation> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                individual_contractor_employment_information 
            WHERE contractor_uuid = $1",
        )
        .bind(uuid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_employment_information_individual(
        &self,
        request: EmploymentInformation,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                        individual_contractor_employment_information
                (contractor_uuid, team_uuid, designation, start_date, end_date, employment_status)
                VALUES($1, $2, $3, $4, $5, $6)
                ON CONFLICT(contractor_uuid, team_uuid) DO UPDATE SET 
                   designation = $3, 
                   start_date = $4, 
                   end_date = $5, 
                   employment_status = $6",
        )
        .bind(request.contractor_uuid)
        .bind(request.team_uuid)
        .bind(request.designation)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.employment_status)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_employment_information_entity(
        &self,
        uuid: Uuid,
    ) -> GlobeliseResult<EmploymentInformation> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                entity_contractor_employment_information 
            WHERE contractor_uuid = $1",
        )
        .bind(uuid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_employment_information_entity(
        &self,
        request: EmploymentInformation,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                        entity_contractor_employment_information
                (contractor_uuid, team_uuid, designation, start_date, end_date, employment_status)
                VALUES($1, $2, $3, $4, $5, $6)
                ON CONFLICT(contractor_uuid, team_uuid) DO UPDATE SET 
                   designation = $3, 
                   start_date = $4, 
                   end_date = $5, 
                   employment_status = $6",
        )
        .bind(request.contractor_uuid)
        .bind(request.team_uuid)
        .bind(request.designation)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.employment_status)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
