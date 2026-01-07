use serde::{Deserialize, Serialize};

// market data from gamma API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub slug: String,
    // slug with no '-'
    pub title: String,
    //pub condition_id: String,
    //pub question: String,
    // from clobTokenIds
    pub yes_token_id: String,
    pub no_token_id: String,
    // lowk not sure why polymarket marks things as TRUE for active and closed
    pub active: bool,
    pub closed: bool,
    pub volume: f64,
    pub volume_1w: f64,
    pub volume_1m: f64,
    pub volume_1y: f64,
    pub created_at: String,
    pub comment_count: u64,
    // TODO: get category
 //   pub tag: Option<String>,
}


