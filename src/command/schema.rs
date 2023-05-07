use super::Command;
use crate::prelude::*;
use crate::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Not implemented
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct WeaviateClass {
    /** @description Name of the class as URI relative to the schema URL. */
    class: String,
    /** @description Name of the vector index to use, eg. (HNSW) */
    vector_index_type: Option<String>,
    /** @description Vector-index config, that is specific to the type of index selected in vectorIndexType */
    vector_index_config: Option<HashMap<String, String>>,
    /** @description Manage how the index should be sharded and distributed in the cluster */
    sharding_config: Option<HashMap<String, String>>,
    // replicationConfig?: definitions['ReplicationConfig'];
    // invertedIndexConfig?: definitions['InvertedIndexConfig'];
    // @description Specify how the vectors for this class should be determined. The options are either 'none' - this means you have to import a vector with each object yourself - or the name of a module that provides vectorization capabilities, such as 'text2vec-contextionary'. If left empty, it will use the globally configured default which can itself either be 'none' or a specific module.
    vectorizer: String,
    /** @description Configuration specific to modules this Weaviate instance has installed */
    module_config: Option<HashMap<String, String>>,
    /** @description Description of the class. */
    description: Option<String>,
    // The properties of the class.
    // properties?: definitions['Property'][];
}

impl Default for WeaviateClass {
    fn default() -> Self {
        Self {
            class: "".to_string(),
            vector_index_type: None,
            vector_index_config: None,
            sharding_config: None,
            // we override default to match the default set by DB
            vectorizer: "none".to_string(),
            module_config: None,
            description: None,
        }
    }
}

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

/// ## Schema Commands
#[derive(derive_new::new)]
pub struct Schema<'a> {
    conn: &'a Connection,
}

impl<'a> Schema<'a> {
    pub fn get_schema(self) -> SchemaGetter<'a> {
        SchemaGetter::new(self.conn)
    }

    pub fn create_class(self, class: WeaviateClass) -> ClassCreator<'a> {
        ClassCreator::new(self.conn, class)
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
}

#[derive(derive_new::new)]
pub struct ClassCreator<'a> {
    conn: &'a Connection,
    class: WeaviateClass,
}

#[async_trait::async_trait]
impl<'a> Command<WeaviateClass> for ClassCreator<'a> {
    async fn r#do(&self) -> Result<WeaviateClass> {
        Ok(self
            .conn
            .client
            .post("/schema", &self.class)
            .await?
            .json::<WeaviateClass>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConnectionBuilder, WeaviateClientBuilder};

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

    #[tokio::test]
    async fn test_class_creator() {
        let mock_class = WeaviateClass {
            class: "TestClass".to_owned(),
            ..Default::default()
        };

        let mut server = mockito::Server::new();

        let mock = server
            .mock("POST", "/v1/schema")
            .with_body(serde_json::to_string(&mock_class).expect("error serializing mock response"))
            .create();

        let client = WeaviateClientBuilder::new("http", server.host_with_port()).build();

        let class = client
            .schema()
            .create_class(mock_class.clone())
            .r#do()
            .await
            .expect("");

        mock.assert();
        assert_eq!(class, mock_class)
    }
}
