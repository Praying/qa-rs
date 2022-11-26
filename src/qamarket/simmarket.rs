use crate::qaaccount::account::QA_Account;
use crate::qaaccount::order::QAOrder;
use crate::qamarket::order_book::OrderBook;
/// realtime market with lots of account reg
///
///
///
use chrono::Local;
use hashbrown::HashMap;

/// setting :: simmarket rules:
/// rules trading for 24h/ every time// no tradegap
///
pub struct QASIMMarket {
    pub reg_account: HashMap<String, QA_Account>,
    pub OrderBook: HashMap<String, OrderBook>,
    pub orders: HashMap<String, QAOrder>,
    time: String,
    portfolio_name: String,
    pub trading_day: String,
    pub marketpanel: HashMap<String, f64>,
}

impl QASIMMarket {
    pub fn new() -> Self {
        QASIMMarket {
            reg_account: HashMap::new(),
            orders: HashMap::new(),
            OrderBook: HashMap::new(),
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
        let account = self.reg_account.get_mut(account_cookie).unwrap();
        let towards = account.get_towards(direction, offset);
        let result = account.send_order(code, amount, &self.time, towards, price, "");
        match result {
            Ok(order) => {
                let single_order_book = self.OrderBook.get_mut(code).unwrap();
                let result = single_order_book.insert_order(&order);
                self.orders.insert(order.order_id.clone(), order);
            }
            Err(e) => {}
        }
    }
}
