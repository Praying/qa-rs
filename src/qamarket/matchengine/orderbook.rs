use std::fmt::Debug;
use chrono::Utc;
use super::domain::{Order, OrderDirection, OrderType};
use super::order_queues::OrderQueue;
use super::orders::OrderRequest;
use super::sequence;
use super::validation::OrderRequestValidator;

const MIN_SEQUENCE_ID: u64 = 1;
const MAX_SEQUENCE_ID: u64 = 1000;
const MAX_STALLED_INDICES_IN_QUEUE: u64 = 10;
const ORDER_QUEUE_INIT_CAPACITY: usize = 500;

pub type OrderProcessingResult = Vec<Result<Success, Failed>>;

#[derive(Debug)]
pub enum Success {
    Accepted {
        id: u64,
        order_type: OrderType,
        ts: i64,
    },

    Filled {
        order_id: u64,
        direction: OrderDirection,
        order_type: OrderType,
        price: f64,
        volume: f64,
        ts: i64,
    },

    PartiallyFilled {
        order_id: u64,
        direction: OrderDirection,
        order_type: OrderType,
        price: f64,
        volume: f64,
        ts: i64,
    },

    Amended {
        id: u64,
        price: f64,
        volume: f64,
        ts: i64,
    },

    Cancelled {
        id: u64,
        ts: i64,
    },
}

#[derive(Debug)]
pub enum Failed {
    ValidationFailed(String),
    DuplicateOrderID(u64),
    NoMatch(u64),
    OrderNotFound(u64),
}

pub struct Orderbook<Asset>
where
    Asset: Debug + Clone + Copy + Eq,
{
    order_book_id: Asset,
    bid_queue: OrderQueue<Order<Asset>>,
    ask_queue: OrderQueue<Order<Asset>>,
    seq: sequence::TradeSequence,
    order_validator: OrderRequestValidator<Asset>,
}

