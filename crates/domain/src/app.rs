use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation: {0}")]
    Validation(String),
    #[error("auth: {0}")]
    Auth(String),
    #[error("database: {0}")]
    Database(String),
    #[error("agent: {0}")]
    Agent(String),
    #[error("internal: {0}")]
    Internal(String),
}


pub type AppResult<T> = Result<T, AppError>;