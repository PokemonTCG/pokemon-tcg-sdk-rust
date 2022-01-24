use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Prices {
    /// The average sell price as shown in the chart at the website for non-foils
    #[serde(alias = "averageSellPrice")]
    pub average_sell_price: Option<f32>,
    /// The lowest price at the market for non-foils
    #[serde(alias = "lowPrice")]
    pub low_price: Option<f32>,
    /// The trend price as shown at the website (and in the chart) for non-foils
    #[serde(alias = "trendPrice")]
    pub trend_price: Option<f32>,
    /// The lowest sell price from German professional sellers
    #[serde(alias = "germanProLow")]
    pub german_pro_low: Option<f32>,
    /// A suggested sell price for professional users, determined by an internal algorithm; this algorithm is controlled by cardmarket, not this API
    #[serde(alias = "suggestedPrice")]
    pub suggested_price: Option<f32>,
    /// The average sell price as shown in the chart at the website for reverse holos
    #[serde(alias = "reverseHoloSell")]
    pub reverse_holo_sell: Option<f32>,
    /// The lowest price at the market as shown at the website (for condition EX+) for reverse holos
    #[serde(alias = "reverseHoloLow")]
    pub reverse_holo_low: Option<f32>,
    /// The trend price as shown at the website (and in the chart) for reverse holos
    #[serde(alias = "reverseHoloTrend")]
    pub reverse_holo_trend: Option<f32>,
    /// The lowest price at the market for non-foils with condition EX or better
    #[serde(alias = "lowPriceExPlus")]
    pub low_price_ex_plus: Option<f32>,
    /// The average sale price over the last day
    pub avg1: Option<f32>,
    /// The average sale price over the last 7 days
    pub avg7: Option<f32>,
    /// The average sale price over the last 30 days
    pub avg30: Option<f32>,
    /// The average sale price over the last day for reverse holos
    #[serde(alias = "reverseHoloAvg1")]
    pub reverse_holo_avg1: Option<f32>,
    /// The average sale price over the last 7 days for reverse holos
    #[serde(alias = "reverseHoloAvg7")]
    pub reverse_holo_avg7: Option<f32>,
    /// The average sale price over the last 30 days for reverse holos
    #[serde(alias = "reverseHoloAvg30")]
    pub reverse_holo_avg30: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardMarket {
    /// The URL to the cardmarket store page to purchase this card.
    pub url: String,
    /// A date that the price was last updated. In the format of YYYY/MM/DD
    pub updated_at: Option<String>,
    /// A hash of price types. All prices are in Euros.
    pub prices: Option<Prices>,
}
