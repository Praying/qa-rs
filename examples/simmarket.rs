use qa_rs::qamarket::simmarket::QASIMMarket;
use chrono::{Local, Utc};
use chrono::TimeZone;

fn main() {
    let mut market = QASIMMarket::new();
    market.register_account("x1", 10000000.0);
    market.register_account("x2", 10000000.0);
    market.sync_time();
    market.insert_order("x1", "YUT", 20.0, 1000.0, "BUY", "OPEN");
    
    //println!("{:?}", market.orders);
    market.insert_order("x1", "YUT", 17.0, 1000.0, "BUY", "OPEN");
    //println!("{:?}", market.orders);
    market.matchs();
    println!("new orders");
    market.insert_order("x1", "YUT", 22.0, 1000.0, "SELL", "OPEN");
    market.matchs();
    println!("new orders from x2 ");
    market.insert_order("x2", "YUT", 19.0, 4000.0, "SELL", "OPEN");
    market.matchs();
    market.insert_order("x2", "YUT", 19.0, 4000.0, "SELL", "OPEN");
    market.matchs();
   //println!("{:?}", market.orders);
    // println!("{:?}", SystemTime::now());
    // println!("{:?}", Local::now().timestamp_nanos()-28800000000000);

    // println!("{:?}", Utc::now().timestamp_nanos()-28800000000000);
    let  utctime = Utc
    .datetime_from_str(
        "2022-11-27 22:01:22", "%Y-%m-%d %H:%M:%S")
    .unwrap()
    .timestamp_nanos()
    - 28800000000000;
    // println!("{:?}", utctime);
}