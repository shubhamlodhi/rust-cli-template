use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    {% if include-config %}
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    {% endif %}
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}