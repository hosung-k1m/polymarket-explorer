use serde::{Deserialize, Serialize};

// raw responce from GAMMA API
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaMarketResponse {
    pub slug: String,
    pub title: String,
    //pub condition_id: String,
    //pub question: String,
    pub tokens: Vec<String>,
    pub active: bool,
    pub closed: bool,
    pub volume: f64,
    pub volume_1w: f64,
    pub volume_1m: f64,
    pub volume_1y: f64,
    pub created_at: String,
    pub comment_count: u64,
}
