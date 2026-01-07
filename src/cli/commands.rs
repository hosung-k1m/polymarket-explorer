use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "polymarket-explorer",
    version = "0.1.0",
    about = "explore more into polymarket"
)]

pub struct CLI {
    // gets slug
    #[arg(short, long)]
    pub market_slug: String,
}
