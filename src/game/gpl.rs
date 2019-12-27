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
