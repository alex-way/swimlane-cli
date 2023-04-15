#[macro_use]
mod util;
pub mod apps;
pub mod error;
pub mod groups;
pub mod python;
pub mod roles;
pub mod tasks;
pub mod users;
pub mod workflows;
pub mod workspaces;

use error::SwimlaneClientError;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseEntity {
    pub id: String,
    pub name: String,
    pub disabled: bool,
}

#[derive(Clone)]
pub struct SwimlaneClient {
    http_client: Client,
    pub base_url: String,
}

impl SwimlaneClient {
    pub fn new(base_url: String, pat: String) -> Self {
        assert!(
            base_url.starts_with("https://"),
            "Invalid base url. Must start with https://"
        );

        let mut headers = HeaderMap::new();
        headers.insert("Private-Token", pat.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Could not build http client");

        Self {
            http_client,
            base_url,
        }
    }

    /// Pings the health endpoint and asserts a successful response.
    pub async fn health_ping(&self) -> Result<(), SwimlaneClientError> {
        let url = format!("{}/api/health/ping", self.base_url);
        let response = self.http_client.get(url).send().await?;
        response.error_for_status()?;
        Ok(())
    }
}
