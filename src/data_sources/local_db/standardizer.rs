use crate::standard_data::models::{Trader, Position, Transaction};
use anyhow::{Context, Result};
use polars::prelude::*;

pub struct LocalDbStandardizer;

impl LocalDbStandardizer {
    // convert data frame to Vec(traders)
    pub fn standardize_traders(df: DataFrame) -> Result<Vec<Trader>> {
        if df.height() == 0 {
            return Ok(Vec::new())
        }

        let mut traders = Vec::new();

        let addresses = df.column("trader_address")?.str()?;
        let total_entered = df.column("total_markets_entered")?.u32()?;
        let total_resolved = df.column("total_markets_resolved")?.u32()?;
        let total_wins = df.column("total_wins")?.u32()?;
        let accuracy = df.column("accuracy")?.f64()?;
        let total_invested = df.column("total_invested")?.f64()?;
        let total_returned = df.column("total_returned")?.f64()?;
        let roi = df.column("roi")?.f64()?;

        for i in 0..df.height() {
            traders.push(Trader {
                trader_address: addresses
                    .get(i)
                    .context("Missing trader_address")?
                    .to_string(),
                total_markets_entered: total_entered
                    .get(i)
                    .context("Missing total_markets_entered")?,
                total_markets_resolved: total_resolved
                    .get(i)
                    .context("Missing total_markets_resolved")?,
                total_wins: total_wins
                    .get(i)
                    .context("Missing total_wins")?,
                accuracy: accuracy
                    .get(i)
                    .context("Missing accuracy")?,
                total_invested: total_invested
                    .get(i)
                    .context("Missing total_invested")?,
                total_returned: total_returned
                    .get(i)
                    .context("Missing total_returned")?,
                roi: roi
                    .get(i)
                    .context("Missing roi")?,
            });
        }

        Ok(traders)
    }
    

    // convert data frame to vec(positons)    
    pub fn standardize_positions(df: DataFrame) -> Result<Vec<Position>> {
        if df.height() == 0 {
            return Ok(Vec::new());
        }

        let mut positions = Vec::new();

        let addresses = df.column("trader_address")?.str()?;
        let token_ids = df.column("token_id")?.str()?;
        let market_ids = df.column("market_id")?.str()?;
        let sides = df.column("side")?.str()?;
        let shares = df.column("shares_held")?.f64()?;
        let avg_prices = df.column("avg_entry_price")?.f64()?;
        
        // first_entry_block is optional
        let first_blocks = df.column("first_entry_block").ok()
            .and_then(|col| col.u64().ok());

        for i in 0..df.height() {
            let first_entry_block = first_blocks
                .and_then(|col| col.get(i));

            positions.push(Position {
                trader_address: addresses
                    .get(i)
                    .context("Missing trader_address")?
                    .to_string(),
                token_id: token_ids
                    .get(i)
                    .context("Missing token_id")?
                    .to_string(),
                market_id: market_ids
                    .get(i)
                    .context("Missing market_id")?
                    .to_string(),
                side: sides
                    .get(i)
                    .context("Missing side")?
                    .to_string(),
                shares_held: shares
                    .get(i)
                    .context("Missing shares_held")?,
                avg_entry_price: avg_prices
                    .get(i)
                    .context("Missing avg_entry_price")?,
                first_entry_block,
            });
        }

        Ok(positions)
    }

    // convert data frame to vec(transaction)
    pub fn standardize_transactions(df: DataFrame) -> Result<Vec<Transaction>> {
        if df.height() == 0 {
            return Ok(Vec::new());
        }

        let mut transactions = Vec::new();

        let block_numbers = df.column("block_number")?.u64()?;
        let tx_hashes = df.column("transaction_hash")?.str()?;
        let trader_addresses = df.column("trader_address")?.str()?;
        let token_ids = df.column("token_id")?.str()?;
        let sides = df.column("side")?.str()?;
        let actions = df.column("action")?.str()?;
        let shares = df.column("shares")?.f64()?;
        let usdc_amounts = df.column("usdc_amount")?.f64()?;
        let market_ids = df.column("market_id")?.str()?;

        for i in 0..df.height() {
            transactions.push(Transaction {
                block_number: block_numbers
                    .get(i)
                    .context("Missing block_number")?,
                transaction_hash: tx_hashes
                    .get(i)
                    .context("Missing transaction_hash")?
                    .to_string(),
                trader_address: trader_addresses
                    .get(i)
                    .context("Missing trader_address")?
                    .to_string(),
                token_id: token_ids
                    .get(i)
                    .context("Missing token_id")?
                    .to_string(),
                side: sides
                    .get(i)
                    .context("Missing side")?
                    .to_string(),
                action: actions
                    .get(i)
                    .context("Missing action")?
                    .to_string(),
                shares: shares
                    .get(i)
                    .context("Missing shares")?,
                usdc_amount: usdc_amounts
                    .get(i)
                    .context("Missing usdc_amount")?,
                market_id: market_ids
                    .get(i)
                    .context("Missing market_id")?
                    .to_string(),
            });
        }

        Ok(transactions)
    }
}

