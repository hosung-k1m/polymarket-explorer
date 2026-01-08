use serde::{Deserialize, Serialize};

// raw from Gamma API
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaMarketGroupResponse {
    pub slug: String,
    pub title: String,
    pub active: bool,
    pub closed: bool,
    pub volume: f64,
    pub liquidity: f64,
    pub markets: Vec<GammaMarketResponse>,
}

// individual market events
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaMarketResponse {
    pub question: String,
    pub condition_id: String,
    pub slug: String,
    pub outcomes: String,
    pub outcome_prices: String,
    pub clob_token_ids: String,
    pub active: bool,
    pub closed: bool,
    pub volume_num: f64,
    pub volume_24hr: f64,
    pub volume_1wk: f64,
    pub volume_1mo: f64,
    pub volume_1yr: f64,
    pub liquidity_num: f64,
    pub competitive: f64,
    pub last_trade_price: f64,
    pub best_bid: f64,
    pub best_ask: f64,
}
