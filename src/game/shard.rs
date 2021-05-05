//! Shard information.
//!
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard)

use wasm_bindgen::prelude::*;
use js_sys::JsString;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "shard")]
    pub type Shard;

    /// Current shard name.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = name)]
    fn name() -> JsString;

    /// Shard type.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = type)]
    fn shard_type() -> JsString;

    /// Flag for if this is a public test server or not.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = ptr)]
    pub fn ptr() -> bool;
}

pub fn name() -> String {
    Shard::name().into()
}

pub fn shard_type() -> String {
    Shard::shard_type().into()
}

pub fn ptr() -> bool {
    Shard::ptr()
}