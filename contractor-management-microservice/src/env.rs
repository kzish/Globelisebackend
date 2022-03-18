use axum::http::HeaderValue;
use once_cell::sync::Lazy;

macro_rules! init_global_static {
    ($name:ident) => {
        pub static $name: Lazy<String> = Lazy::new(|| {
            std::env::var(stringify!($name)).expect(&format!("{} must be set", stringify!($name)))
        });
    };
}

init_global_static!(LISTENING_ADDRESS);
init_global_static!(USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL);

pub static FRONTEND_URL: Lazy<HeaderValue> = Lazy::new(|| {
    std::env::var("FRONTEND_URL")
        .expect("FRONTEND_URL not set")
        .parse()
        .expect("FRONTEND_URL not set properly")
});
