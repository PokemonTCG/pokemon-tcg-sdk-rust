use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ability {
    /// The name of the ability
    pub name: String,
    /// The text value of the ability
    pub text: String,
    /// The type of the ability, such as Ability or Pokémon-Power
    #[serde(alias = "type")]
    pub type_name: String,
}
