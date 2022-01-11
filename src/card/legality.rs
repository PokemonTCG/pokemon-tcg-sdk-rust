use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Legality {
    pub standard: Option<String>,
    pub expanded: Option<String>,
    pub unlimited: Option<String>,
}
