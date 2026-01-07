use crate::adapters::HttpClient;
use crate::data_sources::polymarket_api::types::GammaMarketResponse;
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
    pub async fn fetch_market(&self, slug: &str) -> Result<GammaMarketResponse> {
        let url = format!("{}/events/slug/{}", GAMMA_API_URL, slug);
        self.http_client.get(&url).await
    }
}
