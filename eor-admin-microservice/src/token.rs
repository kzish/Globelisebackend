use common_utils::{custom_serde::EmailWrapper, token::TokenLike, DaprAppId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::Duration;
use uuid::Uuid;

/// Claims for access tokens.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdminAccessToken {
    pub ulid: Uuid,
    pub email: EmailWrapper,
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
