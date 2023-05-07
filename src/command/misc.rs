use super::Command;
use crate::{http::HttpClient, utils::db_version::DbVersionProvider, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Misc {
    pub get_meta: MetaGetter,
    pub check_live: LiveChecker,
    pub get_openid_configuration: OpenidConfigurationGetter,
}

#[derive(Clone)]
pub struct MetaGetter {
    /// The client's connection
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
    /// The client's connection
    client: HttpClient,
    db_version_provider: DbVersionProvider,
}

impl LiveChecker {
    pub fn new(conn: &Connection, db_version_provider: DbVersionProvider) -> Self {
        Self {
            client: conn.client.clone(),
            db_version_provider,
        }
    }
}

#[async_trait::async_trait]
impl Command<bool> for LiveChecker {
    async fn r#do(&self) -> Result<bool, anyhow::Error> {
        let well_known = self.client.get("/.well-known/live").await.is_ok();
        let version = self.db_version_provider.clone().get().await;

        Ok(well_known && version.is_ok())
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
