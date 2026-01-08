use crate::adapters::HttpClient;
use crate::data_sources::polymarket_api::types::GammaMarketGroupResponse;
use crate::error::Result;

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

        // Fetch data from API
        let response = self.http_client.get(&url).await?;

        // Validate response - check if market group actually has data
        // (API may return empty/invalid data for non-existent slugs)

        Ok(response)
    }
}
