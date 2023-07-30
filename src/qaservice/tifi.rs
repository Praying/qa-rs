use actix::prelude::*;
// trading protocol
use serde::{Deserialize, Serialize};
use serde_json::Value;
//
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct insert_order {
    pub aid: String,
    pub user_id: String,
    pub order_id: String,
    pub exchange_id: String,
    pub instrument_id: String,
    pub direction: String,
    pub offset: String,
    pub volume: f64,
    pub price_type: String,
    pub limit_price: f64,
    pub volume_condition: String,
    pub time_condition: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct cancel_order {
    pub aid: String,
    pub user_id: String,
    pub order_id: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct query_bank {
    pub aid: String,
    pub bank_id: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_password: String,
    pub currency: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct req_transfer {
    pub aid: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_id: String,
    pub bank_password: String,
    pub currency: String,
    pub amount: f64,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct subscribe_quote {
    pub aid: String,
    pub ins_list: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct peek_message {
    pub aid: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct login {
    pub aid: String,
    pub bid: String,
    pub user_name: String,
    pub password: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct query_settlement {
    pub aid: String,
    pub trading_day: String,
}
#[allow(non_camel_case_types)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct change_password {
    pub aid: String,
    pub old_password: String,
    pub new_password: String,
}



#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Notify{
    #[serde(rename = "type")]
    pub types : String,
    pub level : String,
    pub code : i32,
    pub content : String,
}
