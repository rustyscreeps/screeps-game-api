//! Global Control Level information.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "gcl")]
    type Gcl;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gcl", static_method_of = Gcl, getter, js_name = level)]
    fn level() -> u32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gcl", static_method_of = Gcl, getter, js_name = progress)]
    fn progress() -> f64;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gcl", static_method_of = Gcl, getter, js_name = progressTotal)]
    fn progress_total() -> f64;
}

/// Your current Global Control Level, which determines the number of rooms
/// you are allowed to claim.
pub fn level() -> u32 {
    Gcl::level()
}

/// Your progress toward the next Global Control Level.
pub fn progress() -> f64 {
    Gcl::progress()
}

/// Total progress needed to reach the next Global Control Level.
pub fn progress_total() -> f64 {
    Gcl::progress_total()
}
