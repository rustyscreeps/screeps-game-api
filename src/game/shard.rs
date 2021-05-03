//! Shard information.
//!
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard)

use wasm_bindgen::prelude::*;
use js_sys::JsString;

#[wasm_bindgen]
extern "C" {
    /// Your current Global Control Level, which determines the number of rooms
    /// you are allowed to claim.
    #[wasm_bindgen(js_namespace = ["Game", "shard"], getter = name)]
    fn name_internal() -> JsString;

    /// Your progress toward the next Global Control Level.
    #[wasm_bindgen(js_namespace = ["Game", "shard"], getter = type)]
    fn type_internal() -> JsString;

    /// Total progress needed to reach the next Global Control Level.
    #[wasm_bindgen(js_namespace = ["Game", "shard"], getter = ptr)]
    pub fn ptr() -> bool;
}

pub fn name() -> String {
    name_internal().into()
}

pub fn shard_type() -> String {
    type_internal().into()
}