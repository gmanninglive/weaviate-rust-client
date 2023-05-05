use super::{auth::Auth, auth::OidcAuthenticator, Headers};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use serde_json::Value;

pub struct HttpParams {
    pub host: String,
    pub scheme: String,
    pub headers: Option<Headers>,
    pub auth: Auth,
}

pub struct HttpClient {
    client: reqwest::Client,
    base_uri: String,
    host: String,
    scheme: String,
    headers: Option<Headers>,
    auth: Auth,
}

impl HttpClient {
    pub fn new(params: HttpParams) -> Self {
        Self {
            base_uri: format!("{}://{}/v1", params.scheme, params.host),
            client: reqwest::Client::new(),
            scheme: params.scheme,
            host: params.host,
            auth: params.auth,
            headers: params.headers,
        }
    }

    async fn login(&self) {
        if let Auth::Oidc(config) = &self.auth {
            Auth::Oidc(OidcAuthenticator::new(&config.credentials).refresh());
        }
    }

    pub async fn get(&self, path: String) -> Result<Response, anyhow::Error> {
        self.login().await;

        let headers = init_headers(self, HeaderOptions { content_type: None });

        let response = self
            .client
            .get(self.fmt_url(path))
            .headers(headers)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn post(&self, path: String, payload: Value) -> Result<Response, anyhow::Error> {
        self.login().await;

        let headers = init_headers(
            self,
            HeaderOptions {
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

    pub async fn put(&self, path: String, payload: Value) -> Result<Response, anyhow::Error> {
        self.login().await;

        let headers = init_headers(
            self,
            HeaderOptions {
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

    pub async fn patch(&self, path: String, payload: Value) -> Result<Response, anyhow::Error> {
        self.login().await;

        let headers = init_headers(
            self,
            HeaderOptions {
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
    ) -> Result<Response, anyhow::Error> {
        self.login().await;

        let headers = init_headers(
            self,
            HeaderOptions {
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
    ) -> Result<Response, anyhow::Error> {
        self.login().await;

        let headers = init_headers(
            self,
            HeaderOptions {
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
        format!("{}{}", self.base_uri, path)
    }
}

struct HeaderOptions {
    content_type: Option<String>,
}

/// combines default headers with optional bearer_token and content_type headers
fn init_headers(client: &HttpClient, options: HeaderOptions) -> HeaderMap {
    let mut headers = client.headers.clone().unwrap_or_else(HeaderMap::new);

    match &client.auth {
        Auth::Oidc(oidc) => {
            let token = oidc.token.clone().expect("access token missing");
            let token = format!("{} {}", token.token_type, token.access_token);

            headers.insert(AUTHORIZATION, token.parse().unwrap());
        }
        Auth::ApiKey(key) => {
            headers.insert(AUTHORIZATION, key.parse().unwrap());
        }
        Auth::None => {}
    }

    if let Some(content_type) = options.content_type {
        headers.insert(CONTENT_TYPE, content_type.parse().unwrap());
    }

    headers
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

        let client = HttpClient::new(HttpParams {
            host: server.host_with_port(),
            scheme: "http".to_string(),
            headers: None,
            auth: Auth::None,
        });

        let response = client
            .get("/meta".to_string())
            .await
            .expect("error fetching meta data")
            .json::<MetaResponse>()
            .await
            .expect("error deserializing met data");

        mock.assert();

        assert_eq!(mock_response, response);
    }
}
