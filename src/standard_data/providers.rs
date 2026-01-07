use crate::standard_data::models::Market;
use anyhow::Result;
use async_trait::async_trait;

// interface for market data getter
#[async_trait]
pub trait MarketMetadataProvider: Send + Sync {
    async fn get_market(&self, slug: &str) -> Result<Market>;
}


