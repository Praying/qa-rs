use hashbrown::HashMap as bmap;
use polars::prelude::*;
use qa_rs::qadata::localdata::QADataStructStockDay;
use qa_rs::qadata::marketcenter::{run_t, Kline, QAMarketCenter};
use qa_rs::qamarket::marketsys::QAMarket;
use std::collections::HashMap;
use std::fs::File;
#[warn(unused_doc_comments)]
fn main() {
    let mut file = File::open("..\\data\\mkt.pq").expect("Cannot open file.");
    let df = ParquetReader::new(&mut file).finish().unwrap();
    println!("{:?}", df);

    let mut MC = QAMarketCenter::new(
        df.partition_by(vec!["date"])
            .unwrap()
            .iter()
            .map(|df| {
                let (k, v) = run_t(df.clone());
                (k, v)
            })
            .collect::<bmap<i32, bmap<String, Kline>>>(),
    );
    let mut market = QAMarket::new();
    let data = QADataStructStockDay::load_mkt();
    market.register_account("BLX_Factor_p5d_std_twap_norank");
    let acc = market.get_account("BLX_Factor_p5d_std_twap_norank");
    acc.change_datetime("2022-01-03".to_string());
    //acc.buy("000001.XSHE", 20.0, "2020-01-03", 2.0).unwrap();
    let mut targetpanel = HashMap::new();
    targetpanel.insert("000001.XSHE".to_string(), 2.0);
    targetpanel.insert("000002.XSHE".to_string(), 4.0);
    let mut targetmap = HashMap::new();
    targetmap.insert("000001.XSHE".to_string(), 22.0);
    targetmap.insert("000002.XSHE".to_string(), 62.0);
    acc.target(targetpanel, targetmap);
    market.run_single(data)
    //market.run("2020-01-01", "2020-01-03");
}
