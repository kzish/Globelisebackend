use axum::{
    extract::{ContentLengthLimit, Query},
    Extension, Json,
};
use common_utils::{
    calc_limit_and_offset,
    custom_serde::{Country, FORM_DATA_LENGTH_LIMIT},
    error::GlobeliseResult,
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
pub struct GetManyEntityTypeQuery {
    country_code: Option<Country>,
    per_page: Option<u32>,
    page: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PutOneEntityTypeCode {
    entity_name: String,
    country_code: Country,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteOneEntityTypeCode {
    entity_name: String,
    country_code: Country,
}

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntityType {
    entity_name: String,
    country_code: Country,
}

pub async fn get_many(
    Query(query): Query<GetManyEntityTypeQuery>,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<Json<Vec<EntityType>>> {
    let database = database.lock().await;

    let result = database
        .select_many_entity_type(query.country_code, query.page, query.per_page)
        .await?;

    Ok(Json(result))
}

pub async fn post_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<PutOneEntityTypeCode>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .insert_one_entity_type(body.entity_name, body.country_code)
        .await?;

    Ok(())
}

pub async fn delete_one(
    _: Token<AdminAccessToken>,
    ContentLengthLimit(Json(body)): ContentLengthLimit<
        Json<DeleteOneEntityTypeCode>,
        FORM_DATA_LENGTH_LIMIT,
    >,
    Extension(database): Extension<SharedDatabase>,
) -> GlobeliseResult<()> {
    let database = database.lock().await;

    database
        .delete_one_entity_type(body.entity_name, body.country_code)
        .await?;

    Ok(())
}

impl Database {
    pub async fn insert_one_entity_type(
        &self,
        entity_name: String,
        country_code: Country,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        INSERT INTO entity_types (
            entity_name, country_code
        ) VALUES (
            $1, $2)",
        )
        .bind(entity_name)
        .bind(country_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn delete_one_entity_type(
        &self,
        entity_name: String,
        country_code: Country,
    ) -> GlobeliseResult<()> {
        sqlx::query(
            "
        DELETE FROM entity_types
        WHERE
            entity_name = $1 AND
            country_code = $2",
        )
        .bind(entity_name)
        .bind(country_code)
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn select_many_entity_type(
        &self,
        country_code: Option<Country>,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> GlobeliseResult<Vec<EntityType>> {
        let (limit, offset) = calc_limit_and_offset(per_page, page);

        let result = sqlx::query_as(
            "
        SELECT
            *
        FROM
            entity_types
        WHERE
            ($1 IS NULL OR country_code = $1)
        LIMIT
            $2
        OFFSET
            $3",
        )
        .bind(country_code)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.0)
        .await?;

        Ok(result)
    }
}
