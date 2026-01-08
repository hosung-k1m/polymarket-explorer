use crate::standard_data::models::{Market, MarketGroup};
use crate::data_sources::polymarket_api::types::{GammaMarketGroupResponse, GammaMarketResponse};
use crate::error::{Result, ParseError, NormalizationError};

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
        let market_slug = raw.slug.clone();

        // Parse JSON string fields with detailed error messages
        let outcomes: Vec<String> = serde_json::from_str(&raw.outcomes)
            .map_err(|e| ParseError::JsonDeserializationFailed {
                field_name: Some("outcomes".to_string()),
                expected_type: "Vec<String>".to_string(),
                json_snippet: raw.outcomes.clone(),
                reason: e.to_string(),
            })?;

        let outcome_prices: Vec<String> = serde_json::from_str(&raw.outcome_prices)
            .map_err(|e| ParseError::JsonDeserializationFailed {
                field_name: Some("outcome_prices".to_string()),
                expected_type: "Vec<String>".to_string(),
                json_snippet: raw.outcome_prices.clone(),
                reason: e.to_string(),
            })?;

        let token_ids: Vec<String> = serde_json::from_str(&raw.clob_token_ids)
            .map_err(|e| ParseError::JsonDeserializationFailed {
                field_name: Some("clob_token_ids".to_string()),
                expected_type: "Vec<String>".to_string(),
                json_snippet: raw.clob_token_ids.clone(),
                reason: e.to_string(),
            })?;

        // Extract YES and NO token IDs with proper validation
        if token_ids.len() < 2 {
            return Err(NormalizationError::TokenIdExtractionFailed {
                market_slug: market_slug.clone(),
                reason: format!(
                    "Expected at least 2 token IDs (YES, NO), found {}",
                    token_ids.len()
                ),
            }.into());
        }

        let yes_token_id = token_ids[0].clone();
        let no_token_id = token_ids[1].clone();

        // Validate token IDs are not empty
        if yes_token_id.is_empty() || no_token_id.is_empty() {
            return Err(NormalizationError::TokenIdExtractionFailed {
                market_slug: market_slug.clone(),
                reason: "Token IDs cannot be empty".to_string(),
            }.into());
        }

        // Validate outcomes match expected format
        if outcomes.len() != 2 {
            return Err(NormalizationError::OutcomeMappingFailed {
                market_slug: market_slug.clone(),
                outcomes: outcomes.clone(),
                reason: format!(
                    "Expected 2 outcomes (YES, NO), found {}",
                    outcomes.len()
                ),
            }.into());
        }

        // Validate price data
        if outcome_prices.len() != 2 {
            return Err(NormalizationError::InvalidPriceData {
                market_slug: market_slug.clone(),
                field_name: "outcome_prices".to_string(),
                reason: format!(
                    "Expected 2 prices, found {}",
                    outcome_prices.len()
                ),
            }.into());
        }

        // Validate numeric fields are not negative
        if raw.volume_num < 0.0 || raw.liquidity_num < 0.0 {
            return Err(NormalizationError::InvalidVolumeData {
                market_slug: market_slug.clone(),
                field_name: "volume or liquidity".to_string(),
                reason: "Values cannot be negative".to_string(),
            }.into());
        }

        // Validate required string fields are not empty
        if raw.question.is_empty() {
            return Err(NormalizationError::EmptyRequiredField {
                field_name: "question".to_string(),
                entity_type: "Market".to_string(),
            }.into());
        }

        if raw.condition_id.is_empty() {
            return Err(NormalizationError::EmptyRequiredField {
                field_name: "condition_id".to_string(),
                entity_type: "Market".to_string(),
            }.into());
        }

        Ok(Market {
            question: raw.question,
            condition_id: raw.condition_id,
            slug: market_slug,
            outcomes,
            outcome_prices,
            yes_token_id,
            no_token_id,
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
