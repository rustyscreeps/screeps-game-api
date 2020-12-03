//! Constants that apply only to Screeps seasonal servers.
//!
//! [Screeps seasonal documentation](https://docs-season.screeps.com/api/#Season-constants)
// Season 1's RESOURCE_SCORE defined in `types.rs`
// Season 1's FIND_SCORE_CONTAINERS defined in `find.rs`
/// Radius around each [`ScoreCollector`] that starts surrounded with
/// constructed walls with random hit points.
///
/// [`ScoreCollector`]: crate::objects::ScoreCollector
#[cfg(feature = "enable-score")]
pub const WALLS_RADIUS: u32 = 5;
// todo mechanics once they're public
#[cfg(feature = "enable-score")]
pub const SCORE_CONTAINER_SPAWN_CHANCE: f32 = 0.01;
// todo mechanics once they're public
#[cfg(feature = "enable-score")]
pub const SCORE_CONTAINER_SPAWN_INTERVAL: u32 = 500;
/// Amount of capacity in a [`ScoreCollector`] that regenerates each tick.
///
/// [`ScoreCollector`]: crate::objects::ScoreCollector
#[cfg(feature = "enable-score")]
pub const SCORE_COLLECTOR_SINK: u32 = 20;
/// Maximum capacity bucket for a [`ScoreCollector`]
///
/// [`ScoreCollector`]: crate::objects::ScoreCollector
#[cfg(feature = "enable-score")]
pub const SCORE_COLLECTOR_MAX_CAPACITY: u32 = 20_000;
