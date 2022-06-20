use axum::{
    extract::{ContentLengthLimit, Query},
    Extension, Json,
};
use common_utils::{
    calc_limit_and_offset, custom_serde::FORM_DATA_LENGTH_LIMIT, error::GlobeliseResult,
    token::Token,
};
use eor_admin_microservice_sdk::token::AdminAccessToken;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;

use crate::database::{Database, SharedDatabase};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetManyCountryCodeQuery {
    per_page: Option<u32>,
    page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PutOneCountryCode {
    code: String,
    long_name: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteOneCountryCode {
    code: String,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CountryCode {
    code: String,
    long_name: String,
}

pub async fn get_many(
    Query(query): Query<GetManyCountryCodeQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<CountryCode>>> {
    let database = database.lock().await;

    let result = database
        .select_many_country_codes(query.page, query.per_page)
        .await?;

    Ok(Json(result))
}

pub async fn post_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PutOneCountryCode>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_country_code(body.code, body.long_name)
        .await?;

    Ok(())
}

pub async fn delete_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<DeleteOneCountryCode>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_one_country_code(body.code).await?;

    Ok(())
}

impl Database {
    pub async fn insert_one_country_code(
        &self,
        code: String,
        long_name: String,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO country_codes (
            code, long_name
        ) VALUES (
            $1, $2
        ) ON CONFLICT(code) DO UPDATE SET 
            long_name = $2",
        )
        .bind(code)
        .bind(long_name)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_one_country_code(&self, code: String) -> GlobeliseResult<()> {
        sqlx::query(
            "
        DELETE FROM country_codes
        WHERE
            code = $1",
        )
        .bind(code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn select_many_country_codes(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<CountryCode>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
        SELECT
            *
        FROM
            country_codes
        LIMIT
            $1
        OFFSET
            $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }
}
