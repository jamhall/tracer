use std::str::FromStr;

use reqwest::{Client, header, Response};
use serde_json::json;

use crate::common::{Command, CommandCreate, Session};
use crate::error::ApplicationError;

pub struct ApiClient {
    url: String,
    client: Client,
}

pub trait RequestExt {
    fn is_successful(&self) -> bool;
}

impl RequestExt for Response {
    fn is_successful(&self) -> bool {
        self.status().is_success()
    }
}

impl ApiClient {

    pub fn new(url: &str, token: &str) -> Result<Self, ApplicationError> {
        let default_headers = Self::default_headers(token)?;
        let client = Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Self {
            url: url.into(),
            client,
        })
    }

    fn default_headers(
        token: &str,
    ) -> Result<header::HeaderMap<header::HeaderValue>, ApplicationError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        Ok(headers)
    }

    pub async fn commands(&self) -> Result<Option<Vec<Command>>, ApplicationError> {
        let url = format!("{}/commands", self.url);
        let response = self.client.get(url).send().await?;
        if response.is_successful() {
            let commands = response.json().await?;
            return Ok(Some(commands));
        }
        Ok(None)
    }

    pub async fn create_command(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<Command>, ApplicationError> {
        let url = format!("{}/commands", self.url);
        let response = self
            .client
            .post(url)
            .json(&json!({
                "name": name.as_ref()
            }))
            .send()
            .await?;
        if response.is_successful() {
            let command = response.json().await?;
            return Ok(Some(command));
        }
        Ok(None)
    }

    pub async fn create_session(
        &self,
        command: &Command,
    ) -> Result<Option<Session>, ApplicationError> {
        let url = format!("{}/commands/{}/sessions", self.url, command.id());
        let response = self.client.post(url).send().await?;
        if response.is_successful() {
            let session = response.json().await?;
            return Ok(Some(session));
        }
        Ok(None)
    }
}
