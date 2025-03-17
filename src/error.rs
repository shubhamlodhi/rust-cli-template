use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    {% if use_config %}
    #[error("Configuration error: {0}")]
    Config(String),
    {% endif %}

    #[error("Processing error: {0}")]
    Processing(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, Error>;