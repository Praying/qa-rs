use crate::qaaccount::account::QA_Account;
use crate::qaaccount::portfolio::QA_Portfolio;
use crate::qaanalysis::qifianalysis::QIFIAnalysis;
use crate::qadata::marketcenter::{Kline, QAMarketCenter};
use crate::qaenv::localenv::CONFIG;
use crate::qaprotocol::qifi::account::QIFI;
use crate::qautil::tradedate::{QATradeDate, QA_util_datelist_to_string};
use hashbrown::HashMap;
use polars::prelude::*;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use rayon::prelude::*;

use queues::*;
#[pyclass]
#[allow(unused_must_use)]
pub struct QAMarket {
    pub reg_account: HashMap<String, QA_Account>,
    pub portfolio_name: String,
    pub data: QAMarketCenter,
    pub market_event: DataFrame,
    pub today: String,
    pub schedule_queue: Queue<(String, String, HashMap<String, f64>, String)>,
    pub cache: HashMap<String, Vec<QIFI>>,
}

impl QAMarket {
    pub fn new() -> Self {
        Self {
            reg_account: HashMap::new(),
            portfolio_name: String::from(""),
            data: QAMarketCenter::new(&CONFIG.DataPath.cache),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
            schedule_queue: queue![],
            cache: HashMap::new(),
        }
    }
    pub fn reinit(&mut self){
        self.reg_account = HashMap::new();
        self.portfolio_name = String::from("");
        self.market_event = DataFrame::default();
        self.today = "1970-01-01".to_string();
        self.schedule_queue = queue![];
        self.cache = HashMap::new();
    }

    pub fn new_from_path(path: &str, portfolio_name: &str) -> Self {
        Self {
            reg_account: HashMap::new(),
            portfolio_name: portfolio_name.to_string(),
            data: QAMarketCenter::new(path),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
            schedule_queue: queue![],
            cache: HashMap::new(),
        }
    }
    pub fn new_for_realtime(portfolio_name: &str) -> Self {
        Self {
            reg_account: HashMap::new(),
            portfolio_name: portfolio_name.to_string(),
            data: QAMarketCenter::new_for_realtime(),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
            schedule_queue: queue![],
            cache: HashMap::new(),
        }
    }

