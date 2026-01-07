mod handler;
mod standardizer;
mod types;

use crate::adapters::HttpClient;
use crate::standard_data::models::Market;
use crate::standard_data::providers::MarketMetadataProvider;
use anyhow::Result;
use async_trait::async_trait;

use handler::PolymarketApiHandler;
use standardizer::PolymarketApiStandardizer;

pub struct PolymarketApiSource {
    handler: PolymarketApiHandler,
}

impl PolymarketApiSource {
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            handler: PolymarketApiHandler::new(http_client),
        }
    }
}

#[async_trait]
impl MarketMetadataProvider for PolymarketApiSource {
    async fn get_market(&self, slug: &str) -> Result<Market> {
        // get raw data from handler
        let raw = self.handler.fetch_market(slug).await?;
        // standardize the data from source
        println!("{:#?}", raw);
        let market = PolymarketApiStandardizer::standardize_market(raw)?;

        Ok(market)
    }
}
