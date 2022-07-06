use crate::database::{Database, SharedDatabase};
use crate::eor_admin::TryFromInto;
use axum::extract::{Extension, Json, Path, Query};
use common_utils::token::Token;
use common_utils::{
    calc_limit_and_offset, custom_serde::OffsetDateWrapper, error::GlobeliseResult,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use uuid::Uuid;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateTeamRequest {
    pub branch_ulid: Uuid,
    pub team_name: String,
    pub schedule_type: String,
    pub time_zone: String,

    pub working_days_sun: bool,
    pub working_days_mon: bool,
    pub working_days_tue: bool,
    pub working_days_wed: bool,
    pub working_days_thu: bool,
    pub working_days_fri: bool,
    pub working_days_sat: bool,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sun_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_mon_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_tue_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_wed_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_thu_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_fri_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sat_start: sqlx::types::time::OffsetDateTime,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sun_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_mon_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_tue_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_wed_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_thu_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_fri_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sat_end: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UpdateTeamRequest {
    pub team_ulid: Uuid,
    pub branch_ulid: Uuid,
    pub team_name: String,
    pub schedule_type: String,
    pub time_zone: String,

    pub working_days_sun: bool,
    pub working_days_mon: bool,
    pub working_days_tue: bool,
    pub working_days_wed: bool,
    pub working_days_thu: bool,
    pub working_days_fri: bool,
    pub working_days_sat: bool,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sun_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_mon_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_tue_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_wed_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_thu_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_fri_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sat_start: sqlx::types::time::OffsetDateTime,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sun_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_mon_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_tue_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_wed_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_thu_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_fri_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sat_end: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub branch_ulid: Uuid,
    pub team_name: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamsClientUlidRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Uuid,
    pub team_name: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamsResponse {
    pub member: i64,
    pub team_ulid: Uuid,
    pub branch_ulid: Uuid,
    pub client_ulid: Uuid,
    pub team_name: String,
    pub schedule_type: String,
    pub time_zone: String,

    pub working_days_sun: bool,
    pub working_days_mon: bool,
    pub working_days_tue: bool,
    pub working_days_wed: bool,
    pub working_days_thu: bool,
    pub working_days_fri: bool,
    pub working_days_sat: bool,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sun_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_mon_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_tue_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_wed_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_thu_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_fri_start: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sat_start: sqlx::types::time::OffsetDateTime,

    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sun_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_mon_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_tue_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_wed_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_thu_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_fri_end: sqlx::types::time::OffsetDateTime,
    #[serde_as(as = "TryFromInto<OffsetDateWrapper>")]
    pub working_hours_sat_end: sqlx::types::time::OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddContractorToTeamRequest {
    pub team_ulid: Uuid,
    pub contractor_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamContractorsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub team_ulid: Uuid,
    pub contractor_name: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamFreeContractorsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub contractor_name: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamContractorsResponse {
    pub contractor_ulid: Uuid,
    pub contractor_name: Option<String>,
    pub branch_ulid: Option<Uuid>,
    pub branch_name: Option<String>,
    pub team_name: Option<String>,
    pub team_ulid: Option<Uuid>,
    pub country: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListTeamFreeContractorsResponse {
    pub contractor_ulid: Uuid,
    pub contractor_name: Option<String>,
    pub email_address: Option<String>,
    pub teams_count: Option<i64>,
}

pub async fn create_team(
    _: Token<AdminAccessToken>,
    Json(request): Json<CreateTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.create_team(request).await?;

    Ok(())
}

pub async fn delete_team(
    _: Token<AdminAccessToken>,
    Path(team_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_team(team_ulid).await?;

    Ok(())
}

pub async fn update_team(
    _: Token<AdminAccessToken>,
    Json(request): Json<UpdateTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.update_team(request).await?;

    Ok(())
}

pub async fn list_teams(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListTeamsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamsResponse>>> {
    let database = database.lock().await;

    let response = database.list_teams(request).await?;

    Ok(Json(response))
}

pub async fn list_teams_by_client_ulid(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListTeamsClientUlidRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamsResponse>>> {
    let database = database.lock().await;

    let response = database.list_teams_by_client_ulid(request).await?;

    Ok(Json(response))
}

pub async fn add_contrator_to_team(
    _: Token<AdminAccessToken>,
    Json(request): Json<AddContractorToTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.add_contrator_to_team(request).await?;

    Ok(())
}

pub async fn delete_contrator_from_team(
    _: Token<AdminAccessToken>,
    Json(request): Json<AddContractorToTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_contrator_from_team(request).await?;

    Ok(())
}

pub async fn list_team_contractors(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListTeamContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamContractorsResponse>>> {
    let database = database.lock().await;

    let response = database.list_team_contractors(request).await?;

    Ok(Json(response))
}

pub async fn list_contrators_not_in_this_team(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListTeamContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamContractorsResponse>>> {
    let database = database.lock().await;

    let response = database.list_contrators_not_in_this_team(request).await?;

    Ok(Json(response))
}

pub async fn list_contrators_not_in_any_team(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListTeamFreeContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamFreeContractorsResponse>>> {
    let database = database.lock().await;

    let response = database.list_contrators_not_in_any_team(request).await?;

    Ok(Json(response))
}

impl Database {
    pub async fn create_team(&self, request: CreateTeamRequest) -> GlobeliseResult<()> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO
                teams (
                ulid, 
                branch_ulid,
                team_name,
                schedule_type,
                time_zone,
                working_days_sun,
                working_days_mon,
                working_days_tue,
                working_days_wed,
                working_days_thu,
                working_days_fri,
                working_days_sat,
                working_hours_sun_start,
                working_hours_mon_start,
                working_hours_tue_start,
                working_hours_wed_start,
                working_hours_thu_start,
                working_hours_fri_start,
                working_hours_sat_start,
                working_hours_sun_end,
                working_hours_mon_end,
                working_hours_tue_end,
                working_hours_wed_end,
                working_hours_thu_end,
                working_hours_fri_end,
                working_hours_sat_end
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26)",
        )
        .bind(ulid)
        .bind(request.branch_ulid)
        .bind(request.team_name)
        .bind(request.schedule_type)
        .bind(request.time_zone)
        .bind(request.working_days_sun)
        .bind(request.working_days_mon)
        .bind(request.working_days_tue)
        .bind(request.working_days_wed)
        .bind(request.working_days_thu)
        .bind(request.working_days_fri)
        .bind(request.working_days_sat)
        .bind(request.working_hours_sun_start)
        .bind(request.working_hours_mon_start)
        .bind(request.working_hours_tue_start)
        .bind(request.working_hours_wed_start)
        .bind(request.working_hours_thu_start)
        .bind(request.working_hours_fri_start)
        .bind(request.working_hours_sat_start)
        .bind(request.working_hours_sun_end)
        .bind(request.working_hours_mon_end)
        .bind(request.working_hours_tue_end)
        .bind(request.working_hours_wed_end)
        .bind(request.working_hours_thu_end)
        .bind(request.working_hours_fri_end)
        .bind(request.working_hours_sat_end)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_team(&self, team_ulid: Uuid) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                    teams 
                WHERE ulid = $1",
        )
        .bind(team_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_team(&self, request: UpdateTeamRequest) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE teams
                SET                
                branch_ulid = $2,
                team_name = $3,
                schedule_type = $4,
                time_zone = $5,
                working_days_sun = $6,
                working_days_mon = $7,
                working_days_tue = $8,
                working_days_wed = $9,
                working_days_thu = $10,
                working_days_fri = $11,
                working_days_sat = $12,
                working_hours_sun_start = $13,
                working_hours_mon_start = $14,
                working_hours_tue_start = $15,
                working_hours_wed_start = $16,
                working_hours_thu_start = $17,
                working_hours_fri_start = $18,
                working_hours_sat_start = $19,
                working_hours_sun_end = $20,
                working_hours_mon_end = $21,
                working_hours_tue_end = $22,
                working_hours_wed_end = $23,
                working_hours_thu_end = $24,
                working_hours_fri_end = $25,
                working_hours_sat_end = $26
            WHERE ulid = $1",
        )
        .bind(request.team_ulid)
        .bind(request.branch_ulid)
        .bind(request.team_name)
        .bind(request.schedule_type)
        .bind(request.time_zone)
        .bind(request.working_days_sun)
        .bind(request.working_days_mon)
        .bind(request.working_days_tue)
        .bind(request.working_days_wed)
        .bind(request.working_days_thu)
        .bind(request.working_days_fri)
        .bind(request.working_days_sat)
        .bind(request.working_hours_sun_start)
        .bind(request.working_hours_mon_start)
        .bind(request.working_hours_tue_start)
        .bind(request.working_hours_wed_start)
        .bind(request.working_hours_thu_start)
        .bind(request.working_hours_fri_start)
        .bind(request.working_hours_sat_start)
        .bind(request.working_hours_sun_end)
        .bind(request.working_hours_mon_end)
        .bind(request.working_hours_tue_end)
        .bind(request.working_hours_wed_end)
        .bind(request.working_hours_thu_end)
        .bind(request.working_hours_fri_end)
        .bind(request.working_hours_sat_end)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn list_teams(
        &self,
        request: ListTeamsRequest,
    ) -> GlobeliseResult<Vec<ListTeamsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let response = sqlx::query_as(
            "SELECT * FROM
                    teams_index
                WHERE
                    branch_ulid = $3
                AND
                    ($4 IS NULL OR team_name LIKE $4)
                    LIMIT $1
                    OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(request.branch_ulid)
        .bind(format!("%{}%", request.team_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_teams_by_client_ulid(
        &self,
        request: ListTeamsClientUlidRequest,
    ) -> GlobeliseResult<Vec<ListTeamsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let response = sqlx::query_as(
            "SELECT * FROM
                    teams_index
                WHERE
                    client_ulid = $3
                AND
                    ($4 IS NULL OR team_name LIKE $4)
                    LIMIT $1
                    OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(request.client_ulid)
        .bind(format!("%{}%", request.team_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn add_contrator_to_team(
        &self,
        request: AddContractorToTeamRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                teams_contractors (team_ulid, contractor_ulid)
            VALUES ($1, $2)",
        )
        .bind(request.team_ulid)
        .bind(request.contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_contrator_from_team(
        &self,
        request: AddContractorToTeamRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                    teams_contractors 
                WHERE
                    team_ulid = $1
                AND
                    contractor_ulid = $2",
        )
        .bind(request.team_ulid)
        .bind(request.contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn list_team_contractors(
        &self,
        request: ListTeamContractorsRequest,
    ) -> GlobeliseResult<Vec<ListTeamContractorsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "SELECT * FROM 
                    team_contractors_details 
                WHERE
                    team_ulid = $1
                AND ($4 IS NULL or contractor_name LIKE $4)
                LIMIT $2
                OFFSET $3",
        )
        .bind(request.team_ulid)
        .bind(limit)
        .bind(offset)
        .bind(format!("%{}%", request.contractor_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_contrators_not_in_this_team(
        &self,
        request: ListTeamContractorsRequest,
    ) -> GlobeliseResult<Vec<ListTeamContractorsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "SELECT * FROM 
                    team_contractors_details 
                WHERE
                    team_ulid NOT IN ($1)
                AND ($4 IS NULL or contractor_name LIKE $4)
                LIMIT $2
                OFFSET $3",
        )
        .bind(request.team_ulid)
        .bind(limit)
        .bind(offset)
        .bind(format!("%{}%", request.contractor_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_contrators_not_in_any_team(
        &self,
        request: ListTeamFreeContractorsRequest,
    ) -> GlobeliseResult<Vec<ListTeamFreeContractorsResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "SELECT * FROM 
                    contractors_not_in_any_team_details 
                WHERE
                    ($3 IS NULL or contractor_name LIKE $3)
                AND
                    teams_count = 0
                LIMIT $1
                OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(format!("%{}%", request.contractor_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }
}
