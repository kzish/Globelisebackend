use base64::decode;
use jsonwebtoken::DecodingKey;
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
init_global_static!(GLOBELISE_DOMAIN_URL);
init_global_static!(GLOBELISE_SMTP_URL);
init_global_static!(PASSWORD_RESET_URL);

pub static GLOBELISE_SENDER_EMAIL: Lazy<Mailbox> = Lazy::new(|| {
    std::env::var("GLOBELISE_SENDER_EMAIL")
        .expect("GLOBELISE_SMTP_USERNAME not set")
        .parse()
        .expect("GLOBELISE_SENDER_EMAIL not set properly")
});
pub static SMTP_CREDENTIAL: Lazy<SmtpCredentials> = Lazy::new(|| {
    SmtpCredentials::new(
        std::env::var("GLOBELISE_SMTP_USERNAME").expect("GLOBELISE_SMTP_USERNAME not set"),
        std::env::var("GLOBELISE_SMTP_PASSWORD").expect("GLOBELISE_SMTP_PASSWORD not set"),
    )
});

pub static GLOBELISE_EOR_ADMIN_MANAGEMENT_MICROSERVICE_PUBLIC_KEY: Lazy<DecodingKey> =
    Lazy::new(|| {
        let base64 = std::env::var("GLOBELISE_EOR_ADMIN_MANAGEMENT_MICROSERVICE_PUBLIC_KEY")
            .expect("GLOBELISE_EOR_ADMIN_MANAGEMENT_MICROSERVICE_PUBLIC_KEY must be set");
        let decoded = decode(base64).unwrap();
        DecodingKey::from_rsa_pem(&decoded).unwrap()
    });
