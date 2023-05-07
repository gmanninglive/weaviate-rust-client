use weaviate_client::command::meta::MetaResponse;
use weaviate_client::ConnectionBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sdk = ConnectionBuilder::new("http", "localhost:8080").build();

    let meta = sdk
        .client
        .get("/meta")
        .await?
        .json::<MetaResponse>()
        .await?;

    println!("{:?}", meta);

    Ok(())
}
