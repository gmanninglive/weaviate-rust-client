use super::ConnectionParams;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use serde_json::Value;

pub struct HttpClient {
    config: ConnectionParams,
    client: reqwest::Client,
    base_uri: String,
}

impl HttpClient {
    pub fn new(config: ConnectionParams) -> Self {
        Self {
            base_uri: format!("{}://{}/v1", config.scheme, config.host),
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(
        &self,
        path: String,
        bearer_token: Option<String>,
    ) -> Result<Response, anyhow::Error> {
        let mut headers = self
            .config
            .headers
            .clone()
            .unwrap_or_else(|| HeaderMap::new());

        headers = add_auth_header(headers, bearer_token);

        let response = self
            .client
            .get(self.fmt_url(path))
            .headers(headers)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn post(
        &self,
        path: String,
        payload: Value,
        bearer_token: Option<String>,
    ) -> Result<Response, anyhow::Error> {
        let mut headers = self
            .config
            .headers
            .clone()
            .unwrap_or_else(|| HeaderMap::new());

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers = add_auth_header(headers, bearer_token);

        let response = self
            .client
            .post(self.fmt_url(path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn put(
        &self,
        path: String,
        payload: Value,
        bearer_token: Option<String>,
    ) -> Result<Response, anyhow::Error> {
        let mut headers = self
            .config
            .headers
            .clone()
            .unwrap_or_else(|| HeaderMap::new());

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers = add_auth_header(headers, bearer_token);

        let response = self
            .client
            .put(self.fmt_url(path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn patch(
        &self,
        path: String,
        payload: Value,
        bearer_token: Option<String>,
    ) -> Result<Response, anyhow::Error> {
        let mut headers = self
            .config
            .headers
            .clone()
            .unwrap_or_else(|| HeaderMap::new());

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers = add_auth_header(headers, bearer_token);

        let response = self
            .client
            .patch(self.fmt_url(path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn delete(
        &self,
        path: String,
        payload: Option<Value>,
        bearer_token: Option<String>,
    ) -> Result<Response, anyhow::Error> {
        let mut headers = self
            .config
            .headers
            .clone()
            .unwrap_or_else(|| HeaderMap::new());

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers = add_auth_header(headers, bearer_token);

        let response = self
            .client
            .delete(self.fmt_url(path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn head(
        &self,
        path: String,
        payload: Option<Value>,
        bearer_token: Option<String>,
    ) -> Result<Response, anyhow::Error> {
        let mut headers = self
            .config
            .headers
            .clone()
            .unwrap_or_else(|| HeaderMap::new());

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers = add_auth_header(headers, bearer_token);

        let response = self
            .client
            .head(self.fmt_url(path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    /// Joins base_uri with path param
    fn fmt_url(&self, path: String) -> String {
        return format!("{}{}", self.base_uri, path);
    }
}

/// inserts the `Authorization` header if the bearer_token is `Some`
fn add_auth_header(mut headers: HeaderMap, bearer_token: Option<String>) -> HeaderMap {
    match bearer_token {
        Some(token) => {
            headers.insert(AUTHORIZATION, token.parse().unwrap());
        }
        None => {}
    }
    return headers;
}
