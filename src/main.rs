use log::error;
use tokio::net::TcpListener;

mod db;
mod errors;
mod init;
mod logging;
mod models;
mod websocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the application
    let (db_client, url, port) = match init::init().await {
        Ok(result) => result,
        Err(err) => {
            error!("Initialization failed: {}", err);
            return Err(err);
        }
    };

    let addr = format!("{}:{}", url, port);
    let listener = TcpListener::bind(&addr).await?;

    // Accept incoming connections and handle them concurrently
    while let Ok((stream, _)) = listener.accept().await {
        let db_client_clone = db_client.clone();
        tokio::spawn(async move {
            if let Err(e) = websocket::handle_connection(stream, db_client_clone).await {
                error!("Error handling WebSocket connection: {}", e);
            }
        });
    }

    Ok(())
}
