//! Global Control Level information.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
use wasm_bindgen::prelude::*;

use crate::constants::{GCL_MULTIPLY, GCL_POW};

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

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use super::total_for_level;

    #[test]
    fn level_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_approx_eq!(total_for_level(1), 0.);
        assert_approx_eq!(total_for_level(2), 1000000.);
        assert_approx_eq!(total_for_level(3), 5278031.643091577);
        assert_approx_eq!(total_for_level(4), 13966610.165238237);
        assert_approx_eq!(total_for_level(5), 27857618.025475968);
        assert_approx_eq!(total_for_level(6), 47591348.46789695);
        assert_approx_eq!(total_for_level(7), 73716210.39885189);
        assert_approx_eq!(total_for_level(8), 106717414.7996562);
        assert_approx_eq!(total_for_level(9), 147033389.43962047);
        assert_approx_eq!(total_for_level(10), 195066199.50773603);
        assert_approx_eq!(total_for_level(11), 251188643.15095797);
        assert_approx_eq!(total_for_level(12), 315749334.8687436);
        assert_approx_eq!(total_for_level(13), 389076491.09393656);
        assert_approx_eq!(total_for_level(14), 471480836.66525537);
        assert_approx_eq!(total_for_level(15), 563257892.1815147);
        assert_approx_eq!(total_for_level(16), 664689811.2891247);
        assert_approx_eq!(total_for_level(17), 776046882.0533236);
        assert_approx_eq!(total_for_level(18), 897588771.9617443);
        assert_approx_eq!(total_for_level(19), 1029565573.4994452);
        assert_approx_eq!(total_for_level(20), 1172218691.9999762);
        assert_approx_eq!(total_for_level(25), 2053558031.5768352);
        assert_approx_eq!(total_for_level(30), 3234113036.1951885);
        assert_approx_eq!(total_for_level(31), 3508253856.824569);
        assert_approx_eq!(total_for_level(32), 3795491867.4194345);
        assert_approx_eq!(total_for_level(33), 4095999999.9999986);
        assert_approx_eq!(total_for_level(34), 4409947870.045006);
        assert_approx_eq!(total_for_level(35), 4737501940.897796);
        assert_approx_eq!(total_for_level(40), 6584989046.083984);
        assert_approx_eq!(total_for_level(45), 8796024362.57156);
        assert_approx_eq!(total_for_level(50), 11388606621.52188);
        assert_approx_eq!(total_for_level(100), 61592022749.941284);
        assert_approx_eq!(total_for_level(1000), 15810921110646.998);
        assert_approx_eq!(total_for_level(u32::MAX), 1.3155388150906982e29);
    }

    #[test]
    #[should_panic]
    fn bad_level_formula_input() {
        // players cannot be GCL 0, and subtracting 1 (as the formula does)
        // overflows the u32 - this should panic.
        total_for_level(0);
    }
}
