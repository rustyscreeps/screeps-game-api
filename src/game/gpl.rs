//! Global Power Level information.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game.gpl)
use wasm_bindgen::prelude::*;

use crate::constants::{POWER_LEVEL_MULTIPLY, POWER_LEVEL_POW};

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

/// Provides the total number of processed power needed to achieve each level
/// of GPL.
///
/// Calculates the total number of power that need to be processed to achieve a
/// given Global Power Level. The resulting value for your current level, added
/// to your [`progress`], would calculate your total lifetime power
/// points.
pub const fn total_for_level(level: u32) -> u64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120
    (level as u64).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u64
}
