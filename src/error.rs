//! Centralized error handling for the Polymarket Explorer
//!
//! This module defines a comprehensive error system that covers all layers
//! of the data pipeline: Data Source → Parsing → Normalization → Analysis → Output
//!
//! ## Error Hierarchy
//!
//! ```text
//! AppError (top-level)
//!   ├── HttpError (network/transport layer)
//!   ├── DataSourceError (API/data source layer)
//!   ├── ParseError (JSON parsing layer)
//!   ├── NormalizationError (data standardization layer)
//!   ├── AnalysisError (analytical engine layer)
//!   └── OutputError (display/formatting layer)
//! ```

use std::fmt;

/// Top-level application error type that encompasses all error categories
#[derive(Debug)]
pub enum AppError {
    /// HTTP/Network layer errors
    Http(HttpError),
    /// Data source/API layer errors
    DataSource(DataSourceError),
    /// JSON parsing layer errors
    Parse(ParseError),
    /// Data normalization layer errors
    Normalization(NormalizationError),
    /// Analytical engine layer errors
    Analysis(AnalysisError),
    /// Output/display layer errors
    Output(OutputError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Http(e) => write!(f, "HTTP Error: {}", e),
            AppError::DataSource(e) => write!(f, "Data Source Error: {}", e),
            AppError::Parse(e) => write!(f, "Parse Error: {}", e),
            AppError::Normalization(e) => write!(f, "Normalization Error: {}", e),
            AppError::Analysis(e) => write!(f, "Analysis Error: {}", e),
            AppError::Output(e) => write!(f, "Output Error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

// ============================================================================
// HTTP/Network Layer Errors
// ============================================================================

/// Errors that occur during HTTP requests and network communication
#[derive(Debug)]
pub enum HttpError {
    /// HTTP request failed with a status code
    RequestFailed {
        status: u16,
        url: String,
        body: String,
    },
    /// Network connection error
    ConnectionFailed {
        url: String,
        reason: String,
    },
    /// Request timeout
    Timeout {
        url: String,
        duration_secs: u64,
    },
    /// Invalid URL format
    InvalidUrl {
        url: String,
        reason: String,
    },
    /// Response body could not be read
    ResponseReadError {
        url: String,
        reason: String,
    },
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::RequestFailed { status, url, body } => {
                write!(
                    f,
                    "HTTP request failed with status {}: {}\nResponse: {}",
                    status, url, body
                )
            }
            HttpError::ConnectionFailed { url, reason } => {
                write!(f, "Failed to connect to {}: {}", url, reason)
            }
            HttpError::Timeout { url, duration_secs } => {
                write!(f, "Request to {} timed out after {} seconds", url, duration_secs)
            }
            HttpError::InvalidUrl { url, reason } => {
                write!(f, "Invalid URL '{}': {}", url, reason)
            }
            HttpError::ResponseReadError { url, reason } => {
                write!(f, "Failed to read response from {}: {}", url, reason)
            }
        }
    }
}

impl std::error::Error for HttpError {}

// ============================================================================
// Data Source Layer Errors
// ============================================================================

/// Errors that occur when interacting with external data sources (APIs)
#[derive(Debug)]
pub enum DataSourceError {
    /// Market group not found
    MarketGroupNotFound {
        slug: String,
    },
    /// Market not found within a group
    MarketNotFound {
        identifier: String,
        group_slug: String,
    },
    /// API returned invalid or unexpected data structure
    InvalidApiResponse {
        endpoint: String,
        reason: String,
    },
    /// API rate limit exceeded
    RateLimitExceeded {
        retry_after_secs: Option<u64>,
    },
    /// API authentication failed
    AuthenticationFailed {
        reason: String,
    },
    /// API is unavailable or down
    ApiUnavailable {
        service_name: String,
        reason: String,
    },
}

impl fmt::Display for DataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSourceError::MarketGroupNotFound { slug } => {
                write!(f, "Market group '{}' not found", slug)
            }
            DataSourceError::MarketNotFound { identifier, group_slug } => {
                write!(
                    f,
                    "Market '{}' not found in group '{}'",
                    identifier, group_slug
                )
            }
            DataSourceError::InvalidApiResponse { endpoint, reason } => {
                write!(
                    f,
                    "API endpoint '{}' returned invalid response: {}",
                    endpoint, reason
                )
            }
            DataSourceError::RateLimitExceeded { retry_after_secs } => {
                if let Some(secs) = retry_after_secs {
                    write!(f, "API rate limit exceeded. Retry after {} seconds", secs)
                } else {
                    write!(f, "API rate limit exceeded")
                }
            }
            DataSourceError::AuthenticationFailed { reason } => {
                write!(f, "API authentication failed: {}", reason)
            }
            DataSourceError::ApiUnavailable { service_name, reason } => {
                write!(f, "{} API is unavailable: {}", service_name, reason)
            }
        }
    }
}

