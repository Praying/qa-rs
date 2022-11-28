pub mod predict;
pub mod tradedate;

use chrono::{Utc, Local};
use chrono::TimeZone;

pub fn get_qadatestamp() -> i64{
    // qainside && qifi protocol both use utc[+0] as timestamp

    let now = Utc::now();
    let timestamp = now.timestamp_nanos();
    timestamp
}

pub fn parse_datestamp(ts:i64) -> String{

    // utc[+0] => utc[+8]
    let dt = Utc.timestamp_nanos(ts+28800000000000);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}