use lettre::{message::Mailbox, transport::smtp::authentication::Credentials as SmtpCredentials};
use once_cell::sync::Lazy;

macro_rules! init_global_static {
    // `()` indicates that the macro takes no argument.
    ($name:ident) => {
        pub static $name: Lazy<String> = Lazy::new(|| {
            std::env::var(stringify!($name)).expect(&format!("{} must be set", stringify!($name)))
        });
    };
}

init_global_static!(LISTENING_ADDRESS);
init_global_static!(DAPR_ADDRESS);
init_global_static!(USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL);
init_global_static!(GLOBELISE_SMTP_URL);
init_global_static!(MULESOFT_API_URL);
init_global_static!(MULESOFT_CLIENT_ID);
init_global_static!(MULESOFT_CLIENT_SECRET);
init_global_static!(FRONTEND_URL);
init_global_static!(GOOGLE_CLIENT_ID);
init_global_static!(DATABASE_URL);

pub static GLOBELISE_SENDER_EMAIL: Lazy<Mailbox> = Lazy::new(|| {
    std::env::var("GLOBELISE_SENDER_EMAIL")
        .expect("GLOBELISE_SENDER_EMAIL not set")
        .parse()
        .expect("GLOBELISE_SENDER_EMAIL not set properly")
});

pub static SMTP_CREDENTIAL: Lazy<SmtpCredentials> = Lazy::new(|| {
    SmtpCredentials::new(
        std::env::var("GLOBELISE_SMTP_USERNAME").expect("GLOBELISE_SMTP_USERNAME not set"),
        std::env::var("GLOBELISE_SMTP_PASSWORD").expect("GLOBELISE_SMTP_PASSWORD not set"),
    )
});
