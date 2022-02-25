//! Abstractions for interfacing with the Dapr state store.

use std::{sync::Arc, time::Duration};

use dapr::{
    client::{HttpMethod, InvokeServiceResponse},
    dapr::dapr::proto::runtime::v1::dapr_client::DaprClient,
    Client,
};

use serde::Deserialize;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::{
    error::Error,
    microservices::user_management::{UserIndex, USER_MANAGEMENT_MICROSERVICE},
};

pub type SharedState = Arc<Mutex<State>>;

/// Convenience wrapper around the Dapr client.
pub struct State {
    dapr_client: Client<DaprClient<Channel>>,
}

impl State {
    /// Connects to Dapr.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // We must wait for the Dapr gRPC port to be assigned before connecting.
        std::thread::sleep(Duration::from_secs(2));

        let dapr_port: u16 = std::env::var("DAPR_GRPC_PORT")?.parse()?;
        let dapr_address = format!("https://127.0.0.1:{}", dapr_port);
        let dapr_client = dapr::Client::<dapr::client::TonicClient>::connect(dapr_address).await?;
        Ok(Self { dapr_client })
    }

    pub async fn index<I: Iterator<Item = (String, String)>>(
        &mut self,
        params: I,
    ) -> Result<Vec<UserIndex>, Error> {
        // NOTE: DAPR will currently panic if the underlying GRPC call fails...
        // Please fix this.
        let result: InvokeServiceResponse = self
            .dapr_client
            .invoke_service_http(
                USER_MANAGEMENT_MICROSERVICE,
                "index",
                HttpMethod::Get,
                "text/html",
                params
                    .map(|(l, r)| format!("{}={}", l, r))
                    .collect::<Vec<String>>()
                    .join("&"),
                None,
            )
            .await?;
        deserialize_invoke_service_response(&result).unwrap_or_else(|| Ok(vec![]))
    }
}

fn deserialize_invoke_service_response<'a, T>(
    resp: &'a InvokeServiceResponse,
) -> Option<Result<T, Error>>
where
    T: Deserialize<'a>,
{
    match &resp.data {
        Some(v) => {
            let slice = v.value.as_slice();
            let value =
                serde_json::from_slice::<'a, T>(slice).map_err(|e| Error::Internal(e.to_string()));
            Some(value)
        }
        None => None,
    }
}
