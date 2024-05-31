use indexed_db_futures::web_sys::DomException;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CBwebDatabaseError {
    #[error("No record found")]
    NotFound,
    #[error("DOM Exception: {0}")]
    Dom(String),
    #[error("DOM Error: {0}")]
    DomError(String),
    #[error("Serialization Error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Deserialization Error: {0}")]
    DeserializationError(#[from] serde_wasm_bindgen::Error),
    #[error("Invalid operation Error: {0}")]
    InvalidOperation(String),
}

impl From<DomException> for CBwebDatabaseError {
    fn from(e: DomException) -> Self {
        CBwebDatabaseError::Dom(e.message().into())
    }
}
