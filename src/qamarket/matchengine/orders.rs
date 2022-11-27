use std::fmt::Debug;
use std::time::SystemTime;

use super::domain::OrderDirection;

#[derive(Debug)]
pub enum OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    NewMarketOrder {
        order_book_id: Asset,
        direction: OrderDirection,
        volume: f64,
        ts: SystemTime,
    },

    NewLimitOrder {
        order_book_id: Asset,
        direction: OrderDirection,
        price: f64,
        volume: f64,
        ts: SystemTime,
    },

    AmendOrder {
        id: u64,
        direction: OrderDirection,
        price: f64,
        volume: f64,
        ts: SystemTime,
    },

    CancelOrder {
        id: u64,
        direction: OrderDirection,
        //ts: SystemTime,
    },
}

/* Constructors */

/// Create request for the new market order
pub fn new_market_order_request<Asset>(
    order_book_id: Asset,

    direction: OrderDirection,
    volume: f64,
    ts: SystemTime,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::NewMarketOrder {
        order_book_id,

        volume,
        direction,
        ts,
    }
}

/// Create request for the new limit order
pub fn new_limit_order_request<Asset>(
    order_book_id: Asset,

    direction: OrderDirection,
    price: f64,
    volume: f64,
    ts: SystemTime,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::NewLimitOrder {
        order_book_id,
        direction,
        price,
        volume,
        ts,
    }
}

/// Create request for changing price/volume for the active limit order.
///
/// Note: do not change order direction!
/// Instead cancel existing order and create a new one.
pub fn amend_order_request<Asset>(
    id: u64,
    direction: OrderDirection,
    price: f64,
    volume: f64,
    ts: SystemTime,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::AmendOrder {
        id,
        direction,
        price,
        volume,
        ts,
    }
}

/// Create request for cancelling active limit order
pub fn limit_order_cancel_request<Asset>(
    order_id: u64,
    direction: OrderDirection,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::CancelOrder {
        id: order_id,
        direction,
    }
}
