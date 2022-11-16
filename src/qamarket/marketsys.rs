use crate::qaaccount::account::QA_Account;

use crate::qadata::marketcenter::{Kline, QAMarketCenter};
use crate::qaenv::localenv::CONFIG;
use crate::qautil::tradedate::{QATradeDate, QA_util_datelist_to_string};
use hashbrown::HashMap;
use polars::prelude::*;
pub struct QAMarket {
    pub reg_account: HashMap<String, QA_Account>,
    pub data: QAMarketCenter,
    pub market_event: DataFrame,
    pub today: String
}

impl QAMarket {
    pub fn new() -> Self {
        Self {
            reg_account: HashMap::new(),
            data: QAMarketCenter::new(&CONFIG.DataPath.cache),
            market_event: DataFrame::default(),
            today: "1970-01-01".to_string()
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
        for date in QA_util_datelist_to_string(QATradeDate::new().get_index(start, end)) {
            println!("date: {}", date);
            self.today = date.clone();
            self.singleday(&date);
        }
    }
    pub fn target(&mut self, account_name: &str, target_map: &HashMap<String, f64>){
        if self.reg_account.contains_key(account_name){
            let acc = self.get_account(account_name);
            
        } 
    }

    pub fn singleday(&mut self, date: &str) {
        let slice = self.data.get_date(date);

        // before trading
        for acc in self.reg_account.values_mut() {
            for code in acc.get_code_subscribed() {
                let bar = slice.get(&code).unwrap();
                if bar.dividend_cash_before_tax>0.0{
                    acc.dividend_event(&code, bar.dividend_cash_before_tax)
                }
                if bar.split_coefficient_to>0.0{
                    acc.split_event(&code, bar.split_coefficient_to)
                }
            }
        }

        //on bar

        for acc in self.reg_account.values_mut(){
            
            for code in acc.get_code_subscribed(){
                acc.on_price_change(code.clone(), slice.get(&code).unwrap().close, self.today.clone())
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
