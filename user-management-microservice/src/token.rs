use common_utils::{token::TokenLike, DaprAppId};
use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use time::Duration;

use super::user::UserType;

/// Claims for access tokens.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserAccessToken {
    pub ulid: Ulid,
    pub email: String,
    pub user_type: UserType,
}

impl TokenLike for UserAccessToken {
    fn aud() -> &'static str {
        "access_token"
    }

    fn exp() -> Duration {
        Duration::minutes(60)
    }

    fn dapr_app_id() -> DaprAppId {
        DaprAppId::UserManagementMicroservice
    }
}
