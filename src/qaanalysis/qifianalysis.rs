use crate::qaaccount::qaperformance::{QAPerformance, QATradePair};
use crate::qaprotocol::qifi::account::QIFI;
use polars::prelude::*;
pub struct QIFIAnalysis {
    pub account_cookie: String,
    pub qifi_history: Vec<QIFI>,
}

impl QIFIAnalysis {
    pub fn new(account_cookie: String, qifi_history: Vec<QIFI>) -> Self {
        QIFIAnalysis {
            account_cookie,
            qifi_history,
        }
    }

    pub fn get_account_curve(&self) -> DataFrame {
        let mut account_curve = Vec::new();
        let mut time_index = Vec::new();
        for qifi in &self.qifi_history {
            account_curve.push(qifi.accounts.balance);
            time_index.push(qifi.trading_day.clone());
        }
        let s1 = Series::new("account", &account_curve);
        let s2 = Series::new("date", &time_index);

        let df: DataFrame = DataFrame::new(vec![s1, s2])
            .unwrap()
            .sort(["date"], false)
            .unwrap();
        df
    }

    pub fn get_tradepair(&self) -> Vec<QATradePair> {
        let mut perf = QAPerformance::new();
        self.qifi_history.iter().for_each(|qifi| {
            qifi.trades.iter().for_each(|(_, trade)| {
                perf.insert_trade(trade.clone());
            });
        });
        perf.pair()
    }
}
