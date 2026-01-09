mod cli;
mod standard_data;
mod adapters;
mod data_sources;

use clap::Parser;
use cli::{CLI, handle_analyze};
use adapters::HttpClient;
use data_sources::{PolymarketApiSource, LocalDbSource};

#[tokio::main]
async fn main() {
    // run and parse slug or error
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        
        // Print error chain
        if let Some(cause) = e.source() {
            eprintln!("\nCaused by:");
            for (i, err) in std::iter::successors(Some(cause), |e| e.source()).enumerate() {
                eprintln!("  {}: {}", i, err);
            }
        }
        
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // parse
    let cli = CLI::parse();

    // create http cleint
    let http_client = HttpClient::new();

    // make polymarket api source
    let market_provider = PolymarketApiSource::new(http_client);

    // local db source
    let local_db = LocalDbSource::new("/Users/hosungkim/data/poly/processed_data");

    // run
    handle_analyze(
            &cli.market_slug,
            &market_provider,
            &local_db, // trader stats provider
            &local_db, // position provider
    ).await
    
}
