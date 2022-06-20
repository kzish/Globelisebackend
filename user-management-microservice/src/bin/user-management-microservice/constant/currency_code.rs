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
pub struct GetManyCurrencyCodeQuery {
    per_page: Option<u32>,
    page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PutOneCurrencyCode {
    code: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteOneCurrencyCode {
    code: String,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CurrencyCode {
    code: String,
}

pub async fn get_many(
    Query(query): Query<GetManyCurrencyCodeQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<CurrencyCode>>> {
    let database = database.lock().await;

    let result = database
        .select_many_currency_codes(query.page, query.per_page)
        .await?;

    Ok(Json(result))
}

pub async fn post_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PutOneCurrencyCode>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.insert_one_currency_code(body.code).await?;

    Ok(())
}

pub async fn delete_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<DeleteOneCurrencyCode>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database.delete_one_currency_code(body.code).await?;

    Ok(())
}

impl Database {
    pub async fn insert_one_currency_code(&self, code: String) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO currency_codes (
            code
        ) VALUES (
            $1)",
        )
        .bind(code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_one_currency_code(&self, code: String) -> GlobeliseResult<()> {
        sqlx::query(
            "
        DELETE FROM currency_codes
        WHERE
            code = $1",
        )
        .bind(code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn select_many_currency_codes(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<CurrencyCode>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
        SELECT
            *
        FROM
            currency_codes
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
