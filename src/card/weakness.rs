use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Weakness {
    /// The type of weakness, such as Fire or Water.
    #[serde(alias = "type")]
    pub type_name: String,
    /// The value of the weakness
    pub value: String,
}
