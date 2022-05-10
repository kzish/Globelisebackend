use rusty_ulid::Ulid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PaginatedQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
    pub contractor_ulid: Option<Ulid>,
    pub client_ulid: Option<Ulid>,
}
