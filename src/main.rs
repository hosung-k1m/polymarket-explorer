mod cli;
mod standard_data;
mod adapters;
mod data_sources;
mod error;

use clap::Parser;
use cli::{CLI, handle_analyze};
use adapters::HttpClient;
use data_sources::PolymarketApiSource;

#[tokio::main]
async fn main() {
    // run and parse slug or error
    if let Err(e) = run().await {
        eprintln!("\n{} {}\n", "❌ Error:", e);

        // Print helpful context based on error type
        match &e {
            error::AppError::Http(_) => {
                eprintln!("💡 Tip: Check your internet connection and verify the URL is correct.");
            }
            error::AppError::DataSource(_) => {
                eprintln!("💡 Tip: Verify the market slug exists on Polymarket.");
            }
            error::AppError::Parse(_) | error::AppError::Normalization(_) => {
                eprintln!("💡 Tip: The API response format may have changed. Please report this issue.");
            }
            error::AppError::Analysis(_) => {
                eprintln!("💡 Tip: Check that the market has sufficient data for analysis.");
            }
            error::AppError::Output(_) => {
                eprintln!("💡 Tip: Check console output permissions.");
            }
        }

        std::process::exit(1);
    }
}

async fn run() -> error::Result<()> {
    // parse
    let cli = CLI::parse();

    // create http cleint
    let http_client = HttpClient::new();

    // make polymarket api source
    let market_provider = PolymarketApiSource::new(http_client);

    // run
    handle_analyze(&cli.market_slug, &market_provider).await
    
}
