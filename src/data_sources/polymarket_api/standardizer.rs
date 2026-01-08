use crate::standard_data::models::{Market, MarketGroup};
use crate::data_sources::polymarket_api::types::{GammaMarketGroupResponse, GammaMarketResponse};
use anyhow::{Context, Result};

// struct to standardize from X sourcse for analytic engine
pub struct PolymarketApiStandardizer;

impl PolymarketApiStandardizer {
    pub fn standardize_market_group(raw: GammaMarketGroupResponse) -> Result<MarketGroup> {
        let markets = raw.markets
            .into_iter()
            .map(Self::standardize_market)
            .collect::<Result<Vec<_>>>()?;

        Ok(MarketGroup {
            slug: raw.slug,
            title: raw.title,
            active: raw.active,
            closed: raw.closed,
            volume: raw.volume,
            liquidity: raw.liquidity,
            markets,
        })
    }

    // convert the gamma api data to standard data model
    fn standardize_market(raw: GammaMarketResponse) -> Result<Market> {
        // Parse JSON strings
        let outcomes: Vec<String> = serde_json::from_str(&raw.outcomes)
            .context("Failed to parse outcomes")?;
        
        let outcome_prices: Vec<String> = serde_json::from_str(&raw.outcome_prices)
            .context("Failed to parse outcome prices")?;
        
        let token_ids: Vec<String> = serde_json::from_str(&raw.clob_token_ids)
            .context("Failed to parse token IDs")?;

        // get token Id for YES, NO from vec
        let yes_token_id = token_ids.get(0)
            .context("Missing YES token ID")?
            .clone();
        
        let no_token_id = token_ids.get(1)
            .context("Missing NO token ID")?
            .clone();

        Ok(Market {
            question: raw.question,
            condition_id: raw.condition_id,
            slug: raw.slug,
            outcomes: outcomes,
            outcome_prices: outcome_prices,
            yes_token_id: yes_token_id,
            no_token_id: no_token_id,
            active: raw.active,
            closed: raw.closed,
            volume: raw.volume_num,
            volume_24h: raw.volume_24hr,
            volume_1w: raw.volume_1wk,
            volume_1m: raw.volume_1mo,
            volume_1y: raw.volume_1yr,
            liquidity: raw.liquidity_num,
            competitive: raw.competitive,
            last_trade_price: raw.last_trade_price,
            bid_price: raw.best_bid,
            ask_price: raw.best_ask,
        })
    }
}
