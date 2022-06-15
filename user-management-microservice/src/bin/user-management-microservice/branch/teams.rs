//! For admin pic to manage cost center

use crate::database::{Database, SharedDatabase};
use crate::eor_admin::teams::{
    AddContractorToTeamRequest, CreateTeamRequest, ListTeamContractorsRequest,
    ListTeamContractorsResponse, ListTeamsRequest, ListTeamsResponse, UpdateTeamRequest,
};
use axum::extract::{Extension, Json, Path, Query};
use common_utils::error::GlobeliseError;
use common_utils::error::GlobeliseResult;
use common_utils::token::Token;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

pub async fn create_team(
    claims: Token<UserAccessToken>,
    Json(request): Json<CreateTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.create_team(request).await?;

    Ok(())
}

pub async fn delete_team(
    claims: Token<UserAccessToken>,
    Path(team_ulid): Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let team = database.get_team(team_ulid).await?;

    if !database
        .team_belongs_to_pic(team_ulid, team.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.delete_team(team_ulid).await?;

    Ok(())
}

pub async fn update_team(
    claims: Token<UserAccessToken>,
    Json(request): Json<UpdateTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .team_belongs_to_pic(request.team_ulid, request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.update_team(request).await?;

    Ok(())
}

pub async fn list_teams(
    claims: Token<UserAccessToken>,
    Query(request): Query<ListTeamsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamsResponse>>> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database.list_teams(request).await?;

    Ok(Json(response))
}

pub async fn add_contrator_to_team(
    claims: Token<UserAccessToken>,
    Json(request): Json<AddContractorToTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let team = database.get_team(request.team_ulid).await?;

    if !database
        .team_belongs_to_pic(request.team_ulid, team.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.add_contrator_to_team(request).await?;

    Ok(())
}

pub async fn delete_contrator_from_team(
    claims: Token<UserAccessToken>,
    Json(request): Json<AddContractorToTeamRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let team = database.get_team(request.team_ulid).await?;

    if !database
        .team_belongs_to_pic(request.team_ulid, team.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    if !database
        .contractor_belongs_to_pic(claims.payload.ulid, request.contractor_ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.delete_contrator_from_team(request).await?;

    Ok(())
}

pub async fn list_team_contractors(
    claims: Token<UserAccessToken>,
    Query(request): Query<ListTeamContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<ListTeamContractorsResponse>>> {
    let database = database.lock().await;

    let team = database.get_team(request.team_ulid).await?;

    if !database
        .team_belongs_to_pic(request.team_ulid, team.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let response = database.list_team_contractors(request).await?;

    Ok(Json(response))
}

impl Database {
    //check this pic is the owner of the team
    pub async fn branch_belongs_to_pic(
        &self,
        branch_ulid: Uuid,
        client_pic_ulid: Uuid,
    ) -> GlobeliseResult<bool> {
        let response = sqlx::query(
            "SELECT
                *
            FROM
                entity_client_branches 
            WHERE ulid = $1
            AND client_ulid = $2",
        )
        .bind(branch_ulid)
        .bind(client_pic_ulid)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(response)
    }

    pub async fn team_belongs_to_pic(
        &self,
        team_ulid: Uuid,
        branch_ulid: Uuid,
        client_pic_ulid: Uuid,
    ) -> GlobeliseResult<bool> {
        let response = sqlx::query(
            "SELECT
                *
            FROM
                entity_client_branches 
            JOIN
                teams ON entity_client_branches.ulid = teams.branch_ulid
            WHERE entity_client_branches.ulid = $1
            AND entity_client_branches.client_ulid = $2
            AND teams.ulid = $3",
        )
        .bind(branch_ulid)
        .bind(client_pic_ulid)
        .bind(team_ulid)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(response)
    }

    pub async fn get_team(&self, team_ulid: Uuid) -> GlobeliseResult<ListTeamsResponse> {
        let response = sqlx::query_as(
            "SELECT * FROM
                    teams_index
                WHERE
                    team_ulid = $1
               ",
        )
        .bind(team_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }
}
