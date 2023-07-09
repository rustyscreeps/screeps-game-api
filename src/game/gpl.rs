//! Global Power Level information.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game.gpl)
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "gpl")]
    type Gpl;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gpl", static_method_of = Gpl, getter, js_name = level)]
    fn level() -> u32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gpl", static_method_of = Gpl, getter, js_name = progress)]
    fn progress() -> f64;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "gpl", static_method_of = Gpl, getter, js_name = progressTotal)]
    fn progress_total() -> f64;
}

/// Your current Global Power Level, which determines power creep
/// development.
pub fn level() -> u32 {
    Gpl::level()
}

/// Your progress toward the next Global Power Level.
pub fn progress() -> f64 {
    Gpl::progress()
}

/// Total progress needed to reach the next Global Power Level.
pub fn progress_total() -> f64 {
    Gpl::progress_total()
}
