//! Types for user data.

use common_utils::{
    custom_serde::{EmailWrapper, UserType},
    error::{GlobeliseError, GlobeliseResult},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub ulid: Uuid,
    pub email: EmailWrapper,
    pub password: Option<String>,
    pub is_google: bool,
    pub is_outlook: bool,
    pub is_entity: bool,
    pub is_individual: bool,
    pub is_client: bool,
    pub is_contractor: bool,
}

impl User {
    pub fn user_type(&self) -> GlobeliseResult<UserType> {
        if self.is_individual {
            Ok(UserType::Individual)
        } else if self.is_entity {
            Ok(UserType::Entity)
        } else {
            Err(GlobeliseError::internal("User is not configured properly"))
        }
    }
}
