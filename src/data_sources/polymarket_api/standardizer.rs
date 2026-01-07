use crate::standard_data::models::Market;
use crate::data_sources::polymarket_api::types::GammaMarketResponse;
use anyhow::{Context, Result};

// struct just converts raw data to the standard data from X number of sources
pub struct PolymarketApiStandardizer;

impl PolymarketApiStandardizer {
    // convert GammaMarketResponse to Market
    pub fn standardize_market(raw: GammaMarketResponse) -> Result<Market> {
        let yes_token = raw.tokens[0].clone();
        let no_token = raw.tokens[1].clone();
        Ok(Market {
            slug: raw.slug,
            title: raw.title,
            //condition_id: raw.condition_id,
           // question: raw.question,
            yes_token_id: yes_token.clone(),
            no_token_id: no_token.clone(),
            active: raw.active,
            closed: raw.closed,
            volume: raw.volume,
            volume_1w: raw.volume_1w,
            volume_1m: raw.volume_1m,
            volume_1y: raw.volume_1y,
            created_at: raw.created_at,
            comment_count: raw.comment_count,
        })
    }
}
