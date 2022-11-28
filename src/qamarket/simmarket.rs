use crate::qaaccount::account::QA_Account;
use crate::qaaccount::order::QAOrder;
use crate::qamarket::matchengine::{
    domain::OrderDirection,
    orderbook::{Failed, Orderbook, Success},
    orders,
    orders::OrderRequest,
};
use crate::qaprotocol::qifi::account::QIFI;
use crate::qautil::parse_datestamp;
use chrono::Utc;
use uuid::Uuid;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
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
        "666666" => Some(SimMarketAsset::YUT),

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
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickSim {
    pub datetime: i64,
    pub bid1: f64,
    pub ask1: f64,
    pub bid_volume1: f64,
    pub ask_volume1: f64,
    pub last_price: f64,
    pub last_volume: f64,
    
}
impl TickSim {
    pub fn to_human(&self) -> (String,f64, f64, f64, f64, f64, f64, ) {
        // println!("bid1:{},ask1:{},bid_volume1:{},ask_volume1:{},last_price:{},last_volume:{},datetime:{}",
        // self.bid1,self.ask1,self.bid_volume1,self.ask_volume1,self.last_price,self.last_volume,parse_datestamp( self.datetime));
        (
            parse_datestamp(self.datetime),
            self.bid1,
            self.ask1,
            self.bid_volume1,
            self.ask_volume1,
            self.last_price,
            self.last_volume,
            
        )
    }
}
#[pyclass]
#[allow(unused_must_use)]
pub struct QASIMMarket {
    pub reg_account: HashMap<String, QA_Account>,
    pub OrderBook: HashMap<String, Orderbook<SimMarketAsset>>,
    // pub orders: HashMap<String, QAOrder>,
    pub matchingE_id: HashMap<String, HashMap<u64, QAOrder>>, // SIMMarketID{matching, order_id}
    time: String,
    portfolio_name: String,
    pub trading_day: String,
    pub lastpricepanel: HashMap<String, f64>,
    pub hisprice: HashMap<String, Vec<TickSim>>,
}

