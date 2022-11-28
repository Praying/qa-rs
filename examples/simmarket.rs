use qa_rs::qamarket::simmarket::QASIMMarket;

fn main() {
    let mut market = QASIMMarket::new();
    market.register_account("x1", 10000000.0);
    market.register_account("x2", 10000000.0);
    market.sync_time();
    market.insert_order("x1", "YUT", 20.0, 1000.0, "BUY", "OPEN");
    // println!("{:#?}", market.get_account("x1").dailyorders);
    //println!("{:?}", market.orders);
    market.insert_order("x1", "YUT", 17.0, 1000.0, "BUY", "OPEN");
    //println!("{:?}", market.orders);
    //println!("{:#?}", market.get_account("x1").dailyorders);
    market.matchs();
    println!("new orders");
    market.insert_order("x1", "YUT", 22.0, 1000.0, "SELL", "OPEN");
    market.matchs();
    println!("new orders from x2 ");
    market.insert_order("x2", "YUT", 19.0, 4000.0, "SELL", "OPEN");
    market.matchs();
    market.insert_order("x2", "YUT", 19.0, 4000.0, "SELL", "OPEN");
    market.matchs();

    println!("{:#?}", market.get_account("x1").trades);
    println!("{:#?}", market.get_account("x1").dailyorders);
    println!("{:#?}", market.get_account("x2").dailyorders);
    //println!("{}", parse_datestamp(1669596950000000000));
}
