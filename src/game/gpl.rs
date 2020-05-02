//! See [http://docs.screeps.com/api/#Game.gpl]
//!
//! [http://docs.screeps.com/api/#Game.gpl]: http://docs.screeps.com/api/#Game.gpl

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

/// Provides the total number of processed power needed to acheieve each level
/// of GPL
///
/// Calculates the total number of power that need to be processed to acheive a
/// given Global Power Level. The resulting value for your current level, added
/// to your [`gpl::progress`][crate::game::gpl::progress], would calculate your
/// total lifetime power points.
pub fn total_for_level(level: u32) -> u64 {
    (level as u64).pow(2) * 1_000
}
