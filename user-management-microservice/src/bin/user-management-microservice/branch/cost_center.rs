//! For admin pic to manage cost center

use crate::database::{Database, SharedDatabase};
use crate::eor_admin::cost_center::{
    AddContractorToCostCenterRequest, CostCenterContractorResponse, GetCostCenterResponse,
    ListCostCentersContractorsRequest, ListCostCentersRequest, PostCostCenterRequest,
    UpdateCostCenterRequest,
};
use axum::extract::{Extension, Json, Query};
use common_utils::error::GlobeliseError;
use common_utils::error::GlobeliseResult;
use common_utils::token::Token;
use user_management_microservice_sdk::token::UserAccessToken;
use uuid::Uuid;

pub async fn create_cost_center(
    claims: Token<UserAccessToken>,
    Json(request): Json<PostCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.create_cost_center(request).await?;

    Ok(())
}

//list the cost centers for a branch
pub async fn list_cost_centers(
    claims: Token<UserAccessToken>,
    Query(request): Query<ListCostCentersRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetCostCenterResponse>>> {
    let database = database.lock().await;

    if !database
        .branch_belongs_to_pic(request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let result = database.list_cost_centers(request).await?;

    Ok(Json(result))
}

pub async fn list_cost_center_contractors(
    claims: Token<UserAccessToken>,
    Query(request): Query<ListCostCentersContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<CostCenterContractorResponse>>> {
    let database = database.lock().await;

    let cost_center = database.get_cost_center(request.cost_center_ulid).await?;

    if !database
        .cost_center_belongs_to_pic(
            request.cost_center_ulid,
            cost_center.branch_ulid,
            claims.payload.ulid,
        )
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    let result = database.list_cost_center_contractors(request).await?;

    Ok(Json(result))
}

pub async fn delete_cost_center(
    claims: Token<UserAccessToken>,
    axum::extract::Path(cost_center_ulid): axum::extract::Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let cost_center = database.get_cost_center(cost_center_ulid).await?;

    if !database
        .cost_center_belongs_to_pic(
            cost_center.ulid,
            cost_center.branch_ulid,
            claims.payload.ulid,
        )
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.delete_cost_center(cost_center_ulid).await?;

    Ok(())
}

pub async fn update_cost_center(
    claims: Token<UserAccessToken>,
    Json(request): Json<UpdateCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    if !database
        .cost_center_belongs_to_pic(request.ulid, request.branch_ulid, claims.payload.ulid)
        .await?
    {
        return Err(GlobeliseError::Forbidden);
    }

    database.update_cost_center(request).await?;

    Ok(())
}

pub async fn add_contractor_to_cost_center(
    claims: Token<UserAccessToken>,
    Json(request): Json<AddContractorToCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let cost_center = database.get_cost_center(request.cost_center_ulid).await?;

    if !database
        .cost_center_belongs_to_pic(
            request.cost_center_ulid,
            cost_center.branch_ulid,
            claims.payload.ulid,
        )
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

    database.add_contractor_to_cost_center(request).await?;

    Ok(())
}

pub async fn delete_contractor_from_cost_center(
    claims: Token<UserAccessToken>,
    Json(request): Json<AddContractorToCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    let cost_center = database.get_cost_center(request.cost_center_ulid).await?;

    if !database
        .cost_center_belongs_to_pic(
            request.cost_center_ulid,
            cost_center.branch_ulid,
            claims.payload.ulid,
        )
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

    database.delete_contractor_from_cost_center(request).await?;

    Ok(())
}

impl Database {
    pub async fn cost_center_belongs_to_pic(
        &self,
        cost_center_ulid: Uuid,
        branch_ulid: Uuid,
        client_pic_ulid: Uuid,
    ) -> GlobeliseResult<bool> {
        let response = sqlx::query(
            "SELECT
                *
            FROM
                entity_client_branches 
            JOIN
                cost_center ON entity_client_branches.ulid = cost_center.branch_ulid
            WHERE entity_client_branches.ulid = $1
            AND entity_client_branches.client_ulid = $2
            AND cost_center.ulid = $3",
        )
        .bind(branch_ulid)
        .bind(client_pic_ulid)
        .bind(cost_center_ulid)
        .fetch_optional(&self.0)
        .await?
        .is_some();

        Ok(response)
    }

    pub async fn get_cost_center(
        &self,
        cost_center_ulid: Uuid,
    ) -> GlobeliseResult<GetCostCenterResponse> {
        let response = sqlx::query_as(
            "SELECT * FROM 
                    cost_center_index 
                WHERE
                    ulid = $1",
        )
        .bind(cost_center_ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }
}
