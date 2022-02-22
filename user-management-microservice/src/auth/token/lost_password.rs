use time::Duration;

use crate::auth::error::Error;

use super::one_time::OneTimeTokenAudience;

#[derive(Debug)]
pub struct LostPasswordToken;

impl OneTimeTokenAudience for LostPasswordToken {
    fn name() -> &'static str {
        "lost_password"
    }

    fn from_str(s: &str) -> Result<(), Error> {
        match s {
            "lost_password" => Ok(()),
            _ => Err(Error::Unauthorized("")),
        }
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
