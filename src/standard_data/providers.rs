use crate::standard_data::models::MarketGroup;
use crate::error::Result;
use async_trait::async_trait;

// interface for market data getter
#[async_trait]
pub trait MarketMetadataProvider: Send + Sync {
    async fn get_market_group(&self, slug: &str) -> Result<MarketGroup>;
}


