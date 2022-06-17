use serde::Deserialize;
use serde_with::serde_as;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PaginatedQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search_text: Option<String>,
    pub contractor_ulid: Option<Uuid>,
    pub client_ulid: Option<Uuid>,
}
