pub mod graphql;
pub mod misc;

#[async_trait::async_trait]
pub trait Command<T> {
    async fn r#do(&self) -> Result<T, anyhow::Error>;
    fn validate();
}
