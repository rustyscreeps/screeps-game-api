//! Access the in-game market to buy or sell resources.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game-market)

use js_sys::{Array, JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::{MarketResourceType, OrderType, ResourceType, ReturnCode},
    js_collections::{JsCollectionFromValue, JsHashMap},
    local::{LodashFilter, RoomName},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "market")]
    type Market;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, getter, js_name = credits)]
    fn credits() -> f64;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, getter, js_name = incomingTransactions)]
    fn incoming_transactions() -> Array;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, getter, js_name = outgoingTransactions)]
    fn outgoing_transactions() -> Array;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, getter, js_name = orders)]
    fn orders() -> Object;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = calcTransactionCost)]
    fn calc_transaction_cost(amount: u32, room_1: &JsString, room_2: &JsString) -> u32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = cancelOrder)]
    fn cancel_order(order_id: &JsString) -> ReturnCode;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = changeOrderPrice)]
    fn change_order_price(order_id: &JsString, new_price: f64) -> ReturnCode;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = createOrder)]
    fn create_order(order_parameters: &Object) -> ReturnCode;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = deal)]
    fn deal(order_id: &JsString, amount: u32, room_name: Option<&JsString>) -> ReturnCode;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = extendOrder)]
    fn extend_order(order_id: &JsString, add_amount: u32) -> ReturnCode;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = getAllOrders)]
    fn get_all_orders(filter: Option<&LodashFilter>) -> Array;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = getHistory)]
    fn get_history(resource: Option<ResourceType>) -> Option<Array>;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "market", static_method_of = Market, js_name = getOrderById)]
    fn get_order_by_id(order_id: &JsString) -> Option<Order>;
}

/// Your current credit balance.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.credits)
pub fn credits() -> f64 {
    Market::credits()
}

/// An [`Array`] of the last 100 [`Transaction`]s sent to your terminals.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.incomingTransactions)
pub fn incoming_transactions() -> Vec<Transaction> {
    Market::incoming_transactions()
        .iter()
        .map(Into::into)
        .collect()
}

/// An [`Array`] of the last 100 [`Transaction`]s sent from your terminals.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.outgoingTransactions)
pub fn outgoing_transactions() -> Vec<Transaction> {
    Market::outgoing_transactions()
        .iter()
        .map(Into::into)
        .collect()
}

/// An [`Object`] with your current buy and sell orders on the market, with
/// order ID [`JsString`] keys and [`MyOrder`] values.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.orders)
pub fn orders() -> JsHashMap<String, MyOrder> {
    Market::orders().into()
}

// todo maybe just implement a native version of this instead?
/// Get the amount of energy required to send a given amount of any resource
/// from one room to another.  See [`TERMINAL_SEND_COST_SCALE`] for
/// information about the calculation.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.calcTransactionCost)
///
/// [`TERMINAL_SEND_COST_SCALE`]: crate::constants::TERMINAL_SEND_COST_SCALE
pub fn calc_transaction_cost(amount: u32, room_1: &JsString, room_2: &JsString) -> u32 {
    Market::calc_transaction_cost(amount, room_1, room_2)
}

/// Cancel one of your existing orders on the market, without refunding
/// associated fees.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.cancelOrder)
pub fn cancel_order(order_id: &JsString) -> ReturnCode {
    Market::cancel_order(order_id)
}

/// Change the price of an existing order. If new_price is greater than old
/// price, you will be charged
/// (newPrice-oldPrice)*remainingAmount*[`MARKET_FEE`] credits.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.changeOrderPrice)
///
/// [`MARKET_FEE`]: crate::constants::MARKET_FEE
pub fn change_order_price(order_id: &JsString, new_price: f64) -> ReturnCode {
    Market::change_order_price(order_id, new_price)
}

// todo type to serialize call options into
/// Create a new order on the market.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.createOrder)
pub fn create_order(order_parameters: &Object) -> ReturnCode {
    Market::create_order(order_parameters)
}

/// Execute a trade on an order on the market. Name of a room with a
/// terminal from which to send or receive resources is required unless the
/// order is for an account resource.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.deal)
pub fn deal(order_id: &JsString, amount: u32, room_name: Option<RoomName>) -> ReturnCode {
    match room_name {
        Some(r) => Market::deal(order_id, amount, Some(&r.into())),
        None => Market::deal(order_id, amount, None),
    }
}

/// Adds more capacity to one of your existing orders, offering or
/// requesting more of the resource and incurring additional fees.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.extendOrder)
pub fn extend_order(order_id: &JsString, add_amount: u32) -> ReturnCode {
    Market::extend_order(order_id, add_amount)
}

/// Get all [`Order`]s on the market, with an optional
/// filter. Note that a `resourceType` filter has special handling in the engine
/// to be more efficient ([source]).
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.getAllOrders)
///
/// [source]: https://github.com/screeps/engine/blob/f7a09e637c20689084fcf4eb43eacdfd51d31476/src/game/market.js#L37
pub fn get_all_orders(filter: Option<&LodashFilter>) -> Vec<Order> {
    Market::get_all_orders(filter)
        .iter()
        .map(Into::into)
        .collect()
}

/// Get information about the price history on the market for the last 14
/// days for a given resource as an [`Array`] of [`OrderHistoryRecord`]s, or
/// for all resources if `None`. Warning: returns an empty [`Object`]
/// instead of an array if there is no history for the resource, verifying
/// the type is recommended before use if the market might be empty.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.getHistory)
pub fn get_history(resource: Option<ResourceType>) -> Vec<OrderHistoryRecord> {
    Market::get_history(resource)
        .map(|arr| arr.iter().map(Into::into).collect())
        .unwrap_or_else(Vec::new)
}

