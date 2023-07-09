//! Additional constants representing internal game mechanics that aren't
//! included in the game's constants

/// Percentage of energy spent on construction that is left in a [`Resource`] if
/// the construction site is destroyed by [`ConstructionSite::remove`], or from
/// being stepped on by a hostile creep
///
/// [`Resource`]: crate::objects::Resource
/// [`ConstructionSite::remove`]: crate::objects::ConstructionSite::remove
pub const CONSTRUCTION_SITE_STOMP_RATIO: f32 = 0.5;

/// Maximum length (in UTF-16 units) of input to [`Creep::sign_controller`]
///
/// [`Creep::sign_controller`]: crate::objects::Creep::sign_controller
pub const CONTROLLER_SIGN_MAX_LENGTH: u32 = 100;

/// Maximum amount of CPU that can be accumulated in [`game::cpu::bucket`] per
/// shard
///
/// [`game::cpu::bucket`]: crate::game::cpu::bucket
pub const CPU_BUCKET_MAX: u32 = 10_000;

/// Time (in milliseconds) that the [`cpu::set_shard_limits`] function needs to
/// cool down between calls
///
/// [`cpu::set_shard_limits`]: crate::game::cpu::set_shard_limits
pub const CPU_SET_SHARD_LIMITS_COOLDOWN: u32 = 12 * 3600 * 1000;

/// Maximum value of [`cpu::tick_limit`], the amount of CPU time (in
/// milliseconds) available this tick, including available time from the bucket
/// ([`game::cpu::bucket`])
///
/// [`cpu::tick_limit`]: crate::game::cpu::tick_limit
/// [`game::cpu::bucket`]: crate::game::cpu::bucket
pub const CPU_TICK_LIMIT_MAX: u32 = 500;

/// [`Creep::hits_max`] gained for each [`Part`] in a given creep's
/// [`Creep::body`]
///
/// [`Creep::hits_max`]: crate::objects::Creep::hits_max
/// [`Part`]: crate::constants::Part
/// [`Creep::body`]: crate::objects::Creep::body
pub const CREEP_HITS_PER_PART: u32 = 100;

/// Maximum length (in UTF-16 units) of the string input to
/// [`StuctureSpawn::spawn_creep`] for the name of a [`Creep`]
///
/// [`StuctureSpawn::spawn_creep`]: crate::objects::StructureSpawn::spawn_creep
/// [`Creep`]: crate::objects::Creep
pub const CREEP_NAME_MAX_LENGTH: u32 = 100;

/// Maximum length (in UTF-16 units) of string input to [`Creep::say`] and
/// [`PowerCreep::say`]
///
/// [`Creep::say`]: crate::objects::Creep::say
/// [`PowerCreep::say`]: crate::objects::PowerCreep::say
pub const CREEP_SAY_MAX_LENGTH: u32 = 10;

/// Maximum length (in UTF-16 units) of names of [`Flag`] objects.
///
/// [`Flag`]: crate::objects::Flag
pub const FLAG_NAME_MAX_LENGTH: u32 = 60;

/// The cost of a single 'intent' (in milliseconds), a CPU penalty charged for
/// most successful API calls which change the game state
pub const INTENT_CPU_COST: f64 = 0.2;

/// Maximum size (in UTF-16 units) of data input to
/// [`inter_shard_memory::set_local`]
///
/// [`inter_shard_memory::set_local`]: crate::inter_shard_memory::set_local
pub const INTER_SHARD_MEMORY_SIZE_LIMIT: u32 = 100 * 1024;

/// The [`Owner::username`] of hostile non-player structures and creeps which
/// create strongholds and spawn to attack rooms due to energy harvesting
/// activity
///
/// [`Owner::username`]: crate::objects::Owner::username
pub const INVADER_USERNAME: &str = "Invader";

/// Maximum range between interacting [`StructureLab`] strucures for
/// [`StructureLab::run_reaction`] or [`StructureLab::reverse_reaction`]
///
/// [`StructureLab`]: crate::objects::StructureLab
/// [`StructureLab::run_reaction`]: crate::objects::StructureLab::run_reaction
/// [`StructureLab::reverse_reaction`]: crate::objects::StructureLab::reverse_reaction
pub const LAB_REACTION_RANGE: u32 = 2;

