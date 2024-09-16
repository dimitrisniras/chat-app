mod init; 
mod db;
mod errors;
mod logging;
mod models;

use log::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the application
    if let Err(err) = init::init().await {
        error!("Initialization failed: {}", err);
        return Err(err); 
    }

    Ok(())
}
