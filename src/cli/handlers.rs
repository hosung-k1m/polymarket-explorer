use crate::cli::output;
use anyhow::Result;
use crate::standard_data::providers::MarketMetadataProvider;

// print the results from the market, takes in a marketprovider
pub async fn handle_analyze<T: MarketMetadataProvider>(
    market_slug: &str,
    market_provider: &T,
) -> Result<()> {
    // get market info
    output::print_header(&format!("Fetching market: {}", market_slug));
    let market_group = market_provider.get_market_group(market_slug).await?;
    
    // display market info
    output::print_market_group_info(&market_group);
    
    // TODO: change here for deciding what market to analyse right now just first
    if let Some(first_market) = market_group.markets.first() {
        output::print_header("ANALYZING PRIMARY MARKET");
        output::print_market_info(first_market);
        
        // TODO: Fetch positions and calculate statistics
        output::print_header("ANALYSIS");
        println!("something will be here soon");
    } else {
        println!("  No markets found in this group\n");
    }
    
    Ok(())
}
