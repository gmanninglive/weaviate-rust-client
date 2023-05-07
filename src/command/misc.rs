use super::Command;
use crate::{http::HttpClient, utils::db_version::DbVersionProvider, Connection};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ## Misc Commands
pub struct Misc<'a> {
    conn: &'a Connection,
}

impl<'a> Misc<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_meta(self) -> MetaGetter {
        MetaGetter::new(self.conn)
    }

    pub fn check_live(self) -> LiveChecker {
        LiveChecker::new(self.conn)
    }

    pub fn check_ready(self) -> ReadyChecker {
        ReadyChecker::new(self.conn)
    }

    pub fn get_openid_configuration(self) -> OpenidConfigurationGetter {
        OpenidConfigurationGetter::new(self.conn)
    }
}

#[derive(Clone)]
pub struct MetaGetter {
    client: HttpClient,
}

impl MetaGetter {
    pub fn new(conn: &Connection) -> Self {
        Self {
            client: conn.client.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleInfo {
    version: String,
    word_count: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MetaResponse {
    pub hostname: String,
    pub modules: HashMap<String, ModuleInfo>,
    pub version: String,
}

#[async_trait::async_trait]
impl Command<MetaResponse> for MetaGetter {
    async fn r#do(&self) -> Result<MetaResponse, anyhow::Error> {
        let res: MetaResponse = self.client.get("/meta".to_string()).await?.json().await?;
        Ok(res)
    }

    fn validate() {
        unimplemented!()
    }
}

pub struct LiveChecker {
    client: HttpClient,
    db_version_provider: DbVersionProvider,
}

impl LiveChecker {
    pub fn new(conn: &Connection) -> Self {
        Self {
            client: conn.client.clone(),
            db_version_provider: DbVersionProvider::new(conn),
        }
    }
}

#[async_trait::async_trait]
impl Command<bool> for LiveChecker {
    async fn r#do(&self) -> Result<bool, anyhow::Error> {
        let well_known = self.client.get("/.well-known/live").await?.status();

        let version = self.db_version_provider.clone().get().await?;

        Ok(StatusCode::le(&well_known, &StatusCode::BAD_REQUEST) && !version.is_empty())
    }

    fn validate() {
        unimplemented!()
    }
}

pub struct ReadyChecker {
    client: HttpClient,
    db_version_provider: DbVersionProvider,
}

impl ReadyChecker {
    pub fn new(conn: &Connection) -> Self {
        Self {
            client: conn.client.clone(),
            db_version_provider: DbVersionProvider::new(conn),
        }
    }
}

#[async_trait::async_trait]
impl Command<bool> for ReadyChecker {
    async fn r#do(&self) -> Result<bool, anyhow::Error> {
        let well_known = self.client.get("/.well-known/ready").await?;

        let version = self.db_version_provider.clone().get().await?;

        Ok(StatusCode::le(&well_known.status(), &StatusCode::BAD_REQUEST) && !version.is_empty())
    }

    fn validate() {
        unimplemented!()
    }
}

pub struct OpenidConfigurationGetter {
    client: HttpClient,
}

impl OpenidConfigurationGetter {
    pub fn new(conn: &Connection) -> Self {
        Self {
            client: conn.client.clone(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct OpenIdConfiguration {}

#[async_trait::async_trait]
impl Command<OpenIdConfiguration> for OpenidConfigurationGetter {
    async fn r#do(&self) -> Result<OpenIdConfiguration, anyhow::Error> {
        Ok(self
            .client
            .get("/.well-known/openid-configuration")
            .await?
            .json::<OpenIdConfiguration>()
            .await?)
    }

    fn validate() {
        unimplemented!()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::connection::{http::HttpParams, Auth, AuthParams, Connection, ConnectionBuilder};

    #[tokio::test]
    async fn test_meta_getter_do() {
        let mut server = mockito::Server::new();
        let response = MetaResponse {
            hostname: "http://[::]:8080".to_string(),
            modules: HashMap::new(),
            version: "1.19.0".to_string(),
        };

        let mock = server
            .mock("GET", "/v1/meta")
            .with_body(serde_json::to_string(&response).expect("error serializing mock response"))
            .create();

        let conn = ConnectionBuilder::new("http", server.host_with_port()).build();

        let meta = MetaGetter::new(&conn)
            .r#do()
            .await
            .expect("error fetching meta data");

        mock.assert();
        assert_eq!(response, meta);
    }
}
