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
    let market = market_provider.get_market(market_slug).await?;
    
    // display market info
    output::print_market_info(&market);
    
    // TODO: will have stats 
    output::print_header("ANALYSIS");
    println!("will have soon");
    
    Ok(())
}
