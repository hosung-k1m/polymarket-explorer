use crate::adapters::HttpClient;
use crate::data_sources::polymarket_api::types::GammaMarketGroupResponse;
use anyhow::Result;

const GAMMA_API_URL: &str = "https://gamma-api.polymarket.com";

pub struct PolymarketApiHandler {
    http_client: HttpClient,
}

impl PolymarketApiHandler {
    // constructor
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    // get market data from gamma api
    pub async fn fetch_market_group(&self, slug: &str) -> Result<GammaMarketGroupResponse> {
        let url = format!("{}/events/slug/{}", GAMMA_API_URL, slug);
        self.http_client.get(&url).await
    }
}
