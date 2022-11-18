use crate::qaaccount::account::{QA_Account};

use crate::qadata::marketcenter::{Kline, QAMarketCenter};
use crate::qaenv::localenv::CONFIG;
use crate::qautil::tradedate::{QATradeDate, QA_util_datelist_to_string};
use hashbrown::HashMap;
use polars::prelude::*;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass]
#[allow(unused_must_use)]
pub struct QAMarket {
    pub reg_account: HashMap<String, QA_Account>,
    pub data: QAMarketCenter,
    pub market_event: DataFrame,
    pub today: String,
}

impl QAMarket {
    pub fn new() -> Self {
        Self {
            reg_account: HashMap::new(),
            data: QAMarketCenter::new(&CONFIG.DataPath.cache),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
        }
    }
    pub fn new_from_path(path: &str) -> Self {
        Self {
            reg_account: HashMap::new(),
            data: QAMarketCenter::new(path),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
        }
    }
    pub fn register_account(&mut self, account_name: &str) {
        if self.reg_account.contains_key(account_name) {
            println!("{} is already registered", account_name);
        } else {
            let acc = QA_Account::new(
                account_name,
                "portfolio",
                "admin",
                10000000000000.0,
                false,
                "backtest",
            );
            self.reg_account.insert(account_name.to_string(), acc);
        }
    }
    pub fn settle(&mut self, slice: &HashMap<String, Kline>) {
        //market settle event
    }

    pub fn before_trade(&mut self) {
        // check holding
        // from settle table
    }
    pub fn get_account(&mut self, account_name: &str) -> &mut QA_Account {
        self.reg_account.get_mut(account_name).unwrap()
    }

    pub fn run(&mut self, start: &str, end: &str) {
        self.on_initmarket();
        for date in QA_util_datelist_to_string(QATradeDate::new().get_index(start, end)) {
            //println!("date: {}", date);
            self.today = date.clone();
            self.singleday(&date);
        }
    }

