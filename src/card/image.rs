use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub small: String,
    pub large: String,
}
