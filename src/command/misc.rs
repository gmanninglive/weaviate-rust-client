use super::Command;
use crate::prelude::*;
use crate::Connection;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ## Misc Commands
#[derive(derive_new::new)]
pub struct Misc<'a> {
    conn: &'a Connection,
}

impl<'a> Misc<'a> {
    pub fn get_meta(self) -> MetaGetter<'a> {
        MetaGetter::new(self.conn)
    }

    pub fn check_live(self) -> LiveChecker<'a> {
        LiveChecker::new(self.conn)
    }

    pub fn check_ready(self) -> ReadyChecker<'a> {
        ReadyChecker::new(self.conn)
    }

    pub fn get_openid_configuration(self) -> OpenidConfigurationGetter<'a> {
        OpenidConfigurationGetter::new(self.conn)
    }
}

#[derive(Clone, derive_new::new)]
pub struct MetaGetter<'a> {
    conn: &'a Connection,
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
impl<'a> Command<MetaResponse> for MetaGetter<'a> {
    async fn r#do(&self) -> Result<MetaResponse> {
        let res: MetaResponse = self
            .conn
            .client
            .get("/meta".to_string())
            .await?
            .json()
            .await?;
        Ok(res)
    }
}

#[derive(derive_new::new)]
pub struct LiveChecker<'a> {
    conn: &'a Connection,
}

#[async_trait::async_trait]
impl<'a> Command<bool> for LiveChecker<'a> {
    async fn r#do(&self) -> Result<bool> {
        let well_known = self.conn.client.get("/.well-known/live").await?.status();

        let version = self.conn.db_version().get().await?;

        Ok(StatusCode::le(&well_known, &StatusCode::BAD_REQUEST) && !version.is_empty())
    }
}

#[derive(derive_new::new)]
pub struct ReadyChecker<'a> {
    conn: &'a Connection,
}

#[async_trait::async_trait]
impl<'a> Command<bool> for ReadyChecker<'a> {
    async fn r#do(&self) -> Result<bool> {
        let well_known = self.conn.client.get("/.well-known/ready").await?;

        let version = self.conn.db_version().get().await?;

        Ok(StatusCode::le(&well_known.status(), &StatusCode::BAD_REQUEST) && !version.is_empty())
    }
}

#[derive(derive_new::new)]
pub struct OpenidConfigurationGetter<'a> {
    conn: &'a Connection,
}

#[derive(Deserialize, Serialize)]
struct OpenIdConfiguration {}

#[async_trait::async_trait]
impl<'a> Command<OpenIdConfiguration> for OpenidConfigurationGetter<'a> {
    async fn r#do(&self) -> Result<OpenIdConfiguration> {
        Ok(self
            .conn
            .client
            .get("/.well-known/openid-configuration")
            .await?
            .json::<OpenIdConfiguration>()
            .await?)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::{
        command,
        connection::{http::HttpParams, Auth, AuthParams, Connection, ConnectionBuilder},
    };

    #[tokio::test]
    async fn meta_getter_works() {
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

        let connection = ConnectionBuilder::new("http", server.host_with_port()).build();

        let misc = Misc::new(&connection);
        let meta = misc
            .get_meta()
            .r#do()
            .await
            .expect("error fetching meta data");

        mock.assert();
        assert_eq!(response, meta);
    }

    #[tokio::test]
    async fn live_checker_works() {
        let mut server = mockito::Server::new();
        let response = MetaResponse {
            hostname: "http://[::]:8080".to_string(),
            modules: HashMap::new(),
            version: "1.19.0".to_string(),
        };

        server
            .mock("GET", "/v1/meta")
            .with_body(serde_json::to_string(&response).expect("error serializing mock response"))
            .create();

        let well_known_mock = server.mock("GET", "/v1/.well-known/live").create();

        let connection = ConnectionBuilder::new("http", server.host_with_port()).build();
        let misc = Misc::new(&connection);
        let is_live = misc
            .check_live()
            .r#do()
            .await
            .expect("error checking is live");

        well_known_mock.assert();
        assert!(is_live);
    }
}