impl std::error::Error for DataSourceError {}

// ============================================================================
// Parse Layer Errors
// ============================================================================

/// Errors that occur when parsing JSON or other data formats
#[derive(Debug)]
pub enum ParseError {
    /// JSON deserialization failed
    JsonDeserializationFailed {
        field_name: Option<String>,
        expected_type: String,
        json_snippet: String,
        reason: String,
    },
    /// Required field is missing
    MissingField {
        field_name: String,
        parent_type: String,
    },
    /// Field has invalid format
    InvalidFieldFormat {
        field_name: String,
        expected_format: String,
        actual_value: String,
    },
    /// Array has unexpected length
    InvalidArrayLength {
        field_name: String,
        expected: usize,
        actual: usize,
    },
    /// Numeric parsing error
    InvalidNumber {
        field_name: String,
        value: String,
        reason: String,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::JsonDeserializationFailed {
                field_name,
                expected_type,
                json_snippet,
                reason,
            } => {
                write!(
                    f,
                    "Failed to deserialize JSON{}: Expected type '{}'\nReason: {}\nJSON: {}",
                    field_name.as_ref().map(|n| format!(" for field '{}'", n)).unwrap_or_default(),
                    expected_type,
                    reason,
                    json_snippet
                )
            }
            ParseError::MissingField { field_name, parent_type } => {
                write!(
                    f,
                    "Required field '{}' is missing from {}",
                    field_name, parent_type
                )
            }
            ParseError::InvalidFieldFormat {
                field_name,
                expected_format,
                actual_value,
            } => {
                write!(
                    f,
                    "Field '{}' has invalid format. Expected: {}, Got: {}",
                    field_name, expected_format, actual_value
                )
            }
            ParseError::InvalidArrayLength {
                field_name,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Array '{}' has invalid length. Expected: {}, Got: {}",
                    field_name, expected, actual
                )
            }
            ParseError::InvalidNumber {
                field_name,
                value,
                reason,
            } => {
                write!(
                    f,
                    "Field '{}' has invalid number '{}': {}",
                    field_name, value, reason
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

// ============================================================================
// Normalization Layer Errors
// ============================================================================

/// Errors that occur when standardizing data from different sources
#[derive(Debug)]
pub enum NormalizationError {
    /// Token ID extraction failed
    TokenIdExtractionFailed {
        market_slug: String,
        reason: String,
    },
    /// Outcome mapping failed
    OutcomeMappingFailed {
        market_slug: String,
        outcomes: Vec<String>,
        reason: String,
    },
    /// Price data is invalid or inconsistent
    InvalidPriceData {
        market_slug: String,
        field_name: String,
        reason: String,
    },
    /// Volume data is invalid or inconsistent
    InvalidVolumeData {
        market_slug: String,
        field_name: String,
        reason: String,
    },
    /// Data validation failed
    ValidationFailed {
        entity_type: String,
        entity_id: String,
        reason: String,
    },
    /// Required field is empty after normalization
    EmptyRequiredField {
        field_name: String,
        entity_type: String,
    },
}

impl fmt::Display for NormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NormalizationError::TokenIdExtractionFailed { market_slug, reason } => {
                write!(
                    f,
                    "Failed to extract token IDs for market '{}': {}",
                    market_slug, reason
                )
            }
            NormalizationError::OutcomeMappingFailed {
                market_slug,
                outcomes,
                reason,
            } => {
                write!(
                    f,
                    "Failed to map outcomes for market '{}' (outcomes: {:?}): {}",
                    market_slug, outcomes, reason
                )
            }
            NormalizationError::InvalidPriceData {
                market_slug,
                field_name,
                reason,
            } => {
                write!(
                    f,
                    "Invalid price data in market '{}' for field '{}': {}",
                    market_slug, field_name, reason
                )
            }
            NormalizationError::InvalidVolumeData {
                market_slug,
                field_name,
                reason,
            } => {
                write!(
                    f,
                    "Invalid volume data in market '{}' for field '{}': {}",
                    market_slug, field_name, reason
                )
            }
            NormalizationError::ValidationFailed {
                entity_type,
                entity_id,
                reason,
            } => {
                write!(
                    f,
                    "Validation failed for {} '{}': {}",
                    entity_type, entity_id, reason
                )
            }
            NormalizationError::EmptyRequiredField {
                field_name,
                entity_type,
            } => {
                write!(
                    f,
                    "Required field '{}' is empty in {}",
                    field_name, entity_type
                )
            }
        }
    }
}

impl std::error::Error for NormalizationError {}

