use sled::{Db,IVec};
use crate::qadata::marketcenter::Kline;



#[cfg(test)]



mod test_sled{
    use super::*;
    #[test]
    fn test_sled(){
        let db = Db::open("my_db").unwrap();
        let tree = db.open_tree("my_tree").unwrap();



        let k = Kline{
            order_book_id: "IX".to_string(),
            open: 1.0,
            close: 1.0,
            high: 1.0,
            low: 1.0,
            volume: 1.0,
            limit_up: 1.0,
            limit_down: 1.0,
            total_turnover: 1.0,
            split_coefficient_to: 1.0,
            dividend_cash_before_tax: 0.0,
        };

        tree.insert("key", IVec::from(k.clone())).unwrap();
        assert_eq!(tree.get("key").unwrap(), Some(k));
    }
}