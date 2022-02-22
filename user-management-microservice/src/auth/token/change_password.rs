use time::Duration;

use super::one_time::OneTimeTokenAudience;

#[derive(Debug)]
pub struct ChangePasswordToken;

impl OneTimeTokenAudience for ChangePasswordToken {
    fn name() -> &'static str {
        "change_password"
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
