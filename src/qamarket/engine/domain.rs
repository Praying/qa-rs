use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum OrderDirection {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]
pub struct Order<Asset>
where
    Asset: Debug + Clone,
{
    pub order_id: u64,
    pub order_book_id: Asset,
    pub direction: OrderDirection,
    pub price: f64,
    pub volume: f64,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
}
