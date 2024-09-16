use dotenv::dotenv;
use log::{info, error};
use crate::db;
use crate::logging;
use anyhow::Result;

pub async fn init() -> Result<()> {
    if let Err(err) = dotenv() {
        error!("Failed to load environment variables: {}", err);
        return Err(err.into()); 
    }

    logging::init();
    info!("Logging initialized");

    db::init().await?;

    Ok(())
}