# Error Handling System

This document describes the comprehensive error handling system implemented in the Polymarket Explorer.

## Overview

The error handling system is designed to align with the data pipeline architecture:

```
Data Source → Parsing → Normalization → Analysis → Output
```

Each stage has its own error types with detailed, contextual error messages that help developers and users understand what went wrong and how to fix it.

## Architecture

### Error Hierarchy

```rust
AppError (top-level)
  ├── HttpError (network/transport layer)
  ├── DataSourceError (API/data source layer)
  ├── ParseError (JSON parsing layer)
  ├── NormalizationError (data standardization layer)
  ├── AnalysisError (analytical engine layer)
  └── OutputError (display/formatting layer)
```

### Location

All error types are defined in `src/error.rs`.

## Error Types

### 1. HttpError

Errors that occur during HTTP requests and network communication.

**Variants:**
- `RequestFailed` - HTTP request failed with a status code
- `ConnectionFailed` - Network connection error
- `Timeout` - Request timeout
- `InvalidUrl` - Invalid URL format
- `ResponseReadError` - Response body could not be read

**Example:**
```rust
HttpError::RequestFailed {
    status: 404,
    url: "https://gamma-api.polymarket.com/events/slug/invalid".to_string(),
    body: "Not Found".to_string(),
}
```

### 2. DataSourceError

Errors when interacting with external data sources (APIs).

**Variants:**
- `MarketGroupNotFound` - Market group not found
- `MarketNotFound` - Market not found within a group
- `InvalidApiResponse` - API returned invalid or unexpected data structure
- `RateLimitExceeded` - API rate limit exceeded
- `AuthenticationFailed` - API authentication failed
- `ApiUnavailable` - API is unavailable or down

**Example:**
```rust
DataSourceError::MarketGroupNotFound {
    slug: "non-existent-market".to_string(),
}
```

### 3. ParseError

Errors when parsing JSON or other data formats.

**Variants:**
- `JsonDeserializationFailed` - JSON deserialization failed
- `MissingField` - Required field is missing
- `InvalidFieldFormat` - Field has invalid format
- `InvalidArrayLength` - Array has unexpected length
- `InvalidNumber` - Numeric parsing error

**Example:**
```rust
ParseError::JsonDeserializationFailed {
    field_name: Some("outcomes".to_string()),
    expected_type: "Vec<String>".to_string(),
    json_snippet: "[\"YES\", \"NO\"]".to_string(),
    reason: "expected array".to_string(),
}
```

### 4. NormalizationError

Errors when standardizing data from different sources.

**Variants:**
- `TokenIdExtractionFailed` - Token ID extraction failed
- `OutcomeMappingFailed` - Outcome mapping failed
- `InvalidPriceData` - Price data is invalid or inconsistent
- `InvalidVolumeData` - Volume data is invalid or inconsistent
- `ValidationFailed` - Data validation failed
- `EmptyRequiredField` - Required field is empty after normalization

**Example:**
```rust
NormalizationError::TokenIdExtractionFailed {
    market_slug: "some-market".to_string(),
    reason: "Expected at least 2 token IDs (YES, NO), found 1".to_string(),
}
```

### 5. AnalysisError

Errors that occur in the analytical engine.

**Variants:**
- `InsufficientData` - Insufficient data to perform analysis
- `CalculationFailed` - Calculation error
- `InvalidPosition` - Position data is invalid
- `StatisticalError` - Statistical analysis failed
- `StaleData` - Market data is stale or outdated

**Example:**
```rust
AnalysisError::InsufficientData {
    analysis_type: "position analysis".to_string(),
    reason: "No positions found for this market".to_string(),
}
```

### 6. OutputError

Errors when formatting or displaying output.

**Variants:**
- `FormattingFailed` - Failed to format data for display
- `WriteFailed` - Failed to write output

**Example:**
```rust
OutputError::FormattingFailed {
    data_type: "market data".to_string(),
    reason: "Invalid UTF-8 sequence".to_string(),
}
```

## Usage Examples

### Basic Error Handling

```rust
use crate::error::{Result, HttpError};

pub async fn fetch_data(url: &str) -> Result<String> {
    // Your code here
    Ok(data)
}
```

### Creating Errors

```rust
// HTTP Error
return Err(HttpError::ConnectionFailed {
    url: url.to_string(),
    reason: "Network unreachable".to_string(),
}.into());

// Parse Error
return Err(ParseError::JsonDeserializationFailed {
    field_name: Some("price".to_string()),
    expected_type: "f64".to_string(),
    json_snippet: json_snippet(json, 200),
    reason: e.to_string(),
}.into());

// Normalization Error
return Err(NormalizationError::InvalidPriceData {
    market_slug: market.slug.clone(),
    field_name: "outcome_prices".to_string(),
    reason: "Price cannot be negative".to_string(),
}.into());
```

