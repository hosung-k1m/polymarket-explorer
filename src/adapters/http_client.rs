use serde::de::DeserializeOwned;
use crate::error::{Result, HttpError, ParseError, json_error_snippet};

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

        // Send request with proper error handling
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    HttpError::Timeout {
                        url: url.to_string(),
                        duration_secs: 30,
                    }
                } else if e.is_connect() {
                    HttpError::ConnectionFailed {
                        url: url.to_string(),
                        reason: e.to_string(),
                    }
                } else {
                    HttpError::RequestFailed {
                        status: e.status().map(|s| s.as_u16()).unwrap_or(0),
                        url: url.to_string(),
                        body: e.to_string(),
                    }
                }
            })?;

        // Check HTTP status
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<unable to read response body>".to_string());

            return Err(HttpError::RequestFailed {
                status: status.as_u16(),
                url: url.to_string(),
                body,
            }.into());
        }

        // Read response body
        let text = response.text().await.map_err(|e| {
            HttpError::ResponseReadError {
                url: url.to_string(),
                reason: e.to_string(),
            }
        })?;

        // Deserialize JSON
        let data = serde_json::from_str::<T>(&text).map_err(|e| {
            ParseError::JsonDeserializationFailed {
                field_name: None,
                expected_type: std::any::type_name::<T>().to_string(),
                json_snippet: json_error_snippet(&text, 500),
                reason: e.to_string(),
            }
        })?;

        Ok(data)
    }
    
}
