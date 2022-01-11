use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Resistance {
    /// The type of resistance, such as Fire or Water.
    #[serde(alias = "type")]
    pub type_name: String,
    /// The value of the resistance
    pub value: String,
}
