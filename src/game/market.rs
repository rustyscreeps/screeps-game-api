//! See [https://docs.screeps.com/api/#Game-market]
//!
//! [https://docs.screeps.com/api/#Game-market]: https://docs.screeps.com/api/#Game-market
use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    constants::{ResourceType, ReturnCode},
    local::LocalRoomName,
    macros::*,
    traits::TryInto,
    Room,
};

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum OrderType {
    Sell = 0,
    Buy = 1,
}

// impl OrderType {
//     fn as_string(&self) -> String {
//         match self {
//             OrderType::Sell => String::from("sell"),
//             OrderType::Buy => String::from("buy")
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct Player {
    username: String,
}
js_deserializable!(Player);

#[derive(Deserialize, Debug)]
pub struct TransactionOrder {
    id: String,
    #[serde(rename = "type")]
    order_type: String,
    price: f64,
}
js_deserializable!(TransactionOrder);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    transaction_id: String,
    time: u32,
    sender: Player,
    recipient: Player,
    resource_type: String,
    amount: u32,
    from: String,
    to: String,
    description: String,
    order: Option<TransactionOrder>,
}
js_deserializable!(Transaction);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    id: String,
    created: u32,
    #[serde(rename = "type")]
    order_type: String,
    resource_type: String,
    room_name: LocalRoomName,
    amount: u32,
    remaining_amount: u32,
    price: f64,
}
js_deserializable!(Order);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyOrder {
    id: String,
    created: u32,
    active: bool,
    #[serde(rename = "type")]
    order_type: String,
    resource_type: String,
    room_name: LocalRoomName,
    amount: u32,
    remaining_amount: u32,
    total_amount: u32,
    price: f64,
}
js_deserializable!(MyOrder);

pub fn credits() -> f64 {
    js_unwrap!(Game.market.credits)
}

pub fn incoming_transactions() -> Vec<Transaction> {
    let arr_transaction_value = js! {
        return Game.market.incomingTransactions;
    };
    arr_transaction_value.try_into().unwrap()
}

pub fn outgoing_transactions() -> Vec<Transaction> {
    let arr_transaction_value = js! {
        return Game.market.outgoingTransactions;
    };
    arr_transaction_value.try_into().unwrap()
}

pub fn orders() -> HashMap<String, MyOrder> {
    let order_book_value = js! {
        return Game.market.orders;
    };
    order_book_value.try_into().unwrap()
}

pub fn calc_transaction_cost(amount: u32, room1: &Room, room2: &Room) -> f64 {
    js_unwrap!(Game.market.calcTransactionCost(@{amount}, @{room1.name()}, @{room2.name()}))
}

pub fn cancel_order(order_id: &str) -> ReturnCode {
    js_unwrap!(Game.market.cancelOrder(@{order_id}))
}

pub fn change_order_price(order_id: &str, new_price: f64) -> ReturnCode {
    js_unwrap!(Game.market.changeOrderPrice(@{order_id}, @{new_price}))
}

pub fn create_order(
    order_type: OrderType,
    resource_type: ResourceType,
    price: f64,
    total_amount: u32,
    room: &Room,
) -> ReturnCode {
    js_unwrap! {
        Game.market.createOrder(__order_type_num_to_str(@{order_type as u32}),
                                __resource_type_num_to_str(@{resource_type as u32}),
                                @{price},
                                @{total_amount},
                                @{room.name()})
    }
}

pub fn deal(order_id: &str, amount: u32, target_room: &Room) -> ReturnCode {
    js_unwrap! {Game.market.deal(@{order_id}, @{amount}, @{target_room.name()})}
}

pub fn extend_order(order_id: &str, add_amount: u32) -> ReturnCode {
    js_unwrap! {Game.market.extendOrder(@{order_id}, @{add_amount})}
}

/// Get all orders from the market
///
/// Contrary to the JS version, filtering should be done afterwards.
pub fn get_all_orders() -> Vec<Order> {
    let all_order = js! {
        return Game.market.getAllOrders();
    };
    all_order.try_into().unwrap()
}

pub fn get_order(id: &str) -> Option<Order> {
    let order = js! {
        return Game.marget.getOrder(@{id});
    };
    order.try_into().ok()
}
