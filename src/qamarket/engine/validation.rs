use std::fmt::Debug;

use super::orders::OrderRequest;

/// Validation errors
const ERR_BAD_order_book_id: &str = "bad order asset";
const ERR_BAD_PRICE_VALUE: &str = "price must be non-negative";
const ERR_BAD_QUANTITY_VALUE: &str = "quantity must be non-negative";
const ERR_BAD_SEQ_ID: &str = "order ID out of range";

/* Validators */

pub struct OrderRequestValidator<Asset> {
    orderbook_order_book_id: Asset,
    min_sequence_id: u64,
    max_sequence_id: u64,
}

impl<Asset> OrderRequestValidator<Asset>
where
    Asset: Debug + Clone + Copy + Eq,
{
    pub fn new(
        orderbook_order_book_id: Asset,
        min_sequence_id: u64,
        max_sequence_id: u64,
    ) -> Self {
        OrderRequestValidator {
            orderbook_order_book_id,
            min_sequence_id,
            max_sequence_id,
        }
    }

    pub fn validate(&self, request: &OrderRequest<Asset>) -> Result<(), &str> {
        match *request {
            OrderRequest::NewMarketOrder {
                order_book_id,
                direction: _direction,
                volume,
                ts: _ts,
            } => self.validate_market(order_book_id, volume),

            OrderRequest::NewLimitOrder {
                order_book_id,
                direction: _direction,
                price,
                volume,
                ts: _ts,
            } => self.validate_limit(order_book_id, price, volume),

            OrderRequest::AmendOrder {
                id,
                price,
                direction: _direction,
                volume,
                ts: _ts,
            } => self.validate_amend(id, price, volume),

            OrderRequest::CancelOrder { id, direction: _direction } => self.validate_cancel(id),
        }
    }

    /* Internal validators */

    fn validate_market(
        &self,
        order_book_id: Asset,
        volume: f64,
    ) -> Result<(), &str> {
        if self.orderbook_order_book_id != order_book_id {
            return Err(ERR_BAD_order_book_id);
        }


        if volume <= 0.0 {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }

    fn validate_limit(
        &self,
        order_book_id: Asset,

        price: f64,
        volume: f64,
    ) -> Result<(), &str> {
        if self.orderbook_order_book_id != order_book_id {
            return Err(ERR_BAD_order_book_id);
        }


        if price <= 0.0 {
            return Err(ERR_BAD_PRICE_VALUE);
        }

        if volume <= 0.0 {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }

    fn validate_amend(&self, id: u64, price: f64, volume: f64) -> Result<(), &str> {
        if self.min_sequence_id > id || self.max_sequence_id < id {
            return Err(ERR_BAD_SEQ_ID);
        }

        if price <= 0.0 {
            return Err(ERR_BAD_PRICE_VALUE);
        }

        if volume <= 0.0 {
            return Err(ERR_BAD_QUANTITY_VALUE);
        }

        Ok(())
    }

    fn validate_cancel(&self, id: u64) -> Result<(), &str> {
        if self.min_sequence_id > id || self.max_sequence_id < id {
            return Err(ERR_BAD_SEQ_ID);
        }

        Ok(())
    }
}
