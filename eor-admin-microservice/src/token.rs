use common_utils::{token::TokenLike, DaprAppId};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use time::Duration;

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdminAccessToken {
    pub ulid: Ulid,
    pub email: String,
}

impl TokenLike for AdminAccessToken {
    fn aud() -> &'static str {
        "access_token"
    }

    fn exp() -> Duration {
        Duration::minutes(60)
    }

    fn dapr_app_id() -> DaprAppId {
        DaprAppId::EorAdminMicroservice
    }
}