    // pub fn on_bar(&mut self, data: &HashMap<String, Kline>) {
    //     println!("on bar")
    // }
    // pub fn on_initmarket(&mut self) {}
    #[allow(unused_must_use)]
    pub fn target(
        &mut self,
        account_name: &str,
        target_panel: &HashMap<String, f64>,
        slice: &HashMap<String, Kline>,
    ) {
        if self.reg_account.contains_key(account_name) {
            let today = self.today.clone();
            let acc = self.get_account(account_name);

            let mut order_list = Vec::new();
            for code in acc.hold.keys() {
                if target_panel.contains_key(code) {
                } else {
                    let pos = acc.hold.get(code).unwrap();
                    let current_data = slice.get(code).unwrap();
                    if pos.volume_long_his > 0.0 {
                        order_list.push((
                            code.to_owned(),
                            current_data.close,
                            pos.volume_long_his,
                            "sell_close",
                            current_data.limit_down,
                        ));
                    }
                    if pos.volume_long_today > 0.0 {
                        order_list.push((
                            code.to_owned(),
                            current_data.close,
                            pos.volume_long_today,
                            "sell_closetoday",
                            current_data.limit_down,
                        ));
                    }
                    if pos.volume_short_his > 0.0 {
                        order_list.push((
                            code.to_owned(),
                            current_data.close,
                            pos.volume_short_his,
                            "buy_close",
                            current_data.limit_up,
                        ));
                    }
                    if pos.volume_short_today > 0.0 {
                        order_list.push((
                            code.to_owned(),
                            current_data.close,
                            pos.volume_short_today,
                            "buy_closetoday",
                            current_data.limit_up,
                        ));
                    }
                }
            }
            for (code, target) in target_panel {
                let current_data = slice.get(code).unwrap();

                let posx = acc.get_position(code);
                let amount = target.clone();
                let price = current_data.close;
                if posx.is_none() {
                    if amount > 0.0 {
                        order_list.push((
                            code.to_owned(),
                            current_data.close,
                            amount,
                            "buy_open",
                            current_data.limit_up,
                        ));
                    } else {
                        order_list.push((
                            code.to_owned(),
                            current_data.close,
                            amount.abs(),
                            "sell_open",
                            current_data.limit_down,
                        ));
                    }
                } else {
                    let pos = posx.unwrap();
                    let volume_long = pos.volume_long();
                    let volume_short = pos.volume_short();

                    if volume_long + volume_short > 0.0 {
                        // code有持仓且,调仓

                        if amount > 0.0 {
                            if volume_short > 0.0 {
                                // 先平空单
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    volume_short,
                                    "buy_close",
                                    current_data.limit_up,
                                ));
                            }

                            if amount > volume_long {
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    amount - volume_long,
                                    "buy_open",
                                    current_data.limit_up,
                                ));
                                //self.acc.buy_open(code, price, amount - volume_long);
                            } else if amount < volume_long {
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    volume_long - amount,
                                    "sell_close",
                                    current_data.limit_down,
                                ));
                                //self.acc.sell_close(code, price, volume_long - amount);
                            }
                        } else if amount < 0.0 {
                            if volume_long > 0.0 {
                                // 先平多单
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    volume_long,
                                    "sell_close",
                                    current_data.limit_down,
                                ));
                                //self.acc.sell_close(code, price, volume_long);
                            }

                            let amount = amount.abs();
                            if amount > volume_short {
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    amount - volume_short,
                                    "sell_open",
                                    current_data.limit_down,
                                ));
                                //self.acc.sell_open(code, price, amount - volume_short);
                            } else if amount < volume_short {
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    volume_short - amount,
                                    "buy_close",
                                    current_data.limit_up,
                                ));
                                //self.acc.buy_close(code, price, volume_short - amount);
                            }
                        } else {
                            //amount==0全平
                            if volume_long > 0.0 {
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    volume_long,
                                    "sell_close",
                                    current_data.limit_down,
                                ));
                                //self.acc.sell_close(code, price, volume_long);
                            }
                            if volume_short > 0.0 {
                                order_list.push((
                                    code.to_owned(),
                                    price,
                                    volume_short,
                                    "buy_close",
                                    current_data.limit_up,
                                ));
                                //self.acc.buy_close(code, price, volume_short);
                            }
                        }
                    } else if volume_long + volume_short == 0.0 {
                        // code没有持仓,新开仓
                        if amount > 0.0 {
                            order_list.push((
                                code.to_owned(),
                                price,
                                amount,
                                "buy_open",
                                current_data.limit_up,
                            ));
                            //self.acc.buy_open(code, price, amount);
                        } else {
                            order_list.push((
                                code.to_owned(),
                                price,
                                amount.abs(),
                                "sell_open",
                                current_data.limit_down,
                            ));
                            //self.acc.sell_open(code, price, -amount);
                        }
                    }

                    // 获取到pos但volume可能为0
                }
            }

            for order in order_list.iter() {
                match order.3 {
                    //"buy_open"=>self.acc.buy_open(order.0,order.1,order.2),
                    //"sell_open"=>self.acc.sell_open(order.0,order.1,order.2),
                    "buy_close" => {
                        if order.4 <= order.1 {
                            println!(
                                "errorlog:{:#?} buyclose  highlimit {:#?}/{:#?}/{:#?}/{:#?}/{:#?}/",
                                today.clone(),
                                order.0.as_str(), //code
                                order.1,          //price
                                order.2,          //amount
                                order.3,          //order_type
                                order.4           //limit
                            );
                        } else {
                            acc.buy_close(order.0.as_str(), order.2, &today.clone(), order.1);
                        }
                    }
                    "sell_close" => {
                        if order.4 >= order.1 {
                            println!(
                                "errorlog:{:#?} sellclose  lowlimit {:#?}/{:#?}/{:#?}/{:#?}/{:#?}/",
                                &today.clone(),
                                order.0.as_str(),
                                order.1,
                                order.2,
                                order.3,
                                order.4
                            );
                        } else {
                            acc.sell_close(order.0.as_str(), order.2, &today.clone(), order.1);
                        }
                    }
                    _ => {}
                }
            }
            for order2 in order_list.iter() {
                match order2.3 {
                    "buy_open" => {
                        if order2.4 <= order2.1 {
                            println!(
                                "errorlog:{:#?} buyopen  highlimit {:#?}/{:#?}/{:#?}/{:#?}/{:#?}/",
                                &today.clone(),
                                order2.0.as_str(),
                                order2.1,
                                order2.2,
                                order2.3,
                                order2.4
                            );
                        } else {
                            acc.buy_open(order2.0.as_str(), order2.2, &today.clone(), order2.1);
                        }
                    }
                    "sell_open" => {
                        if order2.4 >= order2.1 {
                            println!(
                                "errorlog:{:#?} sellopen  lowlimit {:#?}/{:#?}/{:#?}/{:#?}/{:#?}/",
                                &today.clone(),
                                order2.0.as_str(),
                                order2.1,
                                order2.2,
                                order2.3,
                                order2.4
                            );
                        } else {
                            acc.sell_open(order2.0.as_str(), order2.2, &today.clone(), order2.1);
                        }
                    }

                    //"buy_close"=>self.acc.buy_close(order2.0,order2.1,order2.2),
                    //"sell_close"=>self.acc.sell_close(order2.0,order2.1,order2.2),
                    _ => {}
                }
            }
        }
    }

    pub fn singleday(&mut self, date: &str) {
        let slice = self.data.get_date(date);

        // before trading
        for acc in self.reg_account.values_mut() {
            acc.change_datetime(date.to_string());
            for code in acc.get_code_subscribed() {
                let bar = slice.get(&code).unwrap();
                if bar.dividend_cash_before_tax > 0.0 {
                    acc.dividend_event(&code, bar.dividend_cash_before_tax)
                }
                if bar.split_coefficient_to > 0.0 {
                    acc.split_event(&code, bar.split_coefficient_to)
                }
            }
        }

        //on bar
        self.on_bar(&slice);

        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                acc.on_price_change(
                    code.clone(),
                    slice.get(&code).unwrap().close,
                    self.today.clone(),
                )
            }

            acc.settle()
        }
    }

    // pub fn target_weight(
    //     &mut self,
    //     account_name: &str,
    //     targetpanel: HashMap<String, f64>,
    //     model: &str,
    // ) {
    //     if self.reg_account.contains_key(account_name) {
    //         let acc = self.get_account(account_name);
    //         match model {
    //             "twap_30" => {
    //                 let datacolumn = self.data.data.select(["order_book_id", "twap_30"]);
    //             }
    //         }
    //     }
    // }
}

