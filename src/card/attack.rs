use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Attack {
    /// The cost of the attack represented by a list of energy types.
    pub cost: Vec<String>,
    /// The name of the attack
    pub name: String,
    /// The text or description associated with the attack
    pub text: String,
    /// The damage amount of the attack
    pub damage: String,
    /// The total cost of the attack. For example, if it costs 2 fire energy, the converted energy cost is simply 2.
    #[serde(alias = "convertedEnergyCost")]
    pub converted_energy_cost: Option<usize>,
}