/// Get an object with information about a specific order, in the same
/// format as returned by [`get_all_orders`]
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.market.getOrderById)
pub fn get_order_by_id(order_id: &str) -> Option<Order> {
    let order_id: JsString = order_id.into();

    Market::get_order_by_id(&order_id)
}

#[wasm_bindgen]
extern "C" {
    /// An object that represents an order on the market.
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type Order;
    /// The order ID, which can be used to retrieve the order, or execute a
    /// trade using [`MarketInfo::deal`].
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Order) -> JsString;
    /// Tick of order creation, `None` for intershard orders.
    #[wasm_bindgen(method, getter)]
    pub fn created(this: &Order) -> Option<u32>;
    // todo should be u64 but seems to panic at the moment, follow up
    /// Timestamp of order creation in milliseconds since epoch.
    #[wasm_bindgen(method, getter = createdTimestamp)]
    pub fn created_timestamp(this: &Order) -> f64;
    /// The [`OrderType`] of the order (whether the owner is looking to buy or
    /// sell the given resource).
    #[wasm_bindgen(method, getter = type)]
    pub fn order_type(this: &Order) -> OrderType;
    /// The resource type this order is for.
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Order) -> MarketResourceType;
    /// Room that owns the order, `None` for intershard orders.
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &Order) -> Option<JsString>;
    /// The amount of resource currently ready to be traded (loaded in the
    /// terminal).
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Order) -> u32;
    /// The total remaining amount of the resource to be traded before this
    /// order has been completely filled and removed.
    #[wasm_bindgen(method, getter = remainingAmount)]
    pub fn remaining_amount(this: &Order) -> u32;
    /// Price of the order per unit of the resource the order is for.
    #[wasm_bindgen(method, getter)]
    pub fn price(this: &Order) -> f64;
}

// todo docs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type Transaction;
    #[wasm_bindgen(method, getter = transactionId)]
    pub fn transaction_id(this: &Transaction) -> JsString;
    #[wasm_bindgen(method, getter)]
    pub fn time(this: &Transaction) -> u32;
    /// The player who sent resources for this transaction, or `None` if it was
    /// an NPC terminal
    #[wasm_bindgen(method, getter)]
    pub fn sender(this: &Transaction) -> Option<Player>;
    /// The recipient of the resources for this transaction, or `None` if it was
    /// an NPC terminal
    #[wasm_bindgen(method, getter)]
    pub fn recipient(this: &Transaction) -> Option<Player>;
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Transaction) -> ResourceType;
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Transaction) -> u32;
    /// The room that sent resources for this transaction
    #[wasm_bindgen(method, getter)]
    pub fn from(this: &Transaction) -> JsString;
    /// The room that received resources in this transaction
    #[wasm_bindgen(method, getter)]
    pub fn to(this: &Transaction) -> JsString;
    /// The description set in the sender's `StructureTerminal::send()` call, if
    /// any
    #[wasm_bindgen(method, getter)]
    pub fn description(this: &Transaction) -> Option<JsString>;
    /// Information about the market order that this transaction was fulfilling,
    /// if any
    #[wasm_bindgen(method, getter = order)]
    pub fn order(this: &Transaction) -> Option<TransactionOrder>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type Player;
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Player) -> JsString;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type TransactionOrder;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &TransactionOrder) -> JsString;
    #[wasm_bindgen(method, getter = type)]
    pub fn order_type(this: &TransactionOrder) -> OrderType;
    #[wasm_bindgen(method, getter)]
    pub fn price(this: &TransactionOrder) -> f64;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type MyOrder;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &MyOrder) -> JsString;
    /// Tick of order creation, `None` for intershard orders
    #[wasm_bindgen(method, getter)]
    pub fn created(this: &MyOrder) -> Option<u32>;
    /// Timestamp of order creation in milliseconds since epoch
    #[wasm_bindgen(method, getter = createdTimestamp)]
    pub fn created_timestamp(this: &MyOrder) -> u64;
    #[wasm_bindgen(method, getter)]
    pub fn active(this: &MyOrder) -> bool;
    #[wasm_bindgen(method, getter = type)]
    pub fn order_type(this: &MyOrder) -> OrderType;
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &MyOrder) -> MarketResourceType;
    /// Room that owns the order, `None` for intershard orders
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &MyOrder) -> Option<JsString>;
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &MyOrder) -> u32;
    #[wasm_bindgen(method, getter = remainingAmount)]
    pub fn remaining_amount(this: &MyOrder) -> u32;
    #[wasm_bindgen(method, getter = totalAmount)]
    pub fn total_amount(this: &MyOrder) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn price(this: &MyOrder) -> f64;
}

impl JsCollectionFromValue for MyOrder {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type OrderHistoryRecord;
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &OrderHistoryRecord) -> MarketResourceType;
    /// Calendar date in string format, eg "2018-12-31"
    #[wasm_bindgen(method, getter)]
    pub fn date(this: &OrderHistoryRecord) -> JsString;
    /// Total number of transactions for this resource on this day
    #[wasm_bindgen(method, getter)]
    pub fn transactions(this: &OrderHistoryRecord) -> u32;
    /// Total volume of this resource bought and sold on this day
    #[wasm_bindgen(method, getter)]
    pub fn volume(this: &OrderHistoryRecord) -> u32;
    #[wasm_bindgen(method, getter = avgPrice)]
    pub fn avg_price(this: &OrderHistoryRecord) -> f64;
    #[wasm_bindgen(method, getter = stddevPrice)]
    pub fn stddev_price(this: &OrderHistoryRecord) -> f64;
}
