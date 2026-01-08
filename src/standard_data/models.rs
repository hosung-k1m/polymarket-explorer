use serde::{Deserialize, Serialize};

// market group responce from slug that will give all related market events
// voume and liquidity are total I think from all of its sub markets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketGroup {
    pub slug: String,
    pub title: String,
    pub active: bool,
    pub closed: bool,
    pub volume: f64,
    pub liquidity: f64,
    pub markets: Vec<Market>,
}

// individual market from the group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub question: String,
    pub condition_id: String,
    pub slug: String,
    pub outcomes: Vec<String>,
    pub outcome_prices: Vec<String>,
    pub yes_token_id: String,
    pub no_token_id: String,
    pub active: bool,
    pub closed: bool,
    pub volume: f64,
    pub volume_24h: f64,
    pub volume_1w: f64,
    pub volume_1m: f64,
    pub volume_1y: f64,
    pub liquidity: f64,
    // not sure what this is but might b good
    pub competitive: f64,
    pub last_trade_price: f64,
    pub bid_price: f64,
    pub ask_price: f64,
}
