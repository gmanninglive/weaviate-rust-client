use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Command, CommandTrait};

pub type MetaGetter = Command;

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
impl CommandTrait<MetaResponse> for MetaGetter {
    async fn r#do(&self) -> Result<MetaResponse, anyhow::Error> {
        let res: MetaResponse = self
            .client
            .get("/meta".to_string(), None)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    fn validate() {
        unimplemented!()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::connection::{Connection, ConnectionParams};

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

        let conn = Connection::new(ConnectionParams {
            api_key: "".to_string(),
            host: server.host_with_port(),
            scheme: "http".to_string(),
            headers: None,
            auth_client_secret: None,
        });

        let meta = MetaGetter::new(conn)
            .r#do()
            .await
            .expect("error fetching meta data");

        mock.assert();
        assert_eq!(response, meta);
    }
}
