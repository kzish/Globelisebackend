use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationQuery {
    #[serde(default = "PaginationQuery::default_page")]
    pub page: i64,
    #[serde(default = "PaginationQuery::default_per_page")]
    pub per_page: i64,
    pub search_text: Option<String>,
}

impl PaginationQuery {
    fn default_page() -> i64 {
        1
    }

    fn default_per_page() -> i64 {
        25
    }
}

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
