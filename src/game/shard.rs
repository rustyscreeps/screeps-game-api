//! Information about the current shard.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.shard)
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "shard")]
    type Shard;

    /// Current shard name.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = name)]
    pub fn name() -> JsString;

    /// Shard type. Currently always "normal".
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = type)]
    pub fn shard_type() -> JsString;

    /// Flag for if this is a public test server or not.
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = ptr)]
    pub fn ptr() -> bool;
}
