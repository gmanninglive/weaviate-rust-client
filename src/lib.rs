pub mod command;
pub mod connection;
pub mod prelude;
pub mod utils;

pub use prelude::*;

pub use command::{
    misc::{MetaGetter, Misc},
    Command,
};
pub use connection::*;

use command::graphql::GraphQL;

pub struct WeaviateClient {
    conn: Connection,
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

    // TODO
    // schema: Schema,
    // data: Data,
    // classifications: Classifications,
    // batch: Batch,
    // c11y: C11y,
    // backup: Backup,
    // cluster: Cluster,
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

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
}
