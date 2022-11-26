use std::f32::consts::E;

use crate::qaaccount::order::QAOrder;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderBook {
    pub order_book_id: String,
    pub ask_1: f64,
    pub ask_2: f64,
    pub ask_3: f64,
    pub ask_4: f64,
    pub ask_5: f64,
    pub ask_volume1: f64,
    pub ask_volume2: f64,
    pub ask_volume3: f64,
    pub ask_volume4: f64,
    pub ask_volume5: f64,

    pub bid_1: f64,
    pub bid_2: f64,
    pub bid_3: f64,
    pub bid_4: f64,
    pub bid_5: f64,
    pub bid_volume1: f64,
    pub bid_volume2: f64,
    pub bid_volume3: f64,
    pub bid_volume4: f64,
    pub bid_volume5: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub total_turnover: f64,
}

impl OrderBook {
    pub fn new(order_book_id: &str) -> Self {
        OrderBook {
            order_book_id: order_book_id.to_string(),
            ask_1: 0.0,

            ask_volume1: 0.0,
            bid_1: 0.0,
            bid_volume1: 0.0,
            ask_2: 0.0,
            ask_3: 0.0,
            ask_4: 0.0,
            ask_5: 0.0,
            ask_volume2: 0.0,
            ask_volume3: 0.0,
            ask_volume4: 0.0,
            ask_volume5: 0.0,

            bid_2: 0.0,
            bid_3: 0.0,
            bid_4: 0.0,
            bid_5: 0.0,

            bid_volume2: 0.0,
            bid_volume3: 0.0,
            bid_volume4: 0.0,
            bid_volume5: 0.0,
            limit_up: 0.0,
            limit_down: 0.0,
            open: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
            volume: 0.0,
            total_turnover: 0.0,
        }
    }

    pub fn insert_order(&mut self, order: &QAOrder) -> Result<(), String> {
        // order insert event
        // 1. check if the order is valid
        // 2. if deal, update the orderbook
        // 3. if not deal, update the orderbook
        // 4. return the order status

        let limitprice = order.price;
        let left_volume = order.volume_orign;

        if order.direction == "BUY" {
            if limitprice >= self.ask_1 {
                if left_volume >= self.ask_volume1 {
                    // deal
                    self.ask_volume1 = 0.0;
                    self.ask_1 = self.ask_2;
                    self.ask_volume2 = self.ask_volume2 - left_volume;
                    self.ask_2 = self.ask_3;
                    self.ask_volume3 = self.ask_volume3 - left_volume;
                    self.ask_3 = self.ask_4;
                    self.ask_volume4 = self.ask_volume4 - left_volume;
                    self.ask_4 = self.ask_5;
                    self.ask_volume5 = self.ask_volume5 - left_volume;
                    self.ask_5 = 0.0;
                    self.ask_volume5 = 0.0;
                } else {
                }
            } else {
            }
        } else {
        }

        Ok(())
    }
}
