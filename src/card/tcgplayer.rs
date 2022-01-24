use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Prices {
    /// The low price of the card
    pub low: Option<f32>,
    /// The mid price of the card
    pub mid: Option<f32>,
    /// The high price of the card
    pub high: Option<f32>,
    /// The market value of the card. This is usually the best representation of what people are willing to pay.
    pub market: Option<f32>,
    /// The direct low price of the card
    #[serde(alias = "directLow")]
    pub direct_low: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TcgPlayer {
    /// The URL to the TCGPlayer store page to purchase this card.
    pub url: String,
    /// A date that the price was last updated. In the format of YYYY/MM/DD
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    /// A hash of price types. All prices are in US Dollars. See below for possible values.
    pub prices: Option<Prices>,
}
