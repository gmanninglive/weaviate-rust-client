pub mod meta;

#[async_trait::async_trait]
pub trait Command<T> {
    async fn r#do(&self) -> Result<T, anyhow::Error>;
    /// validate that all the required parameters were fed to the builder
    fn validate();
}
