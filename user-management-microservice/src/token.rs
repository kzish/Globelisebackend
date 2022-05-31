use common_utils::{token::TokenLike, DaprAppId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::Duration;
use uuid::Uuid;

use crate::user::UserRole;

use super::user::UserType;

/// Claims for access tokens.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserAccessToken {
    pub ulid: Uuid,
    pub email: String,
    pub user_type: UserType,
    pub user_roles: Vec<UserRole>,
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
