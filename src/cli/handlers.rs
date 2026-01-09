use crate::cli::output;
use anyhow::Result;
use crate::standard_data::providers::{MarketMetadataProvider, TraderStatsProvider, PositionProvider};

// print the results from the market, takes in a marketprovider
pub async fn handle_analyze<M, T, P>(
    market_slug: &str,
    market_provider: &M,
    trader_provider: &T,
    position_provider: &P,
) -> Result<()> 
where   
    M: MarketMetadataProvider,
    T: TraderStatsProvider,
    P: PositionProvider,
{
    // get market info
    output::print_header(&format!("Fetching market: {}", market_slug));
    let market_group = market_provider.get_market_group(market_slug).await?;
    
    // display market info
    output::print_market_group_info(&market_group);
    
    // TODO: change here for deciding what market to analyse right now just first
    if let Some(first_market) = market_group.markets.first() {
        output::print_header("ANALYZING PRIMARY MARKET");
        output::print_market_info(first_market);

        let condition_id = &first_market.condition_id;

        // get positions
        output::print_header("FETCHING POSITION DATA");
        let positions = position_provider.get_positions(condition_id).await?;
        println!("  Found {} positions for this market", positions.len());

        let trader_addresses: Vec<String> = positions
            .iter()
            .map(|p| p.trader_address.clone())
            .collect();
        
        output::print_header("TRADER STATS");
        let traders = trader_provider.get_traders_by_addresses(&trader_addresses).await?;
        println!("  Found {} traders", traders.len());

        println!("Sample data: ");
        if let Some(first_position) = positions.first() {
            println!("\n  Sample position:");
            println!("    Trader: {}", first_position.trader_address);
            println!("    Side: {}", first_position.side);
            println!("    Shares: {}", first_position.shares_held);
            println!("    Avg Price: ${:.4}", first_position.avg_entry_price);
        }
        
        if let Some(first_trader) = traders.first() {
            println!("\n  Sample trader:");
            println!("    Address: {}", first_trader.trader_address);
            println!("    Accuracy: {:.1}%", first_trader.accuracy * 100.0);
            println!("    ROI: {:.1}%", first_trader.roi * 100.0);
            println!("    Markets: {}", first_trader.total_markets_resolved);
        }

        // TODO: Fetch positions and calculate statistics
        output::print_header("ANALYSIS");
        println!("something will be here soon");
    } else {
        println!("  No markets found in this group\n");
    }
    
    Ok(())
}
