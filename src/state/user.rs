use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}
