use crate::standard_data::models::Market;

// helper to  print section headers
pub fn print_header(title: &str) {
    let lines = "=".repeat(67);
    println!("\n{}", lines);
    println!("{}", title);
    println!("{}", lines);
}

pub fn print_market_info(market: &Market) {
    print_header("MARKET INFORMATION");
    
   // println!("  Question:      {}", market.question);
    
    println!("  Slug:          {}", market.slug);
    //println!("  Condition ID:  {}", market.condition_id);
    println!("  YES Token:     {}", market.yes_token_id);
    println!("  NO Token:      {}", market.no_token_id);
    
    println!("  Active:        {}", market.active);
    println!("  Closed:        {}", market.closed);
    println!();
}
