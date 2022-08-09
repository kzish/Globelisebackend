use crate::database::Database;
use common_utils::error::GlobeliseResult;
use sqlx::types::Uuid;

use super::users::GlobeliseUser;

impl Database {
    pub async fn get_globelise_user(&self, ulid: Uuid) -> GlobeliseResult<GlobeliseUser> {
        let result = sqlx::query_as(
            "
            SELECT
                *
            FROM
                users
            WHERE
                ulid = $1",
        )
        .bind(ulid)
        .fetch_one(&self.0)
        .await?;

        Ok(result)
    }
}
