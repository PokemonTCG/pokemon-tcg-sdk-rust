use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub small: String,
    pub large: String,
}
