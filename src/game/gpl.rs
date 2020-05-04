//! See [http://docs.screeps.com/api/#Game.gpl]
//!
//! [http://docs.screeps.com/api/#Game.gpl]: http://docs.screeps.com/api/#Game.gpl

use crate::constants::{POWER_LEVEL_MULTIPLY, POWER_LEVEL_POW};

/// See [http://docs.screeps.com/api/#Game.gpl]
///
/// [http://docs.screeps.com/api/#Game.gpl]: http://docs.screeps.com/api/#Game.gpl
pub fn level() -> u32 {
    js_unwrap!(Game.gpl.level)
}

/// See [http://docs.screeps.com/api/#Game.gpl]
///
/// [http://docs.screeps.com/api/#Game.gpl]: http://docs.screeps.com/api/#Game.gpl
pub fn progress() -> f64 {
    js_unwrap!(Game.gpl.progress)
}

/// See [http://docs.screeps.com/api/#Game.gpl]
///
/// [http://docs.screeps.com/api/#Game.gpl]: http://docs.screeps.com/api/#Game.gpl
pub fn progress_total() -> f64 {
    js_unwrap!(Game.gpl.progressTotal)
}

/// Provides the total number of processed power needed to achieve each level
/// of GPL
///
/// Calculates the total number of power that need to be processed to achieve a
/// given Global Power Level. The resulting value for your current level, added
/// to your [`gpl::progress`][crate::game::gpl::progress], would calculate your
/// total lifetime power points.
pub fn total_for_level(level: u32) -> u64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120
    (level as u64).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u64
}
