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
        let headers = init_headers(
            &self,
            HeaderOptions {
                bearer_token,
                content_type: None,
            },
        );

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
        let headers = init_headers(
            &self,
            HeaderOptions {
                bearer_token,
                content_type: Some("application/json".to_string()),
            },
        );

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
        let headers = init_headers(
            &self,
            HeaderOptions {
                bearer_token,
                content_type: Some("application/json".to_string()),
            },
        );

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
        let headers = init_headers(
            &self,
            HeaderOptions {
                bearer_token,
                content_type: Some("application/json".to_string()),
            },
        );

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
        let headers = init_headers(
            &self,
            HeaderOptions {
                bearer_token,
                content_type: Some("application/json".to_string()),
            },
        );

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
        let headers = init_headers(
            &self,
            HeaderOptions {
                bearer_token,
                content_type: Some("application/json".to_string()),
            },
        );

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

struct HeaderOptions {
    bearer_token: Option<String>,
    content_type: Option<String>,
}

/// combines default headers with optional bearer_token and content_type headers
fn init_headers(client: &HttpClient, options: HeaderOptions) -> HeaderMap {
    let mut headers = client
        .config
        .headers
        .clone()
        .unwrap_or_else(|| HeaderMap::new());

    match options.bearer_token {
        Some(token) => {
            headers.insert(AUTHORIZATION, token.parse().unwrap());
        }
        None => {}
    }

    match options.content_type {
        Some(content_type) => {
            headers.insert(CONTENT_TYPE, content_type.parse().unwrap());
        }
        None => {}
    }

    return headers;
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::{command::meta::MetaResponse, connection::Connection};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get() {
        let mut server = mockito::Server::new();
        let mock_response = MetaResponse {
            hostname: "http://[::]:8080".to_string(),
            modules: HashMap::new(),
            version: "1.19.0".to_string(),
        };

        let mock = server
            .mock("GET", "/v1/meta")
            .with_body(
                serde_json::to_string(&mock_response).expect("error serializing mock response"),
            )
            .create();

        let client = HttpClient::new(ConnectionParams {
            api_key: "".to_string(),
            host: server.host_with_port(),
            scheme: "http".to_string(),
            headers: None,
            auth_client_secret: None,
        });

        let response = client
            .get("/meta".to_string(), None)
            .await
            .expect("error fetching meta data")
            .json::<MetaResponse>()
            .await
            .expect("error deserializing met data");

        mock.assert();

        assert_eq!(mock_response, response);
    }
}
