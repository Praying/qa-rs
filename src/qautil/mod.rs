pub mod predict;
pub mod tradedate;

use chrono::TimeZone;
use chrono::Utc;

pub fn get_qadatestamp() -> i64 {
    // qainside && qifi protocol both use utc[+0] as timestamp

    let now = Utc::now();
    let timestamp = now.timestamp_nanos();
    timestamp
}

pub fn parse_datestamp(ts: i64) -> String {
    // utc[+0] => utc[+8]
    let dt = Utc.timestamp_nanos(ts + 28800000000000);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn parse_fromstr_datestamp(datetime: String) -> i64 {
    match datetime.len() {
        19 => {
            Utc.datetime_from_str(&datetime, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .timestamp_nanos()
                - 28800000000000
        }
        10 => {
            let dt = format!("{} 00:00:00", datetime);
            Utc.datetime_from_str(&dt, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .timestamp_nanos()
                - 28800000000000
        }
        _ => 0,
    }
}
