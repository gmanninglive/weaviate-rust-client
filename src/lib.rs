pub mod command;
pub mod connection;
mod graphql;
pub mod utils;

pub use connection::*;

use graphql::*;

#[derive(Default)]
struct ClientParams {
    host: String,
    scheme: String,
    auth: Auth,
    headers: Headers,
}

struct WeaviateClient {
    conn: Connection,
    graphql: GraphQL,
    // schema: Schema,
    // data: Data,
    // classifications: Classifications,
    // batch: Batch,
    // misc: Misc,
    // c11y: C11y,
    // backup: Backup,
    // cluster: Cluster,
}

impl WeaviateClient {
    pub fn new(params: ClientParams) -> Self {
        let connection = ConnectionBuilder::new(params.scheme, params.host)
            .auth(params.auth)
            .headers(params.headers)
            .build();

        Self {
            conn: connection.clone(),
            graphql: GraphQL::new(connection.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let under_construction = true;
        assert!(under_construction);
    }
}
