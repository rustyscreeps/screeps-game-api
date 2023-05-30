//! Constants that are unique to object types or mechanics found in Screeps
//! seasonal.
//!
//! [Screeps seasonal documentation](https://docs-season.screeps.com/api/#Season-constants)

/// Constants for Screeps seasonal, season 1
///
/// These constants are relevant to the mechanics in seasonal, season 1.
/// [This mod](https://github.com/screeps/mod_season1) can be used to enable these mechanics
/// on a private server.
#[cfg(feature = "seasonal-season-1")]
pub mod season_1 {
    // RESOURCE_SCORE defined in `types.rs`
    // FIND_SCORE_CONTAINERS / FIND_SCORE_COLLECTORS defined in `find.rs`
    // LOOK_SCORE_CONTAINERS / LOOK_SCORE_COLLECTORS defined in `look.rs`
    /// Radius around each [`ScoreCollector`] that starts surrounded with
    /// [`StructureWall`]s with random hit points.
    ///
    /// [`ScoreCollector`]: crate::objects::ScoreCollector
    /// [`StructureWall`]: crate::objects::StructureWall
    pub const WALLS_RADIUS: u8 = 5;
    /// The percentage chance that a given room will have a [`ScoreContainer`]
    /// spawned in it every [`SCORE_CONTAINER_SPAWN_INTERVAL_TICKS`] ticks.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_CONTAINER_SPAWN_CHANCE: f32 = 0.01;
    // SCORE_CONTAINER_SPAWN_INTERVAL not implemented due to being no longer
    // used as of https://github.com/screeps/mod-season1/commit/3b4d0aaabfb4bffab80845ac4ea9611f83935e1c
    /// The number of ticks between chances to spawn a [`ScoreContainer`] in
    /// rooms at random.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_CONTAINER_SPAWN_INTERVAL_TICKS: u32 = 250;
    /// Amount of capacity in a [`ScoreCollector`] that regenerates each tick.
    ///
    /// [`ScoreCollector`]: crate::objects::ScoreCollector
    pub const SCORE_COLLECTOR_SINK: u32 = 20;
    /// Maximum capacity bucket for a [`ScoreCollector`].
    ///
    /// [`ScoreCollector`]: crate::objects::ScoreCollector
    pub const SCORE_COLLECTOR_MAX_CAPACITY: u32 = 20_000;
}

/// Constants for Screeps seasonal, season 2
///
/// These constants are relevant to the mechanics in seasonal, season 2.
/// [This mod](https://github.com/screeps/mod_season2) can be used to enable these mechanics
/// on a private server.
#[cfg(feature = "seasonal-season-2")]
pub mod season_2 {
    // RESOURCE_SYMBOL_* defined in `types.rs`
    use crate::constants::ResourceType;
    /// All of the resources which are 'symbols' that can be taken
    /// to the correct type of [`SymbolDecoder`] to score points.
    ///
    /// [`SymbolDecoder`]: crate::objects::SymbolDecoder
    pub const SYMBOLS: [ResourceType; 22] = [
        ResourceType::SymbolAleph,
        ResourceType::SymbolBeth,
        ResourceType::SymbolGimmel,
        ResourceType::SymbolDaleth,
        ResourceType::SymbolHe,
        ResourceType::SymbolWaw,
        ResourceType::SymbolZayin,
        ResourceType::SymbolHeth,
        ResourceType::SymbolTeth,
        ResourceType::SymbolYodh,
        ResourceType::SymbolKaph,
        ResourceType::SymbolLamedh,
        ResourceType::SymbolMem,
        ResourceType::SymbolNun,
        ResourceType::SymbolSamekh,
        ResourceType::SymbolAyin,
        ResourceType::SymbolPe,
        ResourceType::SymbolTsade,
        ResourceType::SymbolQoph,
        ResourceType::SymbolRes,
        ResourceType::SymbolSin,
        ResourceType::SymbolTaw,
    ];
    // FIND_SYMBOL_CONTAINERS / FIND_SYMBOL_DECODERS defined in `find.rs`
    // LOOK_SYMBOL_CONTAINERS / LOOK_SYMBOL_DECODERS defined in `look.rs`
    /// The percentage chance that a given room will have a [`SymbolContainer`]
    /// spawned in it every [`SYMBOL_CONTAINER_SPAWN_INTERVAL_TICKS`] ticks.
    ///
    /// [`SymbolContainer`]: crate::objects::SymbolContainer
    pub const SYMBOL_CONTAINER_SPAWN_CHANCE: f32 = 0.01;
    /// The number of ticks between chances to spawn a [`SymbolContainer`] in
    /// rooms at random.
    ///
    /// [`SymbolContainer`]: crate::objects::SymbolContainer
    pub const SYMBOL_CONTAINER_SPAWN_INTERVAL_TICKS: u32 = 250;
    /// Translates the `CONTROLLER_LEVEL_SCORE_MULTIPLIERS` constant, the score
    /// multipler for the room's [`SymbolDecoder`] depending on the RCL of the
    /// room.
    ///
    /// [`SymbolDecoder`]: crate::objects::SymbolDecoder
    #[inline]
    pub fn controller_level_score_multiplers(rcl: u32) -> u32 {
        match rcl {
            4 => 3,
            5 => 9,
            6 => 27,
            7 => 81,
            8 => 243,
            _ => 1,
        }
    }
}

// no unique mechanics or constants for seasons 3 (power) or 4 (commodities)

/// Constants for Screeps seasonal, season 5
///
/// These constants are relevant to the mechanics in seasonal, season 5.
// todo: mod link when it exists
#[cfg(feature = "seasonal-season-5")]
pub mod season_5 {
    // RESOURCE_THORIUM defined in `types.rs`
    // FIND_REACTORS defined in `find.rs`
    /// Capacity of the [`Thorium`] storage of a [`Reactor`].
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    /// [`Reactor`]: crate::objects::Reactor
    // no official constant for this currently, but providing as 'extra' constant
    // for consistency with prior seasons
    pub const REACTOR_THORIUM_CAPACITY: u32 = 1_000;

    /// The added decay each tick for all creeps and decaying structures when
    /// [`Thorium`] is present on the same tile.
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    pub fn thorium_decay(thorium_amount: u32) -> u32 {
        // Math.floor(Math.log10([total Thorium on the tile]))
        (thorium_amount as f64).log10().floor() as u32
    }

    /// The points generated by a [`Reactor`] processing [`Thorium`] each tick,
    /// which increases based on the number of ticks of continuous operation.
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    /// [`Reactor`]: crate::objects::Reactor
    pub fn reactor_points_per_tick(continuous_work_ticks: u32) -> u32 {
        // 1 + Math.floor(Math.log10([ticks of continuous operating]))
        1 + (continuous_work_ticks as f64).log10().floor() as u32
    }

    #[cfg(test)]
    mod test {
        use super::{reactor_points_per_tick, thorium_decay};

        #[test]
        fn decay_formula() {
            assert_eq!(thorium_decay(10), 1);
            assert_eq!(thorium_decay(100), 2);
            assert_eq!(thorium_decay(1000), 3);
        }

        #[test]
        fn score_formula() {
            assert_eq!(reactor_points_per_tick(0), 1);
            assert_eq!(reactor_points_per_tick(1), 1);
            assert_eq!(reactor_points_per_tick(10), 2);
            assert_eq!(reactor_points_per_tick(100), 3);
            assert_eq!(reactor_points_per_tick(1000), 4);
            assert_eq!(reactor_points_per_tick(10000), 5);
        }
    }
}
