use crate::errors::ChatError;

use anyhow::{Context, Result};
use log::info;
use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn init() -> Result<Client> {
    let mongodb_uri =
        env::var("MONGODB_URI").context("MONGODB_URI environment variable not set")?;

    let client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .context("Failed to parse MongoDB connection string")?;

    let client = Client::with_options(client_options).map_err(ChatError::DatabaseError)?;

    info!("Database connection initialized");
    Ok(client)
}
