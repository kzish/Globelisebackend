use rusty_ulid::Ulid;
use serde::Deserialize;
use std::num::NonZeroU32;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedQuery {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub query: Option<String>,
    pub contractor_ulid: Option<Ulid>,
    pub client_ulid: Option<Ulid>,
}
