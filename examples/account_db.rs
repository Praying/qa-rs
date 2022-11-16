
use qa_rs::qamarket::marketsys::QAMarket;
use std::time::{ Instant};

#[warn(unused_doc_comments)]
fn main() {

    let mut market = QAMarket::new();
    //market.run();
    let start = Instant::now();
    market.run("2020-01-01", "2021-01-03");
    println!("{}", start.elapsed().as_millis());
}
