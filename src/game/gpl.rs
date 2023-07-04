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
pub fn total_for_level(level: u32) -> u128 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120
    (level as u128).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u128
}

#[cfg(test)]
mod test {
    use super::total_for_level;

    #[test]
    fn level_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_eq!(total_for_level(0), 0);
        assert_eq!(total_for_level(1), 1_000);
        assert_eq!(total_for_level(2), 4_000);
        assert_eq!(total_for_level(3), 9_000);
        assert_eq!(total_for_level(4), 16_000);
        assert_eq!(total_for_level(5), 25_000);
        assert_eq!(total_for_level(6), 36_000);
        assert_eq!(total_for_level(7), 49_000);
        assert_eq!(total_for_level(8), 64_000);
        assert_eq!(total_for_level(9), 81_000);
        assert_eq!(total_for_level(10), 100_000);
        assert_eq!(total_for_level(50), 2_500_000);
        assert_eq!(total_for_level(100), 10_000_000);
        assert_eq!(total_for_level(1_000), 1_000_000_000);
        assert_eq!(total_for_level(5_000), 25_000_000_000);
        assert_eq!(total_for_level(10_000), 100_000_000_000);
        assert_eq!(total_for_level(50_000), 2_500_000_000_000);
        assert_eq!(total_for_level(100_000), 10_000_000_000_000);
        assert_eq!(total_for_level(1_000_000), 1_000_000_000_000_000);
        assert_eq!(total_for_level(5_000_000), 25_000_000_000_000_000);
        assert_eq!(total_for_level(10_000_000), 100_000_000_000_000_000);
        assert_eq!(total_for_level(100_000_000), 10_000_000_000_000_000_000);
        // beyond this value the return overflows a u64
        assert_eq!(total_for_level(135_818_791), 18_446_743_988_701_681_000);
        // must be u128 return to fit this one!
        assert_eq!(total_for_level(135_818_792), 18_446_744_260_339_264_000);
        assert_eq!(
            total_for_level(1_000_000_000),
            1_000_000_000_000_000_000_000
        );
        assert_eq!(
            total_for_level(4_000_000_000),
            16_000_000_000_000_000_000_000
        );
        assert_eq!(total_for_level(u32::MAX), 18_446_744_065_119_617_025_000);
    }
}
