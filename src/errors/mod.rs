use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("MongoDB error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Invalid message format")]
    InvalidMessageFormat,

    #[error("Environment variable not set: {0}")]
    EnvVarError(#[from] std::env::VarError),
}
