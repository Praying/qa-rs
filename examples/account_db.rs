
use qa_rs::qamarket::marketsys::QAMarket;
use qa_rs::qadata::localdata::QADataStructStockDay;
use std::collections::{HashMap};
#[warn(unused_doc_comments)]
fn main() {
    // account test
    // create account
    //
    // # Examples
    //
    // ```
    // use qa_rs::qaaccount::account::QA_Account;
    // ```
    //
    // account test for stock  future index  option
    //

    // environment ---> backtest / realtime
    // under "backtest" env, order will be executed immediately
    // under "realtime" env, order will be sent to broker

    //let mut acc = QA_Account::new("test", "portfolio", "admin", 1000000.0, false, "backtest");

    
    let mut market = QAMarket::new();
    let data = QADataStructStockDay::load_singleday("2020-01-03");
    market.register_account("BLX_Factor_p5d_std_twap_norank");
    let acc = market.get_account("BLX_Factor_p5d_std_twap_norank");
    acc.change_datetime("2022-01-03".to_string());
    //acc.buy("000001.XSHE", 20.0, "2020-01-03", 2.0).unwrap();
    let mut targetpanel = HashMap::new();
    targetpanel.insert("000001.XSHE".to_string() ,2.0);
    targetpanel.insert("000002.XSHE".to_string() ,4.0);
    let mut targetmap = HashMap::new();
    targetmap.insert("000001.XSHE".to_string(), 22.0);
    targetmap.insert("000002.XSHE".to_string(), 62.0);
    acc.target(targetpanel, targetmap);
    market.run_single(data)
    //market.run("2020-01-01", "2020-01-03");

}
