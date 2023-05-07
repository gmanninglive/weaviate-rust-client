pub mod command;
pub mod connection;
mod graphql;
pub mod utils;

pub use command::{
    misc::{MetaGetter, Misc},
    Command,
};
pub use connection::*;

use graphql::GraphQL;

pub struct WeaviateClient {
    conn: Connection,
    // schema: Schema,
    // data: Data,
    // classifications: Classifications,
    // batch: Batch,
    // misc: Misc,
    // c11y: C11y,
    // backup: Backup,
    // cluster: Cluster,
}

#[derive(Default)]
pub struct WeaviateClientBuilder {
    host: String,
    scheme: String,
    auth: Auth,
    headers: Headers,
}

impl WeaviateClientBuilder {
    pub fn new(scheme: impl Into<String>, host: impl Into<String>) -> Self {
        Self {
            scheme: scheme.into(),
            host: host.into(),
            ..Default::default()
        }
    }

    pub fn auth(&mut self, auth: Auth) -> &mut Self {
        self.auth = auth;
        self
    }

    pub fn headers(&mut self, headers: Headers) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn build(self) -> WeaviateClient {
        WeaviateClient {
            conn: ConnectionBuilder::new(self.scheme, self.host)
                .auth(self.auth)
                .headers(self.headers)
                .build(),
        }
    }
}

impl WeaviateClient {
    pub fn graphql(&self) -> GraphQL {
        GraphQL::new(&self.conn)
    }

    pub fn misc(&self) -> Misc {
        Misc::new(&self.conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new();
        let response = command::misc::MetaResponse {
            hostname: "http://[::]:8080".to_string(),
            modules: HashMap::new(),
            version: "1.19.0".to_string(),
        };

        server
            .mock("GET", "/v1/meta")
            .with_body(serde_json::to_string(&response).expect("error serializing mock response"))
            .create();

        let client = WeaviateClientBuilder::new("http", server.host_with_port()).build();
        let misc = client.misc();
        let meta = misc
            .get_meta()
            .r#do()
            .await
            .expect("error fetching meta data");

        assert_eq!(meta, response);
    }

    #[tokio::test]
    async fn live_checker_works() {
        let mut server = mockito::Server::new();
        let response = command::misc::MetaResponse {
            hostname: "http://[::]:8080".to_string(),
            modules: HashMap::new(),
            version: "1.19.0".to_string(),
        };

        server
            .mock("GET", "/v1/meta")
            .with_body(serde_json::to_string(&response).expect("error serializing mock response"))
            .create();

        let well_known_mock = server.mock("GET", "/v1/.well-known/live").create();

        let client = WeaviateClientBuilder::new("http", server.host_with_port()).build();
        let misc = client.misc();
        let is_live = misc
            .check_live()
            .r#do()
            .await
            .expect("error checking is live");

        well_known_mock.assert();
        assert!(is_live);
    }
}
