use dotenv::dotenv;
use std::env;
use mongodb::Client;

pub async fn initialize_app() -> Result<(Client, u16), Box<dyn std::error::Error>> {
    // Load environment variables from .env
    dotenv().ok(); 

    // Get the MongoDB URI and port from the environment
    let mongodb_uri: String = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Initialize the database connection
    let db_client: Client = crate::db::init_db(&mongodb_uri).await?;

    Ok((db_client, port))
}