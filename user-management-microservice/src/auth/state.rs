//! Abstractions for interfacing with the Dapr state store.

use std::{collections::HashMap, sync::Arc, time::Duration};

use argon2::{hash_encoded, verify_encoded};
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};

use rand::Rng;
use rusty_ulid::Ulid;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use super::{
    error::Error,
    token::{
        create_refresh_token,
        one_time::{create_one_time_token, OneTimeTokenAudience},
    },
    user::Role,
    Database, HASH_CONFIG,
};

pub type SharedState = Arc<Mutex<State>>;

/// Convenience wrapper around the Dapr client.
pub struct State {
    dapr_client: Client<DaprClient<Channel>>,
}

impl State {
    /// The state store name.
    const STATE_STORE: &'static str = "state_store";
    /// The category name for sessions.
    const SESSION_CATEGORY: &'static str = "sessions";

    /// Connects to Dapr.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // We must wait for the Dapr gRPC port to be assigned before connecting.
        std::thread::sleep(Duration::from_secs(2));

        let dapr_port: u16 = std::env::var("DAPR_GRPC_PORT")?.parse()?;
        let dapr_address = format!("https://127.0.0.1:{}", dapr_port);
        let dapr_client = dapr::Client::<dapr::client::TonicClient>::connect(dapr_address).await?;
        Ok(Self { dapr_client })
    }

    /// Opens a new session for a user.
    ///
    /// Returns the refresh token for the session.
    pub async fn open_session(
        &mut self,
        database: &Database,
        ulid: Ulid,
        role: Role,
    ) -> Result<String, Error> {
        // Validate that the user and role are correct.
        if database.user(ulid, Some(role)).await?.is_none() {
            return Err(Error::Unauthorized);
        }

        let mut sessions = Sessions::default();
        if let Some(existing_sessions) = self.sessions(ulid).await? {
            sessions = existing_sessions;
        }
        let refresh_token = sessions.open(ulid, role)?;
        self.serialize(Self::SESSION_CATEGORY, &ulid.to_string(), sessions)
            .await?;
        Ok(refresh_token)
    }

    /// Revoke all sessions for a user.
    pub async fn revoke_all_sessions(&mut self, ulid: Ulid) -> Result<(), Error> {
        if let Some(mut sessions) = self.sessions(ulid).await? {
            sessions.revoke_all();
            self.serialize(Self::SESSION_CATEGORY, &ulid.to_string(), sessions)
                .await
        } else {
            Ok(())
        }
    }

    /// Clears expired sessions for a user.
    pub async fn clear_expired_sessions(&mut self, ulid: Ulid) -> Result<(), Error> {
        if let Some(mut sessions) = self.sessions(ulid).await? {
            sessions.clear_expired();
            self.serialize(Self::SESSION_CATEGORY, &ulid.to_string(), sessions)
                .await
        } else {
            Ok(())
        }
    }

    /// Gets existing sessions for a user.
    pub async fn sessions(&mut self, ulid: Ulid) -> Result<Option<Sessions>, Error> {
        self.deserialize(Self::SESSION_CATEGORY, &ulid.to_string())
            .await
    }

    /// Opens a new one-time session for a user.
    pub async fn open_one_time_session<T>(
        &mut self,
        database: &Database,
        ulid: Ulid,
        role: Role,
    ) -> Result<String, Error>
    where
        T: OneTimeTokenAudience,
    {
        // Validate that the user and role are correct.
        if database.user(ulid, Some(role)).await?.is_none() {
            return Err(Error::Unauthorized);
        }

        let mut sessions = OneTimeSessions::default();
        let category = Self::one_time_session_category::<T>();
        if let Some(existing_sessions) = self
            .deserialize::<OneTimeSessions>(&*category, &ulid.to_string())
            .await?
        {
            sessions = existing_sessions;
        }
        let one_time_token = sessions.open::<T>(ulid, role)?;
        self.serialize(&*category, &ulid.to_string(), sessions)
            .await?;
        Ok(one_time_token)
    }

    /// Checks if a one-time token is valid.
    pub async fn is_one_time_token_valid<T>(
        &mut self,
        ulid: Ulid,
        token: &[u8],
    ) -> Result<bool, Error>
    where
        T: OneTimeTokenAudience,
    {
        let category = Self::one_time_session_category::<T>();
        if let Some(mut sessions) = self
            .deserialize::<OneTimeSessions>(&*category, &ulid.to_string())
            .await?
        {
            let mut matching_hash: Option<String> = None;
            sessions.clear_expired();

            for (hash, _) in sessions.iter() {
                if let Ok(true) = verify_encoded(hash, token) {
                    matching_hash = Some(hash.clone());
                    break;
                }
            }

            if let Some(hash) = matching_hash {
                sessions.sessions.remove(&hash);
                self.serialize(&*category, &ulid.to_string(), sessions)
                    .await?;
                return Ok(true);
            } else {
                return Ok(false);
            }
        }

        Ok(false)
    }

    /// Serializes and stores data in the state store.
    async fn serialize<T>(&mut self, category: &str, key: &str, value: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let prefixed_key = category.to_string() + "--" + key;
        let value = serde_json::to_vec(&value).map_err(|e| Error::Conversion(e.to_string()))?;
        self.dapr_client
            .save_state(Self::STATE_STORE, vec![(&*prefixed_key, value)])
            .await
            .map_err(|e| Error::Dapr(e.to_string()))?;
        Ok(())
    }

    /// Deserializes data from the state store.
    async fn deserialize<T>(&mut self, category: &str, key: &str) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        let prefixed_key = category.to_string() + "--" + key;
        let result = self
            .dapr_client
            .get_state(Self::STATE_STORE, &*prefixed_key, None)
            .await
            .map_err(|e| Error::Dapr(e.to_string()))?;

        if !result.data.is_empty() {
            let value: T = serde_json::from_slice(&result.data)
                .map_err(|e| Error::Conversion(e.to_string()))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Gets the store name for one-time sessions.
    fn one_time_session_category<T>() -> String
    where
        T: OneTimeTokenAudience,
    {
        "one_time_".to_string() + T::name()
    }
}

