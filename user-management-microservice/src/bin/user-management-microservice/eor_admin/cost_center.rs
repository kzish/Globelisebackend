use crate::database::{Database, SharedDatabase};
use axum::extract::{Extension, Json, Query};
use common_utils::token::Token;
use common_utils::{calc_limit_and_offset, error::GlobeliseResult};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct GetCostCenterResponse {
    pub ulid: Uuid,
    pub branch_ulid: Uuid,
    pub cost_center_name: String,
    pub member: i64,
    pub currency: String,
    pub client_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct UpdateCostCenterRequest {
    pub ulid: Uuid,
    pub branch_ulid: Uuid,
    pub cost_center_name: String,
    pub currency: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListCostCentersRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub branch_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListCostCentersClientUlidRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub client_ulid: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListCostCentersContractorsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub cost_center_ulid: Uuid,
    pub contractor_name: Option<String>,
    pub branch_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct ListFreeCostCentersContractorsRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub contractor_name: Option<String>,
    pub branch_ulid: Option<Uuid>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct CostCenterContractorResponse {
    pub contractor_ulid: Uuid,
    pub contractor_name: String,
    pub branch_ulid: Uuid,
    pub branch_name: Option<String>,
    pub cost_center_name: String,
    pub cost_center_ulid: Uuid,
    pub country: Option<String>,
    pub currency: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "kebab-case")]
pub struct FreeCostCenterContractorResponse {
    pub contractor_ulid: Uuid,
    pub contractor_name: Option<String>,
    pub email_address: Option<String>,
    pub cost_center_count: Option<i64>,

    pub time_zone: Option<String>,
    pub branch_ulid: Option<Uuid>,
    pub branch_name: Option<String>,
    pub job_description: Option<String>,
    pub country: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostCostCenterRequest {
    pub branch_ulid: Uuid,
    pub cost_center_name: String,
    pub currency: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddContractorToCostCenterRequest {
    pub cost_center_ulid: Uuid,
    pub contractor_ulid: Uuid,
}

pub async fn create_cost_center(
    _: Token<AdminAccessToken>,
    Json(request): Json<PostCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.create_cost_center(request).await?;

    Ok(())
}

//list the cost centers for a branch
pub async fn list_cost_centers(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListCostCentersRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetCostCenterResponse>>> {
    let database = database.lock().await;

    let result = database.list_cost_centers(request).await?;

    Ok(Json(result))
}

//list the cost centers for a client
pub async fn list_cost_centers_by_client_ulid(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListCostCentersClientUlidRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<GetCostCenterResponse>>> {
    let database = database.lock().await;

    let result = database.list_cost_centers_by_client_ulid(request).await?;

    Ok(Json(result))
}

pub async fn list_cost_center_contractors(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListCostCentersContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<CostCenterContractorResponse>>> {
    let database = database.lock().await;

    let result = database.list_cost_center_contractors(request).await?;

    Ok(Json(result))
}

pub async fn list_contrators_not_in_this_cost_center(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListCostCentersContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<FreeCostCenterContractorResponse>>> {
    let database = database.lock().await;

    let result = database
        .list_contrators_not_in_this_cost_center(request)
        .await?;

    Ok(Json(result))
}

pub async fn list_contrators_not_in_any_cost_center(
    _: Token<AdminAccessToken>,
    Query(request): Query<ListFreeCostCentersContractorsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<FreeCostCenterContractorResponse>>> {
    let database = database.lock().await;

    let result = database
        .list_contrators_not_in_any_cost_center(request)
        .await?;

    Ok(Json(result))
}

pub async fn delete_cost_center(
    _: Token<AdminAccessToken>,
    axum::extract::Path(cost_center_ulid): axum::extract::Path<Uuid>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_cost_center(cost_center_ulid).await?;

    Ok(())
}

pub async fn update_cost_center(
    _: Token<AdminAccessToken>,
    Json(request): Json<UpdateCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.update_cost_center(request).await?;

    Ok(())
}

pub async fn add_contractor_to_cost_center(
    _: Token<AdminAccessToken>,
    Json(request): Json<AddContractorToCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.add_contractor_to_cost_center(request).await?;

    Ok(())
}

pub async fn delete_contractor_from_cost_center(
    _: Token<AdminAccessToken>,
    Json(request): Json<AddContractorToCostCenterRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_contractor_from_cost_center(request).await?;

    Ok(())
}

impl Database {
    pub async fn create_cost_center(&self, request: PostCostCenterRequest) -> GlobeliseResult<()> {
        let ulid = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO
                cost_center (ulid, branch_ulid, cost_center_name, currency)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(ulid)
        .bind(request.branch_ulid)
        .bind(request.cost_center_name)
        .bind(request.currency)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn list_cost_centers(
        &self,
        request: ListCostCentersRequest,
    ) -> GlobeliseResult<Vec<GetCostCenterResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let response = sqlx::query_as(
            "SELECT * FROM 
                    cost_center_index 
                WHERE
                    branch_ulid = $1
                LIMIT $2
                OFFSET $3",
        )
        .bind(request.branch_ulid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_cost_centers_by_client_ulid(
        &self,
        request: ListCostCentersClientUlidRequest,
    ) -> GlobeliseResult<Vec<GetCostCenterResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);

        let response = sqlx::query_as(
            "SELECT * FROM 
                    cost_center_index 
                WHERE
                    client_ulid = $1
                LIMIT $2
                OFFSET $3",
        )
        .bind(request.client_ulid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_cost_center_contractors(
        &self,
        request: ListCostCentersContractorsRequest,
    ) -> GlobeliseResult<Vec<CostCenterContractorResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "SELECT * FROM 
                    cost_center_contractors_details 
                WHERE
                    cost_center_ulid = $1
                AND ($4 IS NULL or contractor_name LIKE $4)
                LIMIT $2
                OFFSET $3",
        )
        .bind(request.cost_center_ulid)
        .bind(limit)
        .bind(offset)
        .bind(format!("%{}%", request.contractor_name.unwrap_or_default()))
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_contrators_not_in_this_cost_center(
        &self,
        request: ListCostCentersContractorsRequest,
    ) -> GlobeliseResult<Vec<FreeCostCenterContractorResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "SELECT * FROM 
                    cost_center_contractors_details 
                WHERE
                    cost_center_ulid <> $1
                AND ($4 IS NULL or contractor_name LIKE $4)
                AND branch_ulid = $5
                LIMIT $2
                OFFSET $3",
        )
        .bind(request.cost_center_ulid)
        .bind(limit)
        .bind(offset)
        .bind(format!("%{}%", request.contractor_name.unwrap_or_default()))
        .bind(request.branch_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn list_contrators_not_in_any_cost_center(
        &self,
        request: ListFreeCostCentersContractorsRequest,
    ) -> GlobeliseResult<Vec<FreeCostCenterContractorResponse>> {
        let (limit, offset) = calc_limit_and_offset(request.per_page, request.page);
        let response = sqlx::query_as(
            "SELECT * FROM 
                    contractors_not_in_any_cost_center_details 
                WHERE
                    ($3 IS NULL or contractor_name LIKE $3)
                AND
                    cost_center_count = 0
                AND 
                    branch_ulid = $4
                LIMIT $1
                OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .bind(format!("%{}%", request.contractor_name.unwrap_or_default()))
        .bind(request.branch_ulid)
        .fetch_all(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn delete_cost_center(&self, ulid: Uuid) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                cost_center 
             WHERE
                ulid = $1",
        )
        .bind(ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn update_cost_center(
        &self,
        request: UpdateCostCenterRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "UPDATE
                cost_center 
             SET
                branch_ulid = $2,
                cost_center_name = $3,
                currency = $4
             WHERE 
                ulid = $1",
        )
        .bind(request.ulid)
        .bind(request.branch_ulid)
        .bind(request.cost_center_name)
        .bind(request.currency)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn add_contractor_to_cost_center(
        &self,
        request: AddContractorToCostCenterRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                cost_center_contractor_pairs (cost_center_ulid, contractor_ulid)
             VALUES ($1, $2) ON CONFLICT(cost_center_ulid, contractor_ulid) DO NOTHING",
        )
        .bind(request.cost_center_ulid)
        .bind(request.contractor_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_contractor_from_cost_center(
        &self,
        request: AddContractorToCostCenterRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "DELETE FROM
                cost_center_contractor_pairs 
             WHERE
                contractor_ulid = $1
             AND
                cost_center_ulid = $2",
        )
        .bind(request.contractor_ulid)
        .bind(request.cost_center_ulid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
