use time::Duration;

use super::one_time::OneTimeTokenAudience;

#[derive(Debug)]
pub struct LostPasswordToken;

impl OneTimeTokenAudience for LostPasswordToken {
    fn name() -> &'static str {
        "lost_password"
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