impl<Asset> Orderbook<Asset>
where
    Asset: Debug + Clone + Copy + Eq,
{
    /// Create new orderbook for pair of assets
    ///
    /// # Examples
    ///
    /// Basic usage:
    // todo fix doc test!
    pub fn new(order_book_id: Asset) -> Self {
        Orderbook {
            order_book_id,
            bid_queue: OrderQueue::new(
                OrderDirection::BUY,
                MAX_STALLED_INDICES_IN_QUEUE,
                ORDER_QUEUE_INIT_CAPACITY,
            ),
            ask_queue: OrderQueue::new(
                OrderDirection::SELL,
                MAX_STALLED_INDICES_IN_QUEUE,
                ORDER_QUEUE_INIT_CAPACITY,
            ),
            seq: sequence::new_sequence_gen(MIN_SEQUENCE_ID, MAX_SEQUENCE_ID),
            order_validator: OrderRequestValidator::new(
                order_book_id,
                MIN_SEQUENCE_ID,
                MAX_SEQUENCE_ID,
            ),
        }
    }

    pub fn process_order(&mut self, order: OrderRequest<Asset>) -> OrderProcessingResult {
        // processing result accumulator
        let mut proc_result: OrderProcessingResult = vec![];

        // validate request
        if let Err(reason) = self.order_validator.validate(&order) {
            proc_result.push(Err(Failed::ValidationFailed(String::from(reason))));
            return proc_result;
        }

        match order {
            OrderRequest::NewMarketOrder {
                order_book_id,
                direction,
                volume,
                ts: _ts,
            } => {
                // generate new ID for order
                let order_id = self.seq.next_id();
                proc_result.push(Ok(Success::Accepted {
                    id: order_id,
                    order_type: OrderType::Market,
                    ts: Utc::now().timestamp_nanos(),
                }));

                self.process_market_order(
                    &mut proc_result,
                    order_id,
                    order_book_id,
                    direction,
                    volume,
                );
            }

            OrderRequest::NewLimitOrder {
                order_book_id,
                direction,
                price,
                volume,
                ts,
            } => {
                let order_id = self.seq.next_id();
                proc_result.push(Ok(Success::Accepted {
                    id: order_id,
                    order_type: OrderType::Limit,
                    ts: Utc::now().timestamp_nanos(),
                }));

                self.process_limit_order(
                    &mut proc_result,
                    order_id,
                    order_book_id,
                    direction,
                    price,
                    volume,
                    ts,
                );
            }

            OrderRequest::AmendOrder {
                id,
                direction,
                price,
                volume,
                ts,
            } => {
                self.process_order_amend(&mut proc_result, id, direction, price, volume, ts);
            }

            OrderRequest::CancelOrder { id, direction } => {
                self.process_order_cancel(&mut proc_result, id, direction);
            }
        }

        // return collected processing results
        proc_result
    }

    /// Get current spread as a tuple: (buy, sell)
    pub fn current_spread(&mut self) -> Option<(f64, f64)> {
        let buy = self.bid_queue.peek()?.price;
        let sell = self.ask_queue.peek()?.price;
        Some((buy, sell))
    }
    pub fn plot_orderbook(&mut self) {
        let buy = self.bid_queue.peek();
        let sell = self.ask_queue.peek();
        match (buy, sell) {
            (Some(buy), Some(sell)) => {
                println!("----- orderbook: {:?} -----\n sell: {:?}  |  {:#?} \n---------------------\nbid: {:?}  |  {:#?} \n", self.order_book_id, sell.price,  sell.volume, buy.price,  buy.volume);
            }
            _ => println!("no buy or sell"),
        }
    }
    /* Processing logic */

    fn process_market_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        order_book_id: Asset,
        direction: OrderDirection,
        volume: f64,
    ) {
        // get copy of the current limit order
        let opposite_order_result = {
            let opposite_queue = match direction {
                OrderDirection::BUY => &mut self.ask_queue,
                OrderDirection::SELL => &mut self.bid_queue,
            };
            opposite_queue.peek().cloned()
        };

        if let Some(opposite_order) = opposite_order_result {
            let matching_complete = self.order_matching(
                results,
                &opposite_order,
                order_id,
                order_book_id,
                OrderType::Market,
                direction,
                volume,
            );

            if !matching_complete {
                // match the rest
                self.process_market_order(
                    results,
                    order_id,
                    order_book_id,
                    direction,
                    volume - opposite_order.volume,
                );
            }
        } else {
            // no limit orders found
            results.push(Err(Failed::NoMatch(order_id)));
        }
    }

    fn process_limit_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        order_book_id: Asset,
        direction: OrderDirection,
        price: f64,
        volume: f64,
        ts: i64,
    ) {
        // take a look at current opposite limit order
        let opposite_order_result = {
            let opposite_queue = match direction {
                OrderDirection::BUY => &mut self.ask_queue,
                OrderDirection::SELL => &mut self.bid_queue,
            };
            opposite_queue.peek().cloned()
        };

        if let Some(opposite_order) = opposite_order_result {
            let could_be_matched = match direction {
                // verify buy/sell price overlap
                OrderDirection::BUY => price >= opposite_order.price,
                OrderDirection::SELL => price <= opposite_order.price,
            };

            if could_be_matched {
                // match immediately
                let matching_complete = self.order_matching(
                    results,
                    &opposite_order,
                    order_id,
                    order_book_id,
                    OrderType::Limit,
                    direction,
                    volume,
                );

                if !matching_complete {
                    // process the rest of new limit order
                    self.process_limit_order(
                        results,
                        order_id,
                        order_book_id,
                        direction,
                        price,
                        volume - opposite_order.volume,
                        ts,
                    );
                }
            } else {
                // just insert new order in queue
                self.store_new_limit_order(
                    results,
                    order_id,
                    order_book_id,
                    direction,
                    price,
                    volume,
                    ts,
                );
            }
        } else {
            self.store_new_limit_order(
                results,
                order_id,
                order_book_id,
                direction,
                price,
                volume,
                ts,
            );
        }
    }

    fn process_order_amend(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        direction: OrderDirection,
        price: f64,
        volume: f64,
        ts: i64,
    ) {
        let order_queue = match direction {
            OrderDirection::BUY => &mut self.bid_queue,
            OrderDirection::SELL => &mut self.ask_queue,
        };

        if order_queue.amend(
            order_id,
            price,
            ts,
            Order {
                order_id,
                order_book_id: self.order_book_id,
                direction,
                price,
                volume,
            },
        ) {
            results.push(Ok(Success::Amended {
                id: order_id,
                price,
                volume,
                ts: Utc::now().timestamp_nanos(),
            }));
        } else {
            results.push(Err(Failed::OrderNotFound(order_id)));
        }
    }

    fn process_order_cancel(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        direction: OrderDirection,
    ) {
        let order_queue = match direction {
            OrderDirection::BUY => &mut self.bid_queue,
            OrderDirection::SELL => &mut self.ask_queue,
        };

        if order_queue.cancel(order_id) {
            results.push(Ok(Success::Cancelled {
                id: order_id,
                ts: Utc::now().timestamp_nanos(),
            }));
        } else {
            results.push(Err(Failed::OrderNotFound(order_id)));
        }
    }

    /* Helpers */

    fn store_new_limit_order(
        &mut self,
        results: &mut OrderProcessingResult,
        order_id: u64,
        order_book_id: Asset,
        direction: OrderDirection,
        price: f64,
        volume: f64,
        ts: i64,
    ) {
        let order_queue = match direction {
            OrderDirection::BUY => &mut self.bid_queue,
            OrderDirection::SELL => &mut self.ask_queue,
        };
        if !order_queue.insert(
            order_id,
            price,
            ts,
            Order {
                order_id,
                order_book_id,
                direction,
                price,
                volume,
            },
        ) {
            results.push(Err(Failed::DuplicateOrderID(order_id)))
        };
    }

    fn order_matching(
        &mut self,
        results: &mut OrderProcessingResult,
        opposite_order: &Order<Asset>,
        order_id: u64,
        order_book_id: Asset,
        order_type: OrderType,
        direction: OrderDirection,
        volume: f64,
    ) -> bool {
        // real processing time
        let deal_time = Utc::now().timestamp_nanos();

        // match immediately
        if volume < opposite_order.volume {
            // fill new limit and modify opposite limit

            // report filled new order
            results.push(Ok(Success::Filled {
                order_id,
                direction,
                order_type,
                price: opposite_order.price,
                volume,
                ts: deal_time,
            }));

            // report partially filled opposite limit order
            results.push(Ok(Success::PartiallyFilled {
                order_id: opposite_order.order_id,
                direction: opposite_order.direction,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                volume,
                ts: deal_time,
            }));

            // modify unmatched part of the opposite limit order
            {
                let opposite_queue = match direction {
                    OrderDirection::BUY => &mut self.ask_queue,
                    OrderDirection::SELL => &mut self.bid_queue,
                };
                opposite_queue.modify_current_order(Order {
                    order_id: opposite_order.order_id,
                    order_book_id,
                    direction: opposite_order.direction,
                    price: opposite_order.price,
                    volume: opposite_order.volume - volume,
                });
            }
        } else if volume > opposite_order.volume {
            // partially fill new limit order, fill opposite limit and notify to process the rest

            // report new order partially filled
            results.push(Ok(Success::PartiallyFilled {
                order_id,
                direction,
                order_type,
                price: opposite_order.price,
                volume: opposite_order.volume,
                ts: deal_time,
            }));

            // report filled opposite limit order
            results.push(Ok(Success::Filled {
                order_id: opposite_order.order_id,
                direction: opposite_order.direction,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                volume: opposite_order.volume,
                ts: deal_time,
            }));

            // remove filled limit order from the queue
            {
                let opposite_queue = match direction {
                    OrderDirection::BUY => &mut self.ask_queue,
                    OrderDirection::SELL => &mut self.bid_queue,
                };
                opposite_queue.pop();
            }

            // matching incomplete
            return false;
        } else {
            // orders exactly match -> fill both and remove old limit

            // report filled new order
            results.push(Ok(Success::Filled {
                order_id,
                direction,
                order_type,
                price: opposite_order.price,
                volume,
                ts: deal_time,
            }));
            // report filled opposite limit order
            results.push(Ok(Success::Filled {
                order_id: opposite_order.order_id,
                direction: opposite_order.direction,
                order_type: OrderType::Limit,
                price: opposite_order.price,
                volume,
                ts: deal_time,
            }));

            // remove filled limit order from the queue
            {
                let opposite_queue = match direction {
                    OrderDirection::BUY => &mut self.ask_queue,
                    OrderDirection::SELL => &mut self.bid_queue,
                };
                opposite_queue.pop();
            }
        }

        // complete matching
        true
    }
}

#[cfg(test)]
mod test {

    use super::super::orders;
    use super::*;

    #[derive(PartialEq, Eq, Debug, Copy, Clone)]
    pub enum Asset {
        USD,
        BTC,
    }

    #[test]
    fn cancel_nonexisting() {
        let mut orderbook = Orderbook::new(Asset::BTC);
        let request = orders::limit_order_cancel_request(1, OrderDirection::BUY);
        let mut result = orderbook.process_order(request);

        assert_eq!(result.len(), 1);
        match result.pop().unwrap() {
            Err(_) => (),
            _ => panic!("unexpected events"),
        }
    }
}
