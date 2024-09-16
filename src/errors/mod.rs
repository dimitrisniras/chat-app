use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("MongoDB error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Environment variable not set: {0}")]
    EnvVarError(#[from] std::env::VarError),
}