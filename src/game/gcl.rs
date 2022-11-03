//! Global Control Level information.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
use crate::constants::{GCL_MULTIPLY, GCL_POW};

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

/// Provides the total number of control points needed to achieve each level of
/// GCL.
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The resulting value for your current level, added to
/// your [`progress`], would calculate your total lifetime control
/// points.
pub fn total_for_level(level: u32) -> f64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117
    ((level - 1) as f64).powf(GCL_POW) * GCL_MULTIPLY as f64
}
