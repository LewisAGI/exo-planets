use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExoError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("csv: {0}")]
    Csv(#[from] csv::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("http: {0}")]
    Http(String),
    #[error("parse: {0}")]
    Parse(String),
    #[error("model: {0}")]
    Model(String),
    #[error("missing required field {field} on {id}")]
    MissingField { id: String, field: &'static str },
}

pub type Result<T> = std::result::Result<T, ExoError>;
