use crate::qaaccount::account::QA_Account;
use crate::qaaccount::order::QAOrder;
use crate::qamarket::matchengine::{domain::OrderDirection, orders::OrderRequest, orderbook::Orderbook, orders};
use std::time::SystemTime;
use uuid::Uuid;
//use crate::qamarket::order_book::OrderBook;

/// realtime market with lots of account reg
///
///
///
use chrono::Local;
use hashbrown::HashMap;

/// setting :: simmarket rules:
/// rules trading for 24h/ every time// no tradegap
///
///
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SimMarketAsset {
    YUT,
}
// 此处目的主要是实现注册功能, 避免乱七八糟的报单
fn parse_asset(asset: &str) -> Option<SimMarketAsset> {
    match asset {
        "YUT" => Some(SimMarketAsset::YUT),

        _ => None,
    }
}

fn parse_order_direction(direction: &str) -> Option<OrderDirection> {
    match direction {
        "BUY" => Some(OrderDirection::BUY),
        "SELL" => Some(OrderDirection::SELL),
        _ => None,
    }
}
///
pub struct QASIMMarket {
    pub reg_account: HashMap<String, QA_Account>,
    pub OrderBook: HashMap<String, Orderbook<SimMarketAsset>>,
    pub orders: HashMap<String, QAOrder>,
    time: String,
    portfolio_name: String,
    pub trading_day: String,
    pub marketpanel: HashMap<String, f64>,
}

impl QASIMMarket {
    pub fn new() -> Self {
        let mut match_engine = HashMap::new();
        match_engine.insert("YUT".to_string(), Orderbook::new(SimMarketAsset::YUT));
        QASIMMarket {
            reg_account: HashMap::new(),
            orders: HashMap::new(),
            OrderBook: match_engine,
            time: String::new(),
            trading_day: String::new(),
            portfolio_name: "qasim_markte".to_string(),
            marketpanel: HashMap::new(),
        }
    }

    pub fn sync_time(&mut self) {
        self.time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.trading_day = Local::now().format("%Y-%m-%d").to_string();
        for acc in self.reg_account.values_mut() {
            acc.change_datetime(self.time.clone());
        }
    }
    pub fn get_current_time(&self) -> String {
        self.time.clone()
    }
    pub fn get_trading_day(&self) -> String {
        self.trading_day.clone()
    }

    pub fn register_account(&mut self, account_name: &str, init_cash: f64) {
        if self.reg_account.contains_key(account_name) {
            println!("{} is already registered", account_name);
        } else {
            let acc = QA_Account::new(
                account_name,
                &self.portfolio_name,
                "admin",
                init_cash,
                false,
                "real",
            );
            self.reg_account.insert(account_name.to_string(), acc);
        }
    }
    pub fn settle(&mut self) {
        //market settle event
        let datetime = self.time.clone();
        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                let price = self.marketpanel.get(&code).unwrap();
                acc.on_price_change(code, price.clone(), datetime.clone());
            }
            acc.settle();
        }
    }

    pub fn before_trade(&mut self) {
        // check holding
    }
    pub fn get_account(&mut self, account_name: &str) -> &mut QA_Account {
        self.reg_account.get_mut(account_name).unwrap()
    }

    pub fn insert_order(
        &mut self,
        account_cookie: &str,
        code: &str,
        price: f64,
        amount: f64,
        direction: &str,
        offset: &str,
    ) {

        match parse_asset(code) {
            Some(code_asset) => {
                let account = self.reg_account.get_mut(account_cookie).unwrap();
                let towards = account.get_towards(direction, offset);
                let market_order_id = Uuid::new_v4().to_string();
                let result = account.send_order(code, amount, &self.time, towards, price, &market_order_id, "LIMIT");
                match result {
                    Ok(order) => {

                        self.orders.insert(market_order_id, order);
                    }
                    Err(e) => {}
                }
            }
            _ => {
                println!("{} is not supported", code);
            }
        }
        
    }

    pub fn matchs(&mut self) {
        // trade event
        for (orderid, order) in self.orders.iter() {
            //let single_order_book = self.OrderBook.get_mut(&order.order_book_id.to_string()).unwrap();
            let single_order_book = self.OrderBook.get_mut(&order.instrument_id).unwrap();

            let orderx = orders::new_limit_order_request(
                parse_asset(&order.instrument_id).unwrap(), 
                parse_order_direction(&order.direction).unwrap(),
                order.price, order.volume, SystemTime::now());
            let result = single_order_book.process_order(orderx);
            result.iter().map(|x| {
                println!("order match {:?}", x);
            });
            single_order_book.plot_orderbook();
        }
    }
}
