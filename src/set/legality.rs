use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
pub struct Legality {
    pub standard: Option<String>,
    pub expanded: Option<String>,
    pub unlimited: Option<String>,
}
