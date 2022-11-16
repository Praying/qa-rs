use chrono::NaiveDate;
use hashbrown::HashMap;
use itertools::izip;
use polars::prelude::*;
use std::fs::File;

#[warn(non_camel_case_types)]
pub struct QAMarketCenter {
    pub dateidx: i32,
    pub date: String,
    pub data: HashMap<i32, HashMap<String, Kline>>,
    pub today: HashMap<String, Kline>,
}
impl QAMarketCenter {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).expect("Cannot open file.");
        let df = ParquetReader::new(&mut file).finish().unwrap();

        Self {
            dateidx: 0,
            date: "".to_string(),
            data: df
                .partition_by(vec!["date"])
                .unwrap()
                .iter()
                .map(|df| {
                    let (k, v) = run_t(df.clone());
                    (k, v)
                })
                .collect::<HashMap<i32, HashMap<String, Kline>>>(),
            today: HashMap::new(),
        }
    }
    pub fn get_date(&mut self, date: &str) -> &HashMap<String, Kline> {
        self.date = date.to_string();
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .unwrap()
            .and_hms(0, 0, 0)
            .timestamp()
            * 1_000_000;
        self.dateidx = (date / 86400_000_000) as i32;
        self.data.get(&self.dateidx).unwrap()
    }
}
#[derive(Debug)]
pub struct Kline {
    pub order_book_id: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub total_turnover: f64,
    pub split_coefficient_to: f64,
    pub dividend_cash_before_tax: f64,
}

// ['order_book_id',
//  'date',
//  'num_trades',
//  'limit_up',
//  'limit_down',
//  'open',
//  'high',
//  'low',
//  'close',
//  'volume',
//  'total_turnover',
//  'split_coefficient_to',
//  'dividend_cash_before_tax']

pub fn run_t(df: DataFrame) -> (i32, HashMap<String, Kline>) {
    let opens = df["open"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let closes = df["close"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let highs = df["high"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let lows = df["low"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let volumes = df["volume"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let total_turnover = df["total_turnover"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let codes = df["order_book_id"]
        .utf8()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap().to_owned())
        .collect::<Vec<_>>();
    let limitups = df["limit_up"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap().to_owned())
        .collect::<Vec<_>>();
    let limitdowns = df["limit_down"]
        .f64()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap().to_owned())
        .collect::<Vec<_>>();
    let split_coefficient_tos = df["split_coefficient_to"]
        .f64()
        .unwrap()
        .fill_null( FillNullStrategy::Zero).unwrap()
        .into_iter()
        .map(|x| x.unwrap().to_owned())
        .collect::<Vec<_>>();
    let dividend_cash_before_taxs = df["dividend_cash_before_tax"]
        .f64()
        .unwrap()
        .fill_null( FillNullStrategy::Zero).unwrap()
        .into_iter()
        .map(|x| x.unwrap().to_owned())
        .collect::<Vec<_>>();
    let dates = df["date"].date().unwrap().get(0).unwrap();

    let mut map = HashMap::new();
    for (
        code,
        open,
        close,
        high,
        low,
        volume,
        op,
        lu,
        ld,
        split_coefficient_to,
        dividend_cash_before_tax,
    ) in izip!(
        codes,
        opens,
        closes,
        highs,
        lows,
        volumes,
        total_turnover,
        limitups,
        limitdowns,
        split_coefficient_tos,
        dividend_cash_before_taxs
    ) {
        let seq = Kline {
            order_book_id: code.clone(),
            open: open,
            close: close,
            high: high,
            low: low,
            volume: volume,
            total_turnover: op,
            limit_down: ld,
            limit_up: lu,
            split_coefficient_to: split_coefficient_to,
            dividend_cash_before_tax: dividend_cash_before_tax,
        };
        map.insert(code, seq);
    }
    (dates, map)
}
