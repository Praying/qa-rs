use chrono::{NaiveDate};
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
    pub fn new(data: HashMap<i32, HashMap<String, Kline>>) -> Self {
        QAMarketCenter {
            dateidx: 0,
            date: "".to_string(),
            data: data,
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
    pub total_turnover: f64,
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

fn run_t(df: DataFrame) -> (i32, HashMap<String, Kline>) {
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
    let dates = df["date"].date().unwrap().get(0).unwrap();

    let mut map = HashMap::new();
    for (code, open, close, high, low, volume, op) in
        izip!(codes, opens, closes, highs, lows, volumes, total_turnover)
    {
        let seq = Kline {
            order_book_id: code.clone(),
            open: open,

            close: close,
            high: high,
            low: low,
            volume: volume,
            total_turnover: op,
        };
        map.insert(code, seq);
    }
    (dates, map)
}
fn main() {
    let mut file = File::open("..\\data\\mkt.pq").expect("Cannot open file.");
    let df = ParquetReader::new(&mut file).finish().unwrap();
    println!("{:?}", df);
    use std::time::Instant;
    let start = Instant::now();

    let mut MC = QAMarketCenter::new(
        df.partition_by(vec!["date"])
            .unwrap()
            .iter()
            .map(|df| {
                let (k, v) = run_t(df.clone());
                (k, v)
            })
            .collect::<HashMap<i32, HashMap<String, Kline>>>(),
    );

    println!("partition time: {:?} ms", start.elapsed().as_millis());
    let start = Instant::now();
    // println!("{:#?}",MC.get_date("2022-10-10").get("000001.XSHE").unwrap());
    let somdata = MC.get_date("2022-10-10").get("000001.XSHE").unwrap();
    println!("partition time: {:?} ns", start.elapsed().as_nanos());
}
