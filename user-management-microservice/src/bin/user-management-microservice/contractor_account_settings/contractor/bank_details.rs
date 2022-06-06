use axum::{extract::Extension, Json};
use common_utils::{
    error::{GlobeliseError, GlobeliseResult},
    token::Token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as};
use sqlx::{types::Uuid, FromRow};
use user_management_microservice_sdk::token::UserAccessToken;

use crate::database::{Database, SharedDatabase};

//entities

#[serde_as]
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BankDetailsRequest {
    pub ulid: Uuid,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BankDetailsResponse {
    pub ulid: Uuid,
    pub bank_name: String,
    pub bank_account_name: String,
    pub bank_account_number: String,
    pub bank_code: String,
    pub branch_code: String,
}

//methods
pub async fn get_bank_details_entity(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetailsResponse>> {
    let database = database.lock().await;

    let response = database
        .get_bank_details_entity(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != response.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}

pub async fn post_bank_details_entity(
    claims: Token<UserAccessToken>,
    Json(request): Json<BankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database.post_bank_details_entity(request).await?;

    Ok(())
}

pub async fn get_bank_details_individual(
    claims: Token<UserAccessToken>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<BankDetailsResponse>> {
    let database = database.lock().await;

    let response = database
        .get_bank_details_individual(claims.payload.ulid)
        .await?;

    if claims.payload.ulid != response.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    Ok(Json(response))
}

pub async fn post_bank_details_individual(
    claims: Token<UserAccessToken>,
    Json(request): Json<BankDetailsRequest>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    if claims.payload.ulid != request.ulid {
        return Err(GlobeliseError::Forbidden);
    }

    let database = database.lock().await;

    database.post_bank_details_individual(request).await?;

    Ok(())
}

impl Database {
    pub async fn get_bank_details_entity(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<BankDetailsResponse> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                entity_contractors_bank_details 
            WHERE ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_bank_details_entity(
        &self,
        request: BankDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                        entity_contractors_bank_details
                            (ulid, bank_name, bank_account_name, bank_account_number, branch_code, bank_code)
                  VALUES($1, $2, $3, $4, $5, $6)
                  ON CONFLICT(ulid) DO UPDATE SET 
                            bank_name = $2,
                            bank_account_name = $3,
                            bank_account_number = $4,
                            branch_code = $5,
                            bank_code = $6;
                  "
        )
        .bind(request.ulid)
        .bind(request.bank_name)
        .bind(request.bank_account_name)
        .bind(request.bank_account_number)
        .bind(request.branch_code)
        .bind(request.bank_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_bank_details_individual(
        &self,
        ulid: Uuid,
    ) -> GlobeliseResult<BankDetailsResponse> {
        let response = sqlx::query_as(
            "SELECT
                *
            FROM
                individual_contractors_bank_details 
            WHERE ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(response)
    }

    pub async fn post_bank_details_individual(
        &self,
        request: BankDetailsRequest,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "INSERT INTO
                            individual_contractors_bank_details
                            (ulid, bank_name, bank_account_name, bank_account_number, bank_code, branch_code)
                  VALUES($1, $2, $3, $4, $5, $6)
                  ON CONFLICT(ulid) DO UPDATE SET 
                            bank_name = $2,
                            bank_account_name = $3,
                            bank_account_number = $4,
                            bank_code = $5,
                            branch_code = $6
                  "
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
}