### Error Propagation

The `?` operator automatically converts errors into `AppError`:

```rust
pub async fn process_market(slug: &str) -> Result<Market> {
    let raw_data = fetch_raw_data(slug).await?;  // HttpError → AppError
    let parsed = parse_data(raw_data)?;           // ParseError → AppError
    let normalized = normalize_data(parsed)?;     // NormalizationError → AppError
    Ok(normalized)
}
```

## Error Message Guidelines

### Good Error Messages

✅ **Specific and actionable:**
```
Failed to extract token IDs for market 'presidential-election-2024':
Expected at least 2 token IDs (YES, NO), found 1
```

✅ **Include context:**
```
Failed to deserialize JSON for field 'outcomes': Expected type 'Vec<String>'
Reason: EOF while parsing a list
JSON: ["YES", "NO"
```

### Poor Error Messages

❌ **Vague:**
```
Parse error
```

❌ **No context:**
```
Invalid data
```

## User-Facing Error Display

The main application provides user-friendly error messages with tips:

```rust
if let Err(e) = run().await {
    eprintln!("\n❌ Error: {}\n", e);

    match &e {
        AppError::Http(_) => {
            eprintln!("💡 Tip: Check your internet connection and verify the URL is correct.");
        }
        AppError::DataSource(_) => {
            eprintln!("💡 Tip: Verify the market slug exists on Polymarket.");
        }
        // ... more cases
    }
}
```

## Helper Functions

### `truncate_for_display(s: &str, max_len: usize) -> String`

Truncates a string for display in error messages.

```rust
let snippet = truncate_for_display(&long_json, 200);
```

### `json_error_snippet(json: &str, max_len: usize) -> String`

Extracts a relevant snippet from JSON for error messages.

```rust
let snippet = json_error_snippet(&response_body, 500);
```

## Testing Error Handling

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_id_extraction_error() {
        let result = extract_token_ids(vec!["single_token".to_string()]);

        match result {
            Err(AppError::Normalization(NormalizationError::TokenIdExtractionFailed { .. })) => {
                // Expected error
            }
            _ => panic!("Expected TokenIdExtractionFailed error"),
        }
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_invalid_market_slug() {
    let client = HttpClient::new();
    let source = PolymarketApiSource::new(client);

    let result = source.get_market_group("non-existent-slug").await;

    assert!(result.is_err());
}
```

## Best Practices

1. **Use specific error types** - Don't use generic errors when a specific error type exists
2. **Provide context** - Include relevant identifiers (slug, URL, field name)
3. **Include snippets** - For parse errors, include the problematic data
4. **Validate early** - Validate data as soon as it enters your system
5. **Fail fast** - Return errors immediately when validation fails
6. **Chain errors** - Use `?` to propagate errors up the call stack
7. **Handle at boundaries** - Handle errors at system boundaries (HTTP, CLI)
8. **Log appropriately** - Log errors with sufficient detail for debugging

## Future Enhancements

Potential improvements to the error handling system:

1. **Error codes** - Add unique error codes for easier tracking
2. **Structured logging** - Integrate with a logging framework
3. **Retry logic** - Add automatic retry for transient errors
4. **Error recovery** - Add recovery strategies for certain error types
5. **Telemetry** - Send error metrics to monitoring systems
6. **Localization** - Support multiple languages for error messages
7. **Error documentation** - Generate API documentation from error types

## Migration from anyhow

The codebase previously used `anyhow::Result` and `anyhow::Error`. The migration to custom error types provides:

- **Better type safety** - Errors are part of the function signature
- **Better IDE support** - Autocomplete and error detection
- **Better documentation** - Error types are self-documenting
- **Better error handling** - Can match on specific error types
- **Better user experience** - More helpful error messages

The migration was straightforward:

```rust
// Before
use anyhow::{Result, Context};

pub async fn fetch_data() -> Result<Data> {
    let response = client.get(url).await?;
    let data = serde_json::from_str(&response).context("Failed to parse")?;
    Ok(data)
}

// After
use crate::error::{Result, ParseError};

pub async fn fetch_data() -> Result<Data> {
    let response = client.get(url).await?;
    let data = serde_json::from_str(&response).map_err(|e| {
        ParseError::JsonDeserializationFailed {
            field_name: None,
            expected_type: "Data".to_string(),
            json_snippet: json_error_snippet(&response, 200),
            reason: e.to_string(),
        }
    })?;
    Ok(data)
}
```

## Contributing

When adding new error types:

1. Add the variant to the appropriate error enum in `src/error.rs`
2. Implement the `Display` trait case
3. Add documentation comments
4. Add usage examples to this document
5. Update tests

## Questions?

For questions about the error handling system, please open an issue or contact the maintainers.
