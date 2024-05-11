use indexed_db_futures::web_sys::DomException;
use std::fmt;

#[derive(Debug)]
pub enum CBwebDatabaseError {
    NotFound,
    Dom(DomException),
    DomError(String),
    SerializationError(serde_json::Error),
    DeserializationError(serde_wasm_bindgen::Error),
}

impl fmt::Display for CBwebDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CBwebDatabaseError::Dom(e) => write!(f, "DOM Exception: {}", e.message().as_str()),
            CBwebDatabaseError::DomError(msg) => write!(f, "DOM Error: {}", msg),
            CBwebDatabaseError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
            CBwebDatabaseError::DeserializationError(e) => write!(f, "Deserialization Error: {}", e),
            CBwebDatabaseError::NotFound => write!(f, "No record found"),
        }
    }
}

impl From<DomException> for CBwebDatabaseError {
    fn from(e: DomException) -> Self {
        CBwebDatabaseError::Dom(e)
    }
}

impl From<serde_json::Error> for CBwebDatabaseError {
    fn from(e: serde_json::Error) -> Self {
        CBwebDatabaseError::SerializationError(e)
    }
}

impl From<serde_wasm_bindgen::Error> for CBwebDatabaseError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        CBwebDatabaseError::DeserializationError(e)
    }
}