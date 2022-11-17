use hashbrown::HashMap;
use qa_rs::{
    qadata::marketcenter::Kline,
    qamarket::marketsys::{QAMarket, QAStrategy},
};
use std::time::Instant;
trait QAStrategy2: QAStrategy {
    fn on_bar(&mut self, data: &HashMap<String, Kline>) {}
    fn on_initmarket(&mut self) {}
}
impl QAStrategy2 for QAMarket {
    fn on_initmarket(&mut self) {
        self.register_account("x1")
    }
    fn on_bar(&mut self, data: &HashMap<String, Kline>) {
        println!("dd{:#?}", self.today);
        println!("dd{:#?}", self.get_account("x1").get_account_info());
    }
}
#[warn(unused_doc_comments)]
fn main() {
    let mut market = QAMarket::new();
    //market.run();
    let start = Instant::now();
    market.run("2020-01-01", "2021-01-03");

    println!("{}", start.elapsed().as_millis());
}
