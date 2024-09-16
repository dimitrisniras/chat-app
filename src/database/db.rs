use mongodb::{Client, options::ClientOptions};

pub async fn init_db(uri: &str) -> Result<Client, mongodb::error::Error> {
    let client_options: ClientOptions = ClientOptions::parse(uri).await?;
    let client: Client = Client::with_options(client_options)?;

    Ok(client)
}