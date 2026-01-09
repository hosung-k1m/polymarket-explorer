use serde::{Deserialize, Serialize};

/**
* GAMMA API MODELS
*/
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

/*
* POLAR QUERY MODELS
*/

// trader performace stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trader {
    pub trader_address: String,
    pub total_markets_entered: u32,
    pub total_markets_resolved: u32,
    pub total_wins: u32,
    pub accuracy: f64,
    pub total_invested: f64,
    pub total_returned: f64,
    pub roi: f64,
}

// positions held by trader
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub trader_address: String,
    pub token_id: String,
    pub market_id: String,
    pub side: String,  // "YES" or "NO"
    pub shares_held: f64,
    pub avg_entry_price: f64,
    pub first_entry_block: Option<u64>,
}

// transaction/trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub block_number: u64,
    pub transaction_hash: String,
    pub trader_address: String,
    pub token_id: String,
    pub side: String,  // "YES" or "NO"
    pub action: String,  // "BUY" or "SELL"
    pub shares: f64,
    pub usdc_amount: f64,
    pub market_id: String,
}

// resolved market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketResolution {
    pub condition_id: String,
    pub outcome: String,
    pub resolution_block: u64,
    pub yes_token_id: String,
    pub no_token_id: String,
}