impl QASIMMarket {
    pub fn new() -> Self {
        let mut match_engine = HashMap::new();
        match_engine.insert("666666".to_string(), Orderbook::new(SimMarketAsset::YUT));

        let mut matchingE_id = HashMap::new();
        matchingE_id.insert("666666".to_string(), HashMap::new());

        let mut hisprice = HashMap::new();
        hisprice.insert("666666".to_string(), vec![]);
        QASIMMarket {
            reg_account: HashMap::new(),
            matchingE_id,
            OrderBook: match_engine,
            time: String::new(),
            trading_day: String::new(),
            portfolio_name: "qasim_market".to_string(),
            lastpricepanel: HashMap::new(),
            hisprice,
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

    // pub fn get_l1_orders(&self) -> Vec<orders::OrderRequest> {
    //     let mut orders = vec![];
    //     for orderbook in self.OrderBook.values() {
    //         orders.push(orderbook.get_l1_orders());
    //     }
    //     orders
    // }

    
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
                "sim",
            );
            println!("{}={} is registered", account_name, acc.environment);
            self.reg_account.insert(account_name.to_string(), acc);
        }
    }

    pub fn get_allaccount_qifi(&mut self) -> Vec<QIFI> {
        self.pub_marketdata_to_allaccount();
        let mut qifi = Vec::new();
        for acc in self.reg_account.values_mut() {
            qifi.push(acc.get_qifi_slice());
        }
        qifi
    }

    pub fn pub_marketdata_to_allaccount(&mut self) {
        self.sync_time();
        for acc in self.reg_account.values_mut() {
            for (code, lastprice) in self.lastpricepanel.iter() {
                acc.on_price_change(code.to_string(), lastprice.clone(), self.time.clone());
            }
        }
    }
    pub fn settle(&mut self) {
        //market settle event
        let datetime = self.time.clone();
        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                let price = self.lastpricepanel.get(&code).unwrap();
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
                let result = account.send_order(
                    code,
                    amount,
                    &self.time,
                    towards,
                    price,
                    &market_order_id,
                    "LIMIT",
                );
                match result {
                    Ok(order) => {
                        let single_order_book = self.OrderBook.get_mut(code).unwrap();

                        let orderx = orders::new_limit_order_request(
                            code_asset,
                            parse_order_direction(&order.direction).unwrap(),
                            order.price,
                            order.volume,
                            Utc::now().timestamp_nanos(),
                        );
                        let result = single_order_book.process_order(orderx);
                        self.on_insert_order_book(order, result);
                    }
                    Err(e) => {
                        println!("{} not allowed to send order", account_cookie);
                    }
                }
            }
            _ => {
                println!("{} is not supported", code);
            }
        }
    }
    pub fn get_l1_tick(&self, code: &str) -> &Vec<TickSim> {
        self.hisprice.get(code).unwrap()
    }
    pub fn get_l1_tick_human(
        &self,
        code: &str,
    ) -> Vec<(String, f64, f64, f64, f64, f64, f64, )> {
        let price = self.hisprice.get(code).unwrap().clone();
        price.iter().map(|x| x.to_human()).collect()
    }
    pub fn on_insert_order_book(&mut self, order: QAOrder, result: Vec<Result<Success, Failed>>) {
        let mut orderbook = self.OrderBook.get_mut(&order.instrument_id).unwrap();
        let matchE = self.matchingE_id.get_mut(&order.instrument_id).unwrap();
        result.iter().for_each(|res| {
            match res {
                Ok(event) => {
                    match event {
                        Success::Accepted { id, order_type, ts } => {
                            println!("Accepted order {} of type {:?} at {:?}", id, order_type, ts);

                            matchE.insert(id.clone(), order.clone());
                        }
                        Success::Filled {
                            order_id,
                            order_type,
                            ts,
                            price,
                            volume,
                            direction,
                        } => {
                            println!(
                                "Matched order {} of type {:?} at {:?} for {} units at {}",
                                order_id, order_type, ts, volume, price
                            );
                            //let matchE = self.matchingE_id.get_mut(&order.instrument_id).unwrap();
                            let order = matchE.get_mut(order_id).unwrap();
                            println!("matched order {:?}", order);

                            let acc = self.reg_account.get_mut(&order.account_cookie).unwrap();
                            acc.receive_deal_sim(
                                order.instrument_id.clone(),
                                volume.clone(),
                                price.clone(),
                                parse_datestamp(ts.clone()),
                                order.order_id.clone(),
                                order.order_id.clone(),
                                order.order_id.clone(),
                                order.towards.clone(),
                            );
                            self.lastpricepanel
                                .insert(order.instrument_id.clone(), price.clone());
                            let (bp, bv, ap, av, lp) = orderbook.get_l1_tick();
                            self.hisprice
                                .get_mut(order.instrument_id.as_str())
                                .unwrap()
                                .push(TickSim {
                                    bid1: bp,
                                    ask1: ap,
                                    bid_volume1: bv,
                                    ask_volume1: av,
                                    last_price: lp,
                                    last_volume: volume.clone(),
                                    datetime: ts.clone(),
                                })
                            // 成交事件
                            // 成交需要通知 双方账户
                            //let order = self.orders.get_mut(order_id).unwrap();
                        }
                        Success::Cancelled { id, ts } => {
                            println!("Cancelled order {} of at {:?}", id, ts);
                        }
                        Success::PartiallyFilled {
                            order_id,
                            direction,
                            order_type,
                            price,
                            volume,
                            ts,
                        } => {
                            println!(
                                "Partially filled order {} of type {:?} at {:?} for {} units at {}",
                                order_id, order_type, ts, volume, price
                            );
                            //let matchE = self.matchingE_id.get_mut(&order.instrument_id).unwrap();
                            let order = matchE.get_mut(order_id).unwrap();
                            println!("matched order {:?}", order);

                            let acc = self.reg_account.get_mut(&order.account_cookie).unwrap();
                            acc.receive_deal_sim(
                                order.instrument_id.clone(),
                                volume.clone(),
                                price.clone(),
                                parse_datestamp(ts.clone()),
                                order.order_id.clone(),
                                uuid::Uuid::new_v4().to_string(),
                                order.order_id.clone(),
                                order.towards.clone(),
                            );
                            self.lastpricepanel
                                .insert(order.instrument_id.clone(), price.clone());
                            let (bp, bv, ap, av, lp) = orderbook.get_l1_tick();
                            self.hisprice
                                .get_mut(order.instrument_id.as_str())
                                .unwrap()
                                .push(TickSim {
                                    bid1: bp,
                                    ask1: ap,
                                    bid_volume1: bv,
                                    ask_volume1: av,
                                    last_price: lp,
                                    last_volume: volume.clone(),
                                    datetime: ts.clone(),
                                })
                        }
                        Success::Amended {
                            id,
                            price,
                            volume,
                            ts,
                        } => {
                            println!(
                                "Amended order {} at {:?} to {} units at {}",
                                id, ts, volume, price
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            };
        });
    }

    pub fn plot_askbid(&mut self) {
        // trade event
        for (orderid, single_order_book) in self.OrderBook.iter_mut() {
            single_order_book.plot_orderbook();
        }
    }
}


#[pymethods]
impl QASIMMarket {
    #[new]
    fn new_from_py() -> Self {
        Self::new()
    }

    #[pyo3(name = "init_market")]
    pub fn init_market(&mut self, account_list: HashMap<String, f64>) {
        for acc in account_list.keys() {
            self.register_account(acc, account_list[acc])
        }
    }
    #[pyo3(name = "sync_time")]
    pub fn py_sync_time(&mut self) {
        self.sync_time()
    }

    #[pyo3(name = "insert_order")]
    pub fn py_insert_order(&mut self, account_cookie:&str, code:&str, price:f64, amount:f64, direction:&str, offset:&str) {
        self.insert_order(account_cookie, code, price, amount, direction, offset);
    }
    #[pyo3(name = "plot_askbid")]
    pub fn py_plot_askbid(&mut self) {
        self.plot_askbid()
    }

    #[pyo3(name = "get_l1tick")]
    pub fn py_l1tick(&mut self, ) -> String {
        serde_json::to_string(&self.hisprice).unwrap()
    }

    #[pyo3(name = "get_lastprice")]
    pub fn py_get_lastprice(&mut self, ) -> String {
        serde_json::to_string(&self.lastpricepanel).unwrap()
    }

    #[pyo3(name = "get_all_account")]
    pub fn py_get_all_account(&mut self, ) -> PyResult<String> {
        self.pub_marketdata_to_allaccount();
        let qifislice: HashMap<String, QIFI> = self
        .reg_account
        .par_iter_mut()
        .map(|(accname, acc)| (accname.clone(), acc.get_qifi_slice()))
        .collect::<HashMap<String, QIFI>>();
        // let portqifi =
        //     QA_Portfolio::new(self.portfolio_name.clone(), qifislice).get_portfolio_qifi();
        let v = serde_json::to_string(&qifislice).unwrap();
        Ok(v)
    }
}