use anyhow::Result;
use polars::prelude::*; // dataframe and lazyframe
use std::path::PathBuf;

pub struct ParquetReader {
    data_dir: PathBuf,
}

impl ParquetReader {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: PathBuf::from(data_dir),
        }
    }

    // read from parquet into lazyframe
    pub fn read_lazy(&self, filename: &str) -> Result<LazyFrame> {
        // path for the table
        let path = self.data_dir.join(filename);

        if !path.exists() {
            anyhow::bail!("Parquet file not found {:?}", path);
        }

        let frame = LazyFrame::scan_parquet(path, Default::default())?;

        Ok(frame)
    }

    // read parquet file into DataFrame
    pub fn read(&self, filename: &str) -> Result<DataFrame> {
        let lazy = self.read_lazy(filename)?;

        Ok(lazy.collect()?)
    }

    // check if file exists 
    pub fn exists(&self, filename: &str) -> bool {
        self.data_dir.join(filename).exists()
    }
}
