use time::Duration;

use crate::auth::error::Error;

use super::one_time::OneTimeTokenAudience;

#[derive(Debug)]
pub struct ChangePasswordToken;

impl OneTimeTokenAudience for ChangePasswordToken {
    fn name() -> &'static str {
        "change_password"
    }

    fn from_str(s: &str) -> Result<(), Error> {
        match s {
            "change_password" => Ok(()),
            _ => Err(Error::Unauthorized("")),
        }
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