/// The maximum size (in UTF-16 units) of the serialized [`MapVisual`] data.
///
/// [`MapVisual`]: crate::objects::MapVisual
pub const MAP_VISUAL_SIZE_LIMIT: u32 = 1000 * 1024;

/// The maximum number of times that you can use [`game::market::deal`] in a
/// tick
///
/// [`game::market::deal`]: crate::game::market::deal
pub const MARKET_MAX_DEALS_PER_TICK: u32 = 10;

/// Maximum size (in UTF-16) units of the string contents allowed in
/// [`raw_memory::segments`]
///
/// [`raw_memory::segments`]: crate::raw_memory::segments
pub const MEMORY_SEGMENT_SIZE_LIMIT: u32 = 100 * 1024;

/// Maximum active memory segments to be readable next tick allowed in
/// [`raw_memory::set_active_segments`]
///
/// [`raw_memory::set_active_segments`]: crate::raw_memory::set_active_segments
pub const MEMORY_SEGMENT_ACTIVE_LIMIT: u32 = 10;

/// Maximum size (in UTF-16 units) of the string contents allowed in serialized
/// memory or [`raw_memory::set`]
///
/// [`raw_memory::set`]: crate::raw_memory::set
pub const MEMORY_SIZE_LIMIT: u32 = 2 * 1024 * 1024;

/// Fatigue points removed per effective [`Part::Move`] per tick.
///
/// [`Part::Move`]: crate::constants::Part::Move
pub const MOVE_POWER: u32 = 2;

/// Maximum hits of a [`PowerCreep`] per level.
///
/// [`PowerCreep`]: crate::objects::PowerCreep
pub const POWER_CREEP_HITS_PER_LEVEL: u32 = 1000;

/// Maximum length (in UTF-16 units) of names input to [`PowerCreep::create`] or
/// [`AccountPowerCreep::rename`]
///
/// [`PowerCreep::create`]: crate::objects::PowerCreep::create
/// [`AccountPowerCreep::rename`]: crate::objects::AccountPowerCreep::rename
pub const POWER_CREEP_NAME_MAX_LENGTH: u32 = 100;

/// The range of all ranged actions of [`Creep`] objects
///
/// [`Creep`]: crate::objects::Creep
pub const CREEP_RANGED_ACTION_RANGE: u8 = 3;

/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 1
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_1: u32 = 10;
/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 2
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_2: u32 = 4;
/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 3
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_3: u32 = 1;

/// The maximum size (500 KiB) of the serialized [`RoomVisual`] data for each
/// room
///
/// [`RoomVisual`]: crate::objects::RoomVisual
pub const ROOM_VISUAL_PER_ROOM_SIZE_LIMIT: u32 = 500 * 1024;

/// The maximum height/width of a [`Room`] in the game
///
/// [`Room`]: crate::objects::Room
pub const ROOM_SIZE: u8 = 50;

/// Owner username of hostile non-player structures and creeps which occupy
/// sector center rooms.
pub const SOURCE_KEEPER_USERNAME: &str = "Source Keeper";

/// Maximum length (in UTF-16 units) of the name of a [`StructureSpawn`], set
/// via the optional name parameter when creating a [`ConstructionSite`]
///
/// [`StructureSpawn`]: crate::objects::StructureSpawn
/// [`ConstructionSite`]: crate::objects::ConstructionSite
pub const SPAWN_NAME_MAX_LENGTH: u32 = 100;

/// Exponential scaling rate for [`StructureTerminal`] energy costs
///
/// Energy cost to send resources with a terminal is determined by the formula
/// ([source]):
///
/// ```js
/// Math.ceil(resource_amount * (1 - Math.exp(-range / 30)))
/// ```
///
/// [`StructureTerminal`]: crate::objects::StructureTerminal
/// [source]: https://github.com/screeps/engine/blob/c682c00b058de21e927c3a6c42fadb34c9745767/src/utils.js#L658
pub const TERMINAL_SEND_COST_SCALE: u32 = 30;
