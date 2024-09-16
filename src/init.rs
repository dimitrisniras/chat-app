use crate::db;
use crate::logging;

use std::env;
use anyhow::Context;
use dotenv::dotenv;
use log::{info, error};
use mongodb::Client;
use anyhow::Result;

pub async fn init() -> Result<(Client, String, u16)> {
    if let Err(err) = dotenv() {
        error!("Failed to load environment variables: {}", err);
        return Err(err.into()); 
    }

    logging::init();
    info!("Logging initialized");

    let db_client = db::init().await?;

    let url = env::var("SERVER_URL")
        .context("SERVER_URL environment variable not set")?;
    
    let port = env::var("SERVER_PORT")
        .context("SERVER_PORT environment variable not set")?
        .parse::<u16>()
        .context("Invalid SERVER_PORT value")?;

    Ok((db_client, url, port))
}