// ============================================================================
// Analysis Layer Errors
// ============================================================================

/// Errors that occur in the analytical engine
#[derive(Debug)]
pub enum AnalysisError {
    /// Insufficient data to perform analysis
    InsufficientData {
        analysis_type: String,
        reason: String,
    },
    /// Calculation error
    CalculationFailed {
        calculation_type: String,
        reason: String,
    },
    /// Position data is invalid
    InvalidPosition {
        position_id: String,
        reason: String,
    },
    /// Statistical analysis failed
    StatisticalError {
        metric_name: String,
        reason: String,
    },
    /// Market data is stale or outdated
    StaleData {
        market_slug: String,
        last_update: String,
    },
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::InsufficientData {
                analysis_type,
                reason,
            } => {
                write!(
                    f,
                    "Insufficient data for {} analysis: {}",
                    analysis_type, reason
                )
            }
            AnalysisError::CalculationFailed {
                calculation_type,
                reason,
            } => {
                write!(f, "{} calculation failed: {}", calculation_type, reason)
            }
            AnalysisError::InvalidPosition { position_id, reason } => {
                write!(f, "Invalid position '{}': {}", position_id, reason)
            }
            AnalysisError::StatisticalError { metric_name, reason } => {
                write!(f, "Statistical error for metric '{}': {}", metric_name, reason)
            }
            AnalysisError::StaleData {
                market_slug,
                last_update,
            } => {
                write!(
                    f,
                    "Market '{}' data is stale (last update: {})",
                    market_slug, last_update
                )
            }
        }
    }
}

impl std::error::Error for AnalysisError {}

// ============================================================================
// Output Layer Errors
// ============================================================================

/// Errors that occur when formatting or displaying output
#[derive(Debug)]
pub enum OutputError {
    /// Failed to format data for display
    FormattingFailed {
        data_type: String,
        reason: String,
    },
    /// Failed to write output
    WriteFailed {
        destination: String,
        reason: String,
    },
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputError::FormattingFailed { data_type, reason } => {
                write!(f, "Failed to format {} for output: {}", data_type, reason)
            }
            OutputError::WriteFailed { destination, reason } => {
                write!(f, "Failed to write output to {}: {}", destination, reason)
            }
        }
    }
}

impl std::error::Error for OutputError {}

// ============================================================================
// Error Conversions
// ============================================================================

impl From<HttpError> for AppError {
    fn from(error: HttpError) -> Self {
        AppError::Http(error)
    }
}

impl From<DataSourceError> for AppError {
    fn from(error: DataSourceError) -> Self {
        AppError::DataSource(error)
    }
}

impl From<ParseError> for AppError {
    fn from(error: ParseError) -> Self {
        AppError::Parse(error)
    }
}

impl From<NormalizationError> for AppError {
    fn from(error: NormalizationError) -> Self {
        AppError::Normalization(error)
    }
}

impl From<AnalysisError> for AppError {
    fn from(error: AnalysisError) -> Self {
        AppError::Analysis(error)
    }
}

impl From<OutputError> for AppError {
    fn from(error: OutputError) -> Self {
        AppError::Output(error)
    }
}

// ============================================================================
// External Crate Conversions
// ============================================================================

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        let url = error.url().map(|u| u.to_string()).unwrap_or_default();

        if error.is_timeout() {
            AppError::Http(HttpError::Timeout {
                url,
                duration_secs: 30, // Default timeout
            })
        } else if error.is_connect() {
            AppError::Http(HttpError::ConnectionFailed {
                url,
                reason: error.to_string(),
            })
        } else if error.is_status() {
            let status = error.status().map(|s| s.as_u16()).unwrap_or(0);
            AppError::Http(HttpError::RequestFailed {
                status,
                url,
                body: error.to_string(),
            })
        } else {
            AppError::Http(HttpError::ResponseReadError {
                url,
                reason: error.to_string(),
            })
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::Parse(ParseError::JsonDeserializationFailed {
            field_name: None,
            expected_type: "JSON".to_string(),
            json_snippet: format!("at line {}, column {}", error.line(), error.column()),
            reason: error.to_string(),
        })
    }
}

// ============================================================================
// Result Type Alias
// ============================================================================

/// Convenience type alias for Results using AppError
pub type Result<T> = std::result::Result<T, AppError>;

// ============================================================================
// Helper Functions
// ============================================================================

/// Truncate a string for display in error messages
pub fn truncate_for_display(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}... (truncated)", &s[..max_len])
    }
}

/// Extract a relevant snippet from JSON for error messages
pub fn json_error_snippet(json: &str, max_len: usize) -> String {
    let snippet = truncate_for_display(json, max_len);
    // Remove excessive whitespace
    snippet
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ")
}
