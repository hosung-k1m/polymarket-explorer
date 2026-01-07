use anyhow::Result;
use serde::de::DeserializeOwned;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    // GET reuqest to url
    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        println!("sent GET request to URL: {}", url);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error = response.text().await?;
            anyhow::bail!(
            "HTTP Request failed: {} - {}",
                status,
                error
            );
        }
        
        let text = response.text().await?;

        let data = serde_json::from_str::<T>(&text).map_err(|e| {
            anyhow::anyhow!(
                "Deserialization Error: {}\nExpected Type: {}\nRaw JSON: {}",
                e,
                std::any::type_name::<T>(),
                text
            )
        })?;

        Ok(data)
    }
    
}
