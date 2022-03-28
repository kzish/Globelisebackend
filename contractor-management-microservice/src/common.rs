use rusty_ulid::Ulid;
use serde::Deserialize;
use std::num::NonZeroU32;

#[derive(Debug, Deserialize)]
pub struct PaginatedQuery {
    #[serde(default = "PaginatedQuery::default_page")]
    pub page: NonZeroU32,
    #[serde(default = "PaginatedQuery::default_per_page")]
    pub per_page: NonZeroU32,
    pub query: Option<String>,
    pub contractor_ulid: Option<Ulid>,
    pub client_ulid: Option<Ulid>,
}

impl PaginatedQuery {
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
