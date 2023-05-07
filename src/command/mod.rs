use crate::Connection;

pub mod graphql;
pub mod misc;
pub mod schema;

#[async_trait::async_trait]
pub trait Command<T> {
    async fn r#do(&self) -> Result<T, anyhow::Error>;
    fn validate();
}
