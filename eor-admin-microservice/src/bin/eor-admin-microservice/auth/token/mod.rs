//! Functions and types for handling authorization tokens.

use std::{fs::File, io::Read};

use common_utils::{
    token::{Keys, TokenLike},
    DaprAppId,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::Duration;
use uuid::Uuid;

pub mod one_time;

/// Claims for refresh tokens.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RefreshToken {
    pub ulid: Uuid,
}

impl TokenLike for RefreshToken {
    fn aud() -> &'static str {
        "refresh_token"
    }

    fn exp() -> Duration {
        Duration::minutes(60)
    }

    fn dapr_app_id() -> DaprAppId {
        DaprAppId::EorAdminMicroservice
    }
}

/// The public key used for decoding tokens.
pub static PUBLIC_KEY: Lazy<String> = Lazy::new(|| {
    let mut public_key = String::new();
    File::open("public.pem")
        .expect("Could not open public key")
        .read_to_string(&mut public_key)
        .expect("Could not read public key");
    public_key
});

/// The encoding/decoding key pair.
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let mut private_key: Vec<u8> = Vec::new();
    File::open("private.pem")
        .expect("Could not open private key")
        .read_to_end(&mut private_key)
        .expect("Could not read private key");
    Keys::new(&private_key, PUBLIC_KEY.as_bytes())
});
