//! See [http://docs.screeps.com/api/#Game.gcl]
//!
//! [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
use crate::macros::*;

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub fn level() -> u32 {
    js_unwrap!(Game.gcl.level)
}

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub fn progress() -> f64 {
    js_unwrap!(Game.gcl.progress)
}

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub fn progress_total() -> f64 {
    js_unwrap!(Game.gcl.progressTotal)
}
