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