/// Stores hashes of session tokens, mapped to their expiration time.
#[derive(Default, Deserialize, Serialize)]
pub struct Sessions {
    sessions: HashMap<String, i64>,
}

impl Sessions {
    /// Opens a new session.
    ///
    /// Returns the refresh token for the session.
    fn open(&mut self, ulid: Ulid, role: Role) -> Result<String, Error> {
        let (refresh_token, expiration) = create_refresh_token(ulid, role)?;
        let salt: [u8; 16] = rand::thread_rng().gen();
        let hash = hash_encoded(refresh_token.as_bytes(), &salt, &HASH_CONFIG)
            .map_err(|_| Error::Internal)?;
        self.sessions.insert(hash, expiration);
        Ok(refresh_token)
    }

    /// Revokes all sessions.
    fn revoke_all(&mut self) {
        self.sessions.clear();
    }

    /// Clears all expired sessions.
    fn clear_expired(&mut self) {
        self.sessions
            .retain(|_, expiration| *expiration > OffsetDateTime::now_utc().unix_timestamp());
    }

    /// Produces an iterator over the sessions.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &i64)> {
        self.sessions.iter()
    }
}

/// Stores hashes of session tokens, mapped to their expiration time.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OneTimeSessions {
    sessions: HashMap<String, i64>,
}

impl OneTimeSessions {
    /// Opens a new session.
    ///
    /// Returns the refresh token for the session.
    fn open<T>(&mut self, ulid: Ulid, role: Role) -> Result<String, Error>
    where
        T: OneTimeTokenAudience,
    {
        let (one_time_token, expiration) = create_one_time_token::<T>(ulid, role)?;
        let salt: [u8; 16] = rand::thread_rng().gen();
        let hash = hash_encoded(one_time_token.as_bytes(), &salt, &HASH_CONFIG)
            .map_err(|_| Error::Internal)?;
        self.sessions.insert(hash, expiration);
        Ok(one_time_token)
    }

    /// Clears all expired sessions.
    fn clear_expired(&mut self) {
        self.sessions
            .retain(|_, expiration| *expiration < OffsetDateTime::now_utc().unix_timestamp());
    }

    /// Produces an iterator over the sessions.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &i64)> {
        self.sessions.iter()
    }
}
