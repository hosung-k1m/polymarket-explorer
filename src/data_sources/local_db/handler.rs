use crate::adapters::ParquetReader;
use anyhow::Result;
use polars::prelude::*;

pub struct LocalDbHandler {
    reader: ParquetReader,
}

impl LocalDbHandler {
    pub fn new(reader: ParquetReader) -> Self {
        Self {reader}
    }
    
    // fetch all traders with min resolved markets
    pub fn fetch_traders(&self, mine_resolved_markets: u32) -> Result<DataFrame> {
        let df = self.reader.read_lazy("traders.parquet")?
            .filter(col("total_markets_resolved").gt_eq(lit(mine_resolved_markets)))
            .collect()?;

        Ok(df)
    }

    // fetch specific traders by adresses
    pub fn fetch_traders_by_addresses(&self, addresses: &[String]) -> Result<DataFrame> {
        if addresses.is_empty() {
            // Return empty dataframe with correct schema
            let df = self.reader.read_lazy("traders.parquet")?
                .filter(lit(false))
                .collect()?;
            return Ok(df);
        }
        
        // Build OR condition for each address
        let mut filter_expr = col("trader_address").eq(lit(addresses[0].as_str()));
        for addr in &addresses[1..] {
            filter_expr = filter_expr.or(col("trader_address").eq(lit(addr.as_str())));
        }
        
        let df = self.reader.read_lazy("traders.parquet")?
            .filter(filter_expr)
            .collect()?;
        Ok(df)
    }

    // fetch poitions for a conditoin id
    pub fn fetch_positions(&self, condition_id: &str) -> Result<DataFrame> {
        let df = self.reader.read_lazy("positions.parquet")?
            .filter(col("market_id").eq(lit(condition_id)))
            .collect()?;
        Ok(df)
    }

    // fetch recent transactions for a condition ID
    pub fn fetch_recent_transactions(
        &self,
        condition_id: &str,
        _days_back: u32,
    ) -> Result<DataFrame> {
        // Calculate block threshold (approximate - need block timestamps for precision)
        // For now, just get all transactions for the condition_id
        let df = self.reader.read_lazy("transactions.parquet")?
            .filter(col("market_id").eq(lit(condition_id)))
            .collect()?;
        
        // TODO: Filter by time once we have timestamp data
        Ok(df)
    }
}


