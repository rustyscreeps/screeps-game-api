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
/// The percentage chance that a given room will have a [`ScoreContainer`]
/// spawned in it every [`SCORE_CONTAINER_SPAWN_INTERVAL`] ticks.
///
/// [`ScoreContainer`]: crate::objects::ScoreContainer
#[cfg(feature = "enable-score")]
pub const SCORE_CONTAINER_SPAWN_CHANCE: f32 = 0.01;
/// The number of ticks between chances to spawn a [`ScoreContainer`] in rooms
/// at random.
///
/// [`ScoreContainer`]: crate::objects::ScoreContainer
#[cfg(feature = "enable-score")]
pub const SCORE_CONTAINER_SPAWN_INTERVAL: u32 = 500;
/// Amount of capacity in a [`ScoreCollector`] that regenerates each tick.
///
/// [`ScoreCollector`]: crate::objects::ScoreCollector
#[cfg(feature = "enable-score")]
pub const SCORE_COLLECTOR_SINK: u32 = 20;
/// Maximum capacity bucket for a [`ScoreCollector`].
///
/// [`ScoreCollector`]: crate::objects::ScoreCollector
#[cfg(feature = "enable-score")]
pub const SCORE_COLLECTOR_MAX_CAPACITY: u32 = 20_000;
// possible todo, Seasonal season 1 extras:
// https://github.com/screeps/mod_season1/blob/fef5c8e39904ae150b0ae396028fa5faf51cc5a5/src/scoreContainer.roomObject.js#L1
