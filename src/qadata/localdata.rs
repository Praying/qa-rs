
use polars::prelude::*;
use polars_lazy::prelude::*;
use std::fs::{ File};

use crate::qaenv::localenv::CONFIG;

pub struct QADataStructStockDay {
    pub data: DataFrame,
}


impl QADataStructStockDay {
    pub fn new() -> Self {
        Self {
            data: df!([])
        }
    }


    pub fn load_singleday(date: &str) -> Self {
        let _cache = &CONFIG.DataPath.cache;
        let cachepath = format!("{}twap_stock_day_hfq_{}.pq", &CONFIG.DataPath.cache, date);
        
        let mut file = File::open(cachepath).expect("Cannot open file.");
        let df = ParquetReader::new(&mut file).finish().unwrap();
        Self { data: df }
    }

    pub fn select(&self, codelist: Vec<String>) -> DataFrame {
        let select_codelist =Series::new("", codelist);
        let res = self.data.clone().lazy().filter(col("order_book_id").is_in(lit(select_codelist))).collect().unwrap();
        res
    }

    pub fn high(&mut self) -> &Series {
        &self.data["high"]
    }

    pub fn low(&mut self) -> &Series {
        &self.data["low"]
    }
    pub fn close(&mut self) -> &Series {
        &self.data["close"]
    }

    pub fn open(&mut self) -> &Series {
        &self.data["open"]
    }
    pub fn volume(&mut self) -> &Series {
        &self.data["volume"]
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_qadata_struct_stock_day() {
        let sd = QADataStructStockDay::load_singleday("2020-01-03");

        println!("Final DataFrame:\n{}", sd.data);
        
        
    }


}
