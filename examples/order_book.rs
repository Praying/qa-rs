use chrono::Utc;
use qa_rs::qamarket::matchengine::{domain::OrderDirection, orderbook::Orderbook, orders};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
    YUT,
}
// 此处目的主要是实现注册功能, 避免乱七八糟的报单
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
            OrderDirection::BUY,
            0.98,
            5.0,
            Utc::now().timestamp_nanos(),
        ),
        orders::new_limit_order_request(
            order_book_id,
            OrderDirection::SELL,
            1.02,
            1.0,
            Utc::now().timestamp_nanos(),
        ),
        orders::amend_order_request(
            1,
            OrderDirection::BUY,
            0.99,
            4.0,
            Utc::now().timestamp_nanos(),
        ),
        orders::new_limit_order_request(
            order_book_id,
            OrderDirection::BUY,
            1.01,
            0.4,
            Utc::now().timestamp_nanos(),
        ),
        orders::new_limit_order_request(
            order_book_id,
            OrderDirection::SELL,
            1.03,
            0.5,
            Utc::now().timestamp_nanos(),
        ),
        orders::new_market_order_request(
            order_book_id,
            OrderDirection::BUY,
            1.0,
            Utc::now().timestamp_nanos(),
        ),
        orders::new_limit_order_request(
            order_book_id,
            OrderDirection::SELL,
            1.05,
            0.5,
            Utc::now().timestamp_nanos(),
        ),
        //orders::limit_order_cancel_request(4, OrderDirection::SELL),
        orders::new_limit_order_request(
            order_book_id,
            OrderDirection::BUY,
            1.06,
            0.6,
            Utc::now().timestamp_nanos(),
        ),
    ];

    // processing
    for order in order_list {
        println!("Order => {:?}", &order);
        let res = orderbook.process_order(order);
        println!("Processing => {:?}", res);
        orderbook.plot_orderbook();
        if let Some((buy, sell)) = orderbook.current_spread() {
            println!("Spread => buy: {}, sell: {}\n", buy, sell);
        } else {
            println!("Spread => not available\n");
        }
    }
}
