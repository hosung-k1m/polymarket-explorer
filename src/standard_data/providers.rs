use crate::standard_data::models::{MarketGroup, Trader, Position, Transaction};
use anyhow::Result;
use async_trait::async_trait;

// interface for market data getter
#[async_trait]
pub trait MarketMetadataProvider: Send + Sync {
    async fn get_market_group(&self, slug: &str) -> Result<MarketGroup>;
}

// interface for trader stats
#[async_trait]
pub trait TraderStatsProvider: Send + Sync {
    // get all traders with > min resolved markets
    async fn get_traders(&self, min_resolved_markets: u32) -> Result<Vec<Trader>>;

    // Get position data for traders by address
    async fn get_traders_by_addresses(&self, addresses: &[String]) -> Result<Vec<Trader>>;
}

// interface for position data
#[async_trait]
pub trait PositionProvider: Send + Sync {
    // get all positions from a condition ID
    async fn get_positions(&self, condition_id: &str) -> Result<Vec<Position>>;
}

// interface for transactions or trades in time window
#[async_trait]
pub trait TransactionProvider: Send + Sync {
    // get all transactionf with condition id for a given window
    async fn get_recent_transactions(
        &self,
        condition_id: &str,
        days_back: u32,
    ) -> Result<Vec<Transaction>>;
}
