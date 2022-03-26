use rusty_ulid::Ulid;
use serde::Deserialize;
use std::num::NonZeroU32;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "PaginationQuery::default_page")]
    pub page: NonZeroU32,
    #[serde(default = "PaginationQuery::default_per_page")]
    pub per_page: NonZeroU32,
    pub search_text: Option<String>,
}

impl PaginationQuery {
    fn default_page() -> NonZeroU32 {
        NonZeroU32::new(1).unwrap()
    }

    fn default_per_page() -> NonZeroU32 {
        NonZeroU32::new(50).unwrap()
    }
}

pub fn ulid_to_sql_uuid(ulid: Ulid) -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(ulid.into())
}

pub fn ulid_from_sql_uuid(uuid: sqlx::types::Uuid) -> Ulid {
    Ulid::from(*uuid.as_bytes())
}
