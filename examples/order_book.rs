use qa_rs::qamarket::engine::{domain::OrderSide, orderbook::Orderbook, orders};
use std::time::SystemTime;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
    YUT,
}

fn parse_asset(asset: &str) -> Option<BrokerAsset> {
    match asset {
        "YUT" => Some(BrokerAsset::YUT),

        _ => None,
    }
}

fn main() {
    let mut orderbook = Orderbook::new(BrokerAsset::YUT);
    let order_book_id = parse_asset("YUT").unwrap();

    // create order requests
    let order_list = vec![
        orders::new_limit_order_request(
            order_book_id,
            OrderSide::Bid,
            0.98,
            5.0,
            SystemTime::now(),
        ),
        orders::new_limit_order_request(
            order_book_id,
            OrderSide::Ask,
            1.02,
            1.0,
            SystemTime::now(),
        ),
        orders::amend_order_request(1, OrderSide::Bid, 0.99, 4.0, SystemTime::now()),
        orders::new_limit_order_request(
            order_book_id,
            OrderSide::Bid,
            1.01,
            0.4,
            SystemTime::now(),
        ),
        orders::new_limit_order_request(
            order_book_id,
            OrderSide::Ask,
            1.03,
            0.5,
            SystemTime::now(),
        ),
        orders::new_market_order_request(order_book_id, OrderSide::Bid, 1.0, SystemTime::now()),
        orders::new_limit_order_request(
            order_book_id,
            OrderSide::Ask,
            1.05,
            0.5,
            SystemTime::now(),
        ),
        orders::limit_order_cancel_request(4, OrderSide::Ask),
        orders::new_limit_order_request(
            order_book_id,
            OrderSide::Bid,
            1.06,
            0.6,
            SystemTime::now(),
        ),
    ];

    // processing
    for order in order_list {
        println!("Order => {:?}", &order);
        let res = orderbook.process_order(order);
        println!("Processing => {:?}", res);
        orderbook.plot_orderbook();
        if let Some((bid, ask)) = orderbook.current_spread() {
            println!("Spread => bid: {}, ask: {}\n", bid, ask);

        
        } else {
            println!("Spread => not available\n");
        }
    }

        
}
