use crate::standard_data::models::{MarketGroup, Market};

// helper to  print section headers
pub fn print_header(title: &str) {
    let lines = "=".repeat(67);
    println!("\n{}", lines);
    println!("{}", title);
    println!("{}", lines);
}

pub fn print_market_group_info(group: &MarketGroup) {
    print_header("MARKET GROUP");
    
    println!("  Title: {}", group.title);
    println!("  Slug: {}", group.slug);
    
    println!("  Total Volume: ${:.2}", group.volume);
    println!("  Total Liquidity: ${:.2}", group.liquidity);
    println!("  Active: {}", group.active);
    println!("  Closed: {}", group.closed);
    println!("  Number of Sub Markets: {}", group.markets.len());
    println!();
}

pub fn print_market_info(market: &Market) {
    println!("  Question: {}", market.question);
    println!("  Slug: {}", market.slug);
    println!("  Condition ID: {}", market.condition_id);
    println!("  YES Token: {}", market.yes_token_id);
    println!("  NO Token: {}", market.no_token_id);
    
    if market.outcomes.len() == 2 && market.outcome_prices.len() == 2 {
        println!("  YES Price: {}", market.outcome_prices[0]);
        println!("  NO Price: {}", market.outcome_prices[1]);
    }
    
    println!("  Volume: ${:.2}", market.volume);
    println!("  Volume 24hr: ${:.2}", market.volume_24h);
    println!("  Volume 1 week: ${:.2}", market.volume_1w);
    println!("  Volume 1 month: ${:.2}", market.volume_1m);
    println!("  Volume 1 year: ${:.2}", market.volume_1y);
    println!("  Competitive: {:.5}", market.competitive);
    println!("  Last trade price: ${:.5}", market.last_trade_price);
    println!("  Best Bid Price: ${:.5}", market.bid_price);
    println!("  Best Ask Price: ${:.5}", market.ask_price);
    
    println!();
}