pub trait QAStrategy {
    fn on_bar(&mut self, data: &HashMap<String, Kline>) {}
    fn on_initmarket(&mut self) {}
}

impl QAStrategy for QAMarket {
    fn on_initmarket(&mut self) {
        self.register_account("x1")
    }
    fn on_bar(&mut self, data: &HashMap<String, Kline>) {
        // println!("dd{:#?}",self.today);
        // println!("dd{:#?}", self.get_account("x1").get_account_info());
    }
}

#[pymethods]
impl QAMarket {
    #[new]
    fn new_from_py(path: &str) -> Self {
        Self {
            reg_account: HashMap::new(),
            data: QAMarketCenter::new(path),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
        }
    }

    #[pyo3(name = "run")]
    pub fn run_py(&mut self, start: &str, end: &str) {
        self.run(start, end)
    }
    #[pyo3(name = "init_market")]
    pub fn init_market(&mut self, account_list: Vec<String>) {
        for acc in account_list {
            self.register_account(&acc)
        }
    }
    pub fn init(&mut self) {}

    pub fn on_session_end(&mut self) {}

    #[pyo3(name = "set_date")]
    pub fn set_date_py(&mut self, date: &str) {
        self.today = date.to_string()
    }
    #[pyo3(name = "target_amount")]
    pub fn target_amount_py(&mut self, target_panels: HashMap<String, HashMap<String, f64>>) {
        let slice = self.data.get_date(&self.today);

        for account_name in target_panels.keys() {
            if self.reg_account.contains_key(account_name) {
                self.target(
                    account_name,
                    target_panels.get(account_name).unwrap(),
                    &slice,
                )
            }
        }
    }

    #[pyo3(name = "get_account_qifi")]
    pub fn get_account_qifi_py(&mut self, account_name: &str) -> PyResult<String> {
        if self.reg_account.contains_key(account_name) {
            let acc = self.get_account(account_name);
            let accmsg = acc.get_accountmessage();
            let v = serde_json::to_string(&accmsg).unwrap();
            Ok(v)
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }

    #[pyo3(name = "get_position_qifi")]
    pub fn get_position_qifi_py(&mut self, account_name: &str, code: &str) -> PyResult<String> {
        if self.reg_account.contains_key(account_name) {
            let acc = self.get_account(account_name);
            let pos = acc.get_position(code).unwrap();
            let v = serde_json::to_string(&pos.get_qifi_position()).unwrap();
            Ok(v)
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }

    #[pyo3(name = "get_positions_qifi")]
    pub fn get_positions_qifi_py(&mut self, account_name: &str) -> PyResult<String> {
        if self.reg_account.contains_key(account_name) {
            let acc = self.get_account(account_name);
            let v = serde_json::to_string(&acc.hold).unwrap();
            Ok(v)
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }

    #[pyo3(name = "get_closeprofit")]
    pub fn py_getcloseprofit(&mut self, account_name: &str) -> PyResult<f64> {
        if self.reg_account.contains_key(account_name) {
            Ok(self.get_account(account_name).get_closeprofit())
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }

    #[pyo3(name = "get_floatprofit")]
    pub fn py_getfloatprofit(&mut self, account_name: &str) -> PyResult<f64> {
        if self.reg_account.contains_key(account_name) {
            Ok(self.get_account(account_name).get_floatprofit())
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }

    #[pyo3(name = "get_balance")]
    pub fn py_getbalance(&mut self, account_name: &str) -> PyResult<f64> {
        if self.reg_account.contains_key(account_name) {
            Ok(self.get_account(account_name).get_balance())
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }
}
