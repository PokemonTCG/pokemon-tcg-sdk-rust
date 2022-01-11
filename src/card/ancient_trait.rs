use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AncientTrait {
    /// The name of the ancient trait
    pub name: String,
    /// The text value of the ancient trait
    pub text: String,
}
