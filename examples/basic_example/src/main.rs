use weaviate_client::command::meta::MetaResponse;
use weaviate_client::connection::{Connection, ConnectionParams};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sdk = Connection::new(ConnectionParams {
        api_key: "".to_string(),
        host: "localhost:8080".to_string(),
        scheme: "http".to_string(),
        headers: None,
        auth_client_secret: None,
    });

    let meta = sdk
        .http
        .get("/meta".to_string(), None)
        .await?
        .json::<MetaResponse>()
        .await?;

    println!("{:?}", meta);

    Ok(())
}
