use crate::qaaccount::account::QA_Account;
use hashbrown::{HashMap, HashSet};
use crate::qadata::localdata::QADataStructStockDay;
use crate::qautil::tradedate::{QA_util_datelist_to_string, QATradeDate};
use polars::prelude::*;

pub struct QAMarket {
    pub RegAccount:HashMap<String, QA_Account>,
    pub Data:HashMap<String, QADataStructStockDay>,
}

impl QAMarket {
    pub fn new() -> Self {
        Self {
            RegAccount: HashMap::new(),
            Data: HashMap::new(),
        }
    }

    pub fn register_account(&mut self, account_name: &str) {

        if self.RegAccount.contains_key(account_name) {
            println!("{} is already registered", account_name);
        } else {
            let acc = QA_Account::new(account_name, "portfolio", "admin", 10000000000000.0, false, "backtest");
            self.RegAccount.insert(account_name.to_string(), acc);
        }
        
    }
    pub fn get_account(&mut self, account_name: &str) -> &mut QA_Account {
        self.RegAccount.get_mut(account_name).unwrap()
    }



    pub fn run(&mut self, start: &str, end: &str) {
        for date in QA_util_datelist_to_string(QATradeDate::new().get_index(start, end)) {
            println!("date: {}", date);
            let data= QADataStructStockDay::load_singleday(&date);
            //self.Data.insert(date.clone(), data);

            println!("data: {:?}",data.data);
            self.run_single(data);
        }
    }


    pub fn run_single(&mut self, data: QADataStructStockDay) {

        let mut codesub = HashSet::new();
        for acc in self.RegAccount.values_mut() {
            for code in acc.get_codeSubscribed() {
                codesub.insert(code);
            }

        }
        let usedata = data.select(codesub.into_iter().collect());
        println!("usedata: {:?}", usedata);
    }
}

