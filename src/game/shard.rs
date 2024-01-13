//! Information about the current shard.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.shard)
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "shard")]
    type Shard;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = name)]
    fn name() -> JsString;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = type)]
    fn shard_type() -> JsString;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = ptr)]
    fn ptr() -> bool;
}

/// Current shard name.
pub fn name() -> String {
    Shard::name().into()
}

/// Shard type. Currently always "normal".
pub fn shard_type() -> String {
    Shard::shard_type().into()
}

/// Flag for if this is a public test server or not.
pub fn ptr() -> bool {
    Shard::ptr()
}
