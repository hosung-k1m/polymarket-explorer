mod handler;
mod standardizer;

use crate::adapters::ParquetReader;
use crate::standard_data::models::{Trader, Position, Transaction};
use crate::standard_data::providers::{TraderStatsProvider, PositionProvider, TransactionProvider};
use anyhow::Result;
use async_trait::async_trait;

use handler::LocalDbHandler;
use standardizer::LocalDbStandardizer;

pub struct LocalDbSource {
    handler: LocalDbHandler,
}

impl LocalDbSource {
    pub fn new(data_dir: &str) -> Self {
        let reader = ParquetReader::new(data_dir);

        Self {
            handler: LocalDbHandler::new(reader),
        }
    }
}

#[async_trait]
impl TraderStatsProvider for LocalDbSource {
    async fn get_traders(&self, min_resolved_markets: u32) -> Result<Vec<Trader>> {
        let df = self.handler.fetch_traders(min_resolved_markets)?;
        LocalDbStandardizer::standardize_traders(df)
    }

    async fn get_traders_by_addresses(&self, addresses: &[String]) -> Result<Vec<Trader>> {
        let df = self.handler.fetch_traders_by_addresses(addresses)?;
        LocalDbStandardizer::standardize_traders(df)
    }
}

#[async_trait]
impl PositionProvider for LocalDbSource {
    async fn get_positions(&self, condition_id: &str) -> Result<Vec<Position>> {
        let df = self.handler.fetch_positions(condition_id)?;
        LocalDbStandardizer::standardize_positions(df)
    }
}

#[async_trait]
impl TransactionProvider for LocalDbSource {
    async fn get_recent_transactions( &self, condition_id: &str, days_back: u32) -> Result<Vec<Transaction>> {
        let df = self.handler.fetch_recent_transactions(condition_id, days_back)?;
        LocalDbStandardizer::standardize_transactions(df)
    }
}