    pub fn set_date(&mut self, date: &str) {
        self.today = date.to_string();
        for acc in self.reg_account.values_mut() {
            acc.change_datetime(self.today.clone());
        }
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
    pub fn settle(&mut self, slice: &HashMap<String, Kline>) {
        //market settle event
        let datetime = self.today.clone();
        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                let kline = slice.get(&code).unwrap();
                let price = kline.close;
                acc.on_price_change(code, price, datetime.clone());
            }
            acc.settle();
        }
    }

    pub fn before_trade(&mut self, slice: &HashMap<String, Kline>) {
        // check holding
        // from settle table

        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                let kline = slice.get(&code).unwrap();
                if kline.dividend_cash_before_tax != 0.0 {
                    println!(
                        "[qamarket event] dividend_cash {} {}",
                        code, kline.dividend_cash_before_tax
                    );
                    acc.dividend_event(&code, kline.dividend_cash_before_tax);
                }
                if kline.split_coefficient_to != 0.0 {
                    println!(
                        "[qamarket event] split {} {}",
                        code, kline.split_coefficient_to
                    );
                    acc.split_event(&code, kline.split_coefficient_to);
                }
                let price = kline.open;
                acc.on_price_change(code, price, self.today.clone());
            }
        }
        while self.schedule_queue.size() > 0 {
            let (eventname, accountname, target, model) = self.schedule_queue.remove().unwrap();
            // println!(
            //     "[qamarket event] schedule {} - {} - {}",
            //     self.today, eventname, accountname
            // );
            match eventname.as_ref() {
                "target_amount" => self.target(&accountname, target, slice, &model),
                "target_weight" => self.target_weight(&accountname, target, slice, &model),
                _ => {
                    println!("{} is not supported", eventname)
                }
            };
        }
    }
    pub fn get_account(&mut self, account_name: &str) -> &mut QA_Account {
        self.reg_account.get_mut(account_name).unwrap()
    }

    pub fn run(&mut self, start: &str, end: &str) {
        self.on_initmarket();
        for date in QA_util_datelist_to_string(QATradeDate::new().get_index(start, end)) {
            println!("date: {}", date);
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
        target_panel: HashMap<String, f64>,
        slice: &HashMap<String, Kline>,
        label: &str,
    ) {
        match label {
            "next_open" | "nextopen" => {
                self.schedule_queue.add((
                    "target_amount".to_string(),
                    account_name.to_string(),
                    target_panel,
                    "open".to_string()
                ));
            }
            | "nextclose" | "next_close" => {
                self.schedule_queue.add((
                    "target_amount".to_string(),
                    account_name.to_string(),
                    target_panel,
                    "close".to_string()
                ));
            }
            _ => {
                if self.reg_account.contains_key(account_name) {
                    let today = self.today.clone();
                    let acc = self.get_account(account_name);
                    acc.change_datetime(today);

                    acc.target(target_panel, slice, label);
                }
            }
        }
    }
    #[allow(unused_must_use)]
    pub fn target_weight(
        &mut self,
        account_name: &str,
        target_panel: HashMap<String, f64>,
        slice: &HashMap<String, Kline>,
        label: &str,
    ) {
        match label {
            "next_open" | "nextopen" => {
                self.schedule_queue.add((
                    "target_weight".to_string(),
                    account_name.to_string(),
                    target_panel,
                    "open".to_string()
                ));
            }
        |   "next_close"| "nextclose" =>{
                self.schedule_queue.add((
                    "target_weight".to_string(),
                    account_name.to_string(),
                    target_panel,
                    "close".to_string()
                ));
            }
            _ => {
                if self.reg_account.contains_key(account_name) {
                    let today = self.today.clone();
                    let acc = self.get_account(account_name);
                    acc.change_datetime(today);

                    acc.target_weight(target_panel, slice, label);
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
        self.register_account("x1", 1000000.0)
    }
    fn on_bar(&mut self, data: &HashMap<String, Kline>) {
        // println!("dd{:#?}",self.today);
        // println!("dd{:#?}", self.get_account("x1").get_account_info());
    }
}

#[pymethods]
impl QAMarket {
    #[new]
    fn new_from_py(portfolio_name: &str, path: &str) -> Self {
        Self {
            reg_account: HashMap::new(),
            portfolio_name: portfolio_name.to_string(),
            data: QAMarketCenter::new(path),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string(),
            schedule_queue: queue![],
            cache: HashMap::new(),
        }
    }

    #[pyo3(name = "reinit")]
    pub fn py_reinit(&mut self, portfolio_name: &str) {
        self.reg_account = HashMap::new();
        self.portfolio_name = portfolio_name.to_string();
        self.market_event = DataFrame::default();
        self.today = "1970-01-01".to_string();
        self.schedule_queue = queue![];
        self.cache = HashMap::new();
    }

    #[pyo3(name = "run")]
    pub fn run_py(&mut self, start: &str, end: &str) {
        self.run(start, end)
    }

    #[pyo3(name = "init_market")]
    pub fn init_market(&mut self, account_list: HashMap<String, f64>) {
        for acc in account_list.keys() {
            self.register_account(acc, account_list[acc]);
            self.cache.insert(acc.to_string(), Vec::new());
        }
        self.cache.insert(self.portfolio_name.clone(), Vec::new());
    }
    pub fn init(&mut self) {}

    pub fn on_session_end(&mut self) {}

    #[pyo3(name = "set_date")]
    pub fn set_date_py(&mut self, date: &str) {
        self.set_date(date)
    }
    #[pyo3(name = "target_amount")]
    pub fn target_amount_py(
        &mut self,
        target_panels: HashMap<String, HashMap<String, f64>>,
        label: &str,
    ) {
        let slice = self.data.get_date(&self.today);

        for account_name in target_panels.keys() {
            if self.reg_account.contains_key(account_name) {
                self.target(
                    account_name,
                    target_panels.get(account_name).unwrap().clone(),
                    &slice,
                    label,
                )
            }
        }
    }
    #[pyo3(name = "target_weight")]
    pub fn target_weight_py(
        &mut self,
        target_panels: HashMap<String, HashMap<String, f64>>,
        label: &str,
    ) {
        let slice = self.data.get_date(&self.today);

        for account_name in target_panels.keys() {
            if self.reg_account.contains_key(account_name) {
                self.target_weight(
                    account_name,
                    target_panels.get(account_name).unwrap().clone(),
                    &slice,
                    label,
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

    #[pyo3(name = "settle")]
    pub fn py_settle(&mut self) -> PyResult<String> {
        let slice = self.data.get_date_ref(&self.today);
        //println!("[qamarket] event settle {:#?}", self.today);
        let mut result = HashMap::new();
        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                acc.on_price_change(
                    code.clone(),
                    slice.get(&code).unwrap().close,
                    self.today.clone(),
                )
            }
            let qifi_slice = acc.get_qifi_slice();
            let cache_acc = self.cache.get_mut(&qifi_slice.account_cookie).unwrap();
            cache_acc.push(qifi_slice.clone());

            result.insert(acc.account_cookie.clone(),qifi_slice);
            acc.settle();
        }
        let portqifi =
            QA_Portfolio::new(self.portfolio_name.clone(), result.clone()).get_portfolio_qifi();
        
        let cache_port = self.cache.get_mut(&self.portfolio_name).unwrap();
        cache_port.push(portqifi.clone());
        result.insert(self.portfolio_name.clone(), portqifi);

        let v = serde_json::to_string(&result).unwrap();
        Ok(v)
    }

    #[pyo3(name = "get_mkt_slice")]
    pub fn py_get_mkt_slice(&mut self) -> PyResult<String> {
        let slice = self.data.get_date_ref(&self.today);
        let v = serde_json::to_string(slice).unwrap();
        Ok(v)
    }

    #[pyo3(name = "get_qifis")]
    pub fn py_get_qifis_slice(&mut self) -> PyResult<String> {
        let qifislice: HashMap<String, QIFI> = self
            .reg_account
            .par_iter_mut()
            .map(|(accname, acc)| (accname.clone(), acc.get_qifi_slice()))
            .collect::<HashMap<String, QIFI>>();
        let v = serde_json::to_string(&qifislice).unwrap();
        Ok(v)
    }

    #[pyo3(name = "get_code")]
    pub fn py_get_code(&mut self, code: &str) -> PyResult<String> {
        let slice =
            serde_json::to_string(self.data.get_date_ref(&self.today).get(code).unwrap()).unwrap();
        Ok(slice)
    }

    #[pyo3(name = "before_trade")]
    pub fn py_before_trade(&mut self) {
        let slice = self.data.get_date(&self.today);
        self.before_trade(&slice);
    }

    #[pyo3(name = "get_portfolio")]
    pub fn py_get_portfolio(&mut self) -> PyResult<String> {
        let qifislice: HashMap<String, QIFI> = self
            .reg_account
            .par_iter_mut()
            .map(|(accname, acc)| (accname.clone(), acc.get_qifi_slice()))
            .collect::<HashMap<String, QIFI>>();
        let portqifi =
            QA_Portfolio::new(self.portfolio_name.clone(), qifislice).get_portfolio_qifi();
        let v = serde_json::to_string(&portqifi).unwrap();
        Ok(v)
    }

    #[pyo3(name = "get_tradepair")]
    pub fn py_get_tradepair(&mut self, account_name: &str) -> PyResult<String> {
        if self.reg_account.contains_key(account_name) {

            let tradeanalysis = QIFIAnalysis::new(account_name.to_string(), self.cache.get(account_name).unwrap().clone());
            let tradepair = tradeanalysis.get_tradepair();
            Ok(serde_json::to_string(&tradepair).unwrap())
        } else {
            Err(PyErr::new::<PyTypeError, _>("Error message"))
        }
    }
}
