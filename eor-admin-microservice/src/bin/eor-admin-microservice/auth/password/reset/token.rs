use time::Duration;

use crate::auth::token::one_time::OneTimeTokenAudience;

#[derive(Debug)]
pub struct LostPasswordToken;

impl OneTimeTokenAudience for LostPasswordToken {
    fn name() -> &'static str {
        "eor_admin_microservice_lost_password"
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
#[derive(Debug)]
pub struct ChangePasswordToken;

impl OneTimeTokenAudience for ChangePasswordToken {
    fn name() -> &'static str {
        "eor_admin_microservice_change_password"
    }

    fn lifetime() -> Duration {
        Duration::minutes(60)
    }
}
