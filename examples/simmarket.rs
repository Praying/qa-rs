//use qa_rs::qaconnector::mongo::mongoclient::QAMongoClient;
use qa_rs::qamarket::simmarket::QASIMMarket;
fn main() {
    let mut market = QASIMMarket::new();
    market.register_account("sim00001", 10000000.0);
    market.register_account("sim00002", 10000000.0);
    market.register_account("sim00003", 10000000.0);
    market.sync_time();
    market.insert_order("sim00001", "IX2301", 20.0, 1000.0, "BUY", "OPEN");
    market.insert_order("sim00003", "IX2301", 19.5, 1000.0, "BUY", "OPEN");
    // println!("{:#?}", market.get_account("sim00001").dailyorders);
    //println!("{:?}", market.orders);
    market.insert_order("sim00001", "IX2301", 17.0, 1000.0, "BUY", "OPEN");
    //println!("{:?}", market.orders);
    //println!("{:#?}", market.get_account("sim00001").dailyorders);
    market.plot_askbid();
    println!("new orders");
    market.insert_order("sim00001", "IX2301", 22.0, 1000.0, "SELL", "OPEN");
    market.plot_askbid();
    println!("new orders from sim00002 ");
    market.insert_order("sim00002", "IX2301", 19.0, 2000.0, "SELL", "OPEN");
    market.plot_askbid();
    market.insert_order("sim00002", "IX2301", 19.0, 4000.0, "SELL", "OPEN");
    market.plot_askbid();
    market.insert_order("sim00001", "IX2301", 19.0, 1000.0, "BUY", "OPEN");
    market.plot_askbid();
    // println!("{:#?}", market.get_account("sim00001").trades);
    // println!("{:#?}", market.get_account("sim00001").dailyorders);
    // println!("{:#?}", market.get_account("sim00002").dailyorders);
    // let client = QAMongoClient::new("mongodb://192.168.2.115:27017");
    // for slice in market.get_allaccount_qifi() {
    //     client.save_his_qifi_slice(slice)
    // }
    println!("hisdata \n {:#?}", market.get_l1_tick_human("IX2301"));

    //println!("{}", parse_datestamp(1669596950000000000));
}
