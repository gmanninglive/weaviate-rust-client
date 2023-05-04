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
        self,
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
            .get(make_url(self.base_uri.clone(), path))
            .headers(headers)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn post(
        self,
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
            .post(make_url(self.base_uri.clone(), path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn put(
        self,
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
            .put(make_url(self.base_uri.clone(), path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn patch(
        self,
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
            .patch(make_url(self.base_uri.clone(), path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn delete(
        self,
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
            .delete(make_url(self.base_uri.clone(), path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn head(
        self,
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
            .head(make_url(self.base_uri.clone(), path))
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }
}

fn make_url(base: String, path: String) -> String {
    return format!("{}{}", base, path);
}

fn add_auth_header(mut headers: HeaderMap, bearer_token: Option<String>) -> HeaderMap {
    match bearer_token {
        Some(token) => {
            headers.insert(AUTHORIZATION, token.parse().unwrap());
        }
        None => {}
    }
    return headers;
}
