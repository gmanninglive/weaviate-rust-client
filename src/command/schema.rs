use serde::{Deserialize, Serialize};

use super::Command;
use crate::prelude::*;
use crate::Connection;

/// Not implemented
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct WeaviateClass {}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
struct WeaviateSchema {
    /// Semantic classes that are available.
    classes: Vec<WeaviateClass>,
    /// Format: email
    /// Email of the maintainer.
    maintainer: Option<String>,
    /// Name of the schema.
    name: Option<String>,
}

/// ## Misc Commands
#[derive(derive_new::new)]
pub struct Schema<'a> {
    conn: &'a Connection,
}

impl<'a> Schema<'a> {
    pub fn get_schema(self) -> SchemaGetter<'a> {
        SchemaGetter::new(self.conn)
    }
}

#[derive(derive_new::new)]
pub struct SchemaGetter<'a> {
    conn: &'a Connection,
}

#[async_trait::async_trait]
impl<'a> Command<WeaviateSchema> for SchemaGetter<'a> {
    async fn r#do(&self) -> Result<WeaviateSchema> {
        let res = self
            .conn
            .client
            .get("/schema")
            .await?
            .json::<WeaviateSchema>()
            .await?;

        Ok(res)
    }
    fn validate() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConnectionBuilder;

    #[tokio::test]
    async fn test_schema_getter() {
        let mut server = mockito::Server::new();

        let mock_schema = WeaviateSchema::default();

        let mock = server
            .mock("GET", "/v1/schema")
            .with_body(
                serde_json::to_string(&mock_schema).expect("error serializing mock response"),
            )
            .create();

        let conn = ConnectionBuilder::new("http", server.host_with_port()).build();

        let schema = SchemaGetter::new(&conn)
            .r#do()
            .await
            .expect("error fetching schema");

        mock.assert();
        assert_eq!(schema, mock_schema);
    }
}
