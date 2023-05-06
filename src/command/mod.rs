pub mod meta;

use crate::connection::{http::HttpClient, Connection};

pub struct Command {
    /// The client's connection
    pub client: HttpClient,

    /// An array of validation errors
    pub errors: Vec<String>,
}

#[async_trait::async_trait]
pub trait CommandTrait<T> {
    async fn r#do(&self) -> Result<T, anyhow::Error>;
    // /// Optional method to build the payload of an actual call
    // fn payload<T>() -> T;
    /// validate that all the required parameters were fed to the builder
    fn validate();
}

impl Command {
    pub fn new(conn: Connection) -> Self {
        Self {
            client: conn.client,
            errors: Vec::new(),
        }
    }
}
