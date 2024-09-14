//! Constants that are unique to object types or mechanics found in Screeps
//! seasonal.
//!
//! [Screeps seasonal documentation](https://docs-season.screeps.com/api/#Season-constants)

/// Constants for Screeps seasonal, season 1
///
/// These constants are relevant to the mechanics in seasonal, season 1.
/// [This mod](https://github.com/screeps/mod-season1) can be used to enable these mechanics
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

    // extra constants for s1 related to the score spawn bonus/crisis cycle
    // https://github.com/screeps/mod-season1/blob/7ca3c7ddb47bf9dfbdfb4e72b666a3159fde8780/src/scoreContainer.roomObject.js
    /// The duration of a full score cycle.
    pub const SCORE_CYCLE_DURATION: u32 = 50_000;

    /// The point of the score cycle where bonus time begins, multiplying
    /// spawned score by [`SCORE_CYCLE_BONUS_MULTIPLIER`].
    pub const SCORE_CYCLE_BONUS_START: u32 = 45_000;

    /// The point of the score cycle where bonus time ends.
    pub const SCORE_CYCLE_BONUS_END: u32 = 50_000;

    /// The multiplier for score spawned during bonus time.
    pub const SCORE_CYCLE_BONUS_MULTIPLIER: u8 = 2;

    /// The point of the score cycle where crisis time begins, during which no
    /// new [`ScoreContainer`]s will be spawned.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_CYCLE_CRISIS_START: u32 = 10_000;

    /// The point of the score cycle where crisis time ends, allowing
    /// [`ScoreContainer`]s to be spawned once again.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_CYCLE_CRISIS_END: u32 = 15_000;

    /// The multiplier for score spawned during crisis time.
    pub const SCORE_CYCLE_CRISIS_MULTIPLIER: u8 = 0;

    /// The minimum amount of ticks a [`ScoreContainer`] can exist before
    /// despawning.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    // https://github.com/screeps/mod-season1/blob/7ca3c7ddb47bf9dfbdfb4e72b666a3159fde8780/src/scoreContainer.roomObject.js#L93
    pub const SCORE_MIN_DECAY: u16 = 500;

    /// The maximum amount of ticks a [`ScoreContainer`] can exist before
    /// despawning.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_MAX_DECAY: u16 = 5_000;

    /// The different periods in the score cycle.
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum ScoreCycleState {
        /// Normal score spawns during the normal parts of the cycle
        Normal,
        /// No score spawns during the crisis period
        Crisis,
        /// Double score in each container during the bonus period
        Bonus,
    }

    impl ScoreCycleState {
        /// Gets the multiplier of score associated with the represented part of
        /// the score cycle.
        pub fn multiplier(&self) -> u8 {
            match self {
                ScoreCycleState::Normal => 1,
                ScoreCycleState::Crisis => SCORE_CYCLE_CRISIS_MULTIPLIER,
                ScoreCycleState::Bonus => SCORE_CYCLE_BONUS_MULTIPLIER,
            }
        }
    }

    /// Calculates the state of the score cycle for season 1
    pub const fn score_cycle_at_tick(tick: u32) -> ScoreCycleState {
        match tick % SCORE_CYCLE_DURATION {
            // the bonus/crisis periods are exclusive of their boundaries
            // https://github.com/screeps/mod-season1/blob/7ca3c7ddb47bf9dfbdfb4e72b666a3159fde8780/src/scoreContainer.roomObject.js#L77-L81
            // match on those exact values first
            SCORE_CYCLE_CRISIS_START => ScoreCycleState::Normal,
            SCORE_CYCLE_BONUS_START => ScoreCycleState::Normal,
            // then on the remaining ranges
            SCORE_CYCLE_CRISIS_START..SCORE_CYCLE_CRISIS_END => ScoreCycleState::Crisis,
            SCORE_CYCLE_BONUS_START..SCORE_CYCLE_BONUS_END => ScoreCycleState::Bonus,
            _ => ScoreCycleState::Normal,
        }
    }

    /// Calculates the state of the score cycle for season 7, which reverses the
    /// crisis/bonus order
    pub const fn s7_score_cycle_at_tick(tick: u32) -> ScoreCycleState {
        match tick % SCORE_CYCLE_DURATION {
            // the bonus/crisis periods are exclusive of their boundaries
            // https://github.com/screeps/mod-season1/blob/7ca3c7ddb47bf9dfbdfb4e72b666a3159fde8780/src/scoreContainer.roomObject.js#L77-L81
            // match on those exact values first
            SCORE_CYCLE_CRISIS_START => ScoreCycleState::Normal,
            SCORE_CYCLE_BONUS_START => ScoreCycleState::Normal,
            // then on the remaining ranges - these are flipped from normal in s7, which is
            // not currently open sourced
            SCORE_CYCLE_CRISIS_START..SCORE_CYCLE_CRISIS_END => ScoreCycleState::Bonus,
            SCORE_CYCLE_BONUS_START..SCORE_CYCLE_BONUS_END => ScoreCycleState::Crisis,
            _ => ScoreCycleState::Normal,
        }
    }

    #[cfg(test)]
    mod test {
        use super::{s7_score_cycle_at_tick, score_cycle_at_tick, ScoreCycleState};

        #[test]
        fn s1_score_cycle() {
            assert_eq!(score_cycle_at_tick(0), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(10_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(10_001), ScoreCycleState::Crisis);
            assert_eq!(score_cycle_at_tick(14_999), ScoreCycleState::Crisis);
            assert_eq!(score_cycle_at_tick(15_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(45_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(45_001), ScoreCycleState::Bonus);
            assert_eq!(score_cycle_at_tick(49_999), ScoreCycleState::Bonus);
            assert_eq!(score_cycle_at_tick(50_000), ScoreCycleState::Normal);

            assert_eq!(score_cycle_at_tick(200_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(210_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(210_001), ScoreCycleState::Crisis);
            assert_eq!(score_cycle_at_tick(214_999), ScoreCycleState::Crisis);
            assert_eq!(score_cycle_at_tick(215_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(245_000), ScoreCycleState::Normal);
            assert_eq!(score_cycle_at_tick(245_001), ScoreCycleState::Bonus);
            assert_eq!(score_cycle_at_tick(249_999), ScoreCycleState::Bonus);
            assert_eq!(score_cycle_at_tick(250_000), ScoreCycleState::Normal);
        }

        #[test]
        fn s7_score_cycle() {
            assert_eq!(s7_score_cycle_at_tick(0), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(10_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(10_001), ScoreCycleState::Bonus);
            assert_eq!(s7_score_cycle_at_tick(14_999), ScoreCycleState::Bonus);
            assert_eq!(s7_score_cycle_at_tick(15_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(45_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(45_001), ScoreCycleState::Crisis);
            assert_eq!(s7_score_cycle_at_tick(49_999), ScoreCycleState::Crisis);
            assert_eq!(s7_score_cycle_at_tick(50_000), ScoreCycleState::Normal);

            assert_eq!(s7_score_cycle_at_tick(200_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(210_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(210_001), ScoreCycleState::Bonus);
            assert_eq!(s7_score_cycle_at_tick(214_999), ScoreCycleState::Bonus);
            assert_eq!(s7_score_cycle_at_tick(215_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(245_000), ScoreCycleState::Normal);
            assert_eq!(s7_score_cycle_at_tick(245_001), ScoreCycleState::Crisis);
            assert_eq!(s7_score_cycle_at_tick(249_999), ScoreCycleState::Crisis);
            assert_eq!(s7_score_cycle_at_tick(250_000), ScoreCycleState::Normal);
        }
    }
}

/// Constants for Screeps seasonal, season 2
///
/// These constants are relevant to the mechanics in seasonal, season 2.
/// [This mod](https://github.com/screeps/mod-season2) can be used to enable these mechanics
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
    pub const fn controller_level_score_multiplers(rcl: u32) -> u32 {
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
/// [This mod](https://github.com/screeps/mod-season5) can be used to enable these mechanics
/// on a private server.
#[cfg(feature = "seasonal-season-5")]
pub mod season_5 {
    use crate::constants::Density;
    // RESOURCE_THORIUM defined in `types.rs`
    // FIND_REACTORS defined in `find.rs`
    // LOOK_REACTORS defined in `look.rs`
    /// Capacity of the [`Thorium`] storage of a [`Reactor`].
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    /// [`Reactor`]: crate::objects::Reactor
    // no official constant for this currently, but providing as 'extra' constant
    // for consistency with prior seasons
    pub const REACTOR_THORIUM_CAPACITY: u32 = 1_000;

    impl Density {
        /// Amount of [`Thorium`] generated for each density
        /// level, replacing the amounts from [`Density::amount`].
        ///
        /// [`Thorium`]: crate::constants::ResourceType::Thorium
        #[inline]
        pub const fn thorium_amount(self) -> u32 {
            match self {
                Density::Low => 10_000,
                Density::Moderate => 22_000,
                Density::High => 45_000,
                Density::Ultra => 67_000,
            }
        }
    }

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
            assert_eq!(thorium_decay(0), 0);
            // quantities of thorium below 10 do not cause any additional decay
            assert_eq!(thorium_decay(1), 0);
            assert_eq!(thorium_decay(9), 0);
            assert_eq!(thorium_decay(10), 1);
            assert_eq!(thorium_decay(99), 1);
            assert_eq!(thorium_decay(100), 2);
            assert_eq!(thorium_decay(999), 2);
            assert_eq!(thorium_decay(1000), 3);
            assert_eq!(thorium_decay(10_000), 4);
        }

        #[test]
        fn score_formula() {
            assert_eq!(reactor_points_per_tick(0), 1);
            assert_eq!(reactor_points_per_tick(1), 1);
            assert_eq!(reactor_points_per_tick(9), 1);
            assert_eq!(reactor_points_per_tick(10), 2);
            assert_eq!(reactor_points_per_tick(99), 2);
            assert_eq!(reactor_points_per_tick(100), 3);
            assert_eq!(reactor_points_per_tick(999), 3);
            assert_eq!(reactor_points_per_tick(1000), 4);
            assert_eq!(reactor_points_per_tick(10_000), 5);
            assert_eq!(reactor_points_per_tick(100_000), 6);
        }
    }
}
