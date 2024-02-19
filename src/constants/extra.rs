//! Additional constants representing internal game mechanics that aren't
//! included in the game's constants

/// Percentage of energy spent on construction that is left in a [`Resource`] if
/// the construction site is destroyed by [`ConstructionSite::remove`], or from
/// being stepped on by a hostile creep
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/room/remove-construction-site.js#L18)
///
/// [`Resource`]: crate::objects::Resource
/// [`ConstructionSite::remove`]: crate::objects::ConstructionSite::remove
pub const CONSTRUCTION_SITE_STOMP_RATIO: f32 = 0.5;

/// Maximum length (in UTF-16 units) of input to [`Creep::sign_controller`]
///
/// [Code reference](https://github.com/screeps/common/blob/e3e283ffa5d34f9f4e8d474d998065c26025d4bb/lib/system.js#L74)
///
/// [`Creep::sign_controller`]: crate::objects::Creep::sign_controller
pub const CONTROLLER_SIGN_MAX_LENGTH: u32 = 100;

/// Percentage of progress toward next level controllers retain when downgraded
///
/// After a downgrade, the controller loses its previous progress toward the
/// next level, and has its progress set to 90% of the amount needed to upgrade
/// back to the level that it just downgraded from. [Code reference](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/processor/intents/controllers/tick.js#L66)
pub const CONTROLLER_DOWNGRADE_PROGRESS_RATIO: f32 = 0.9;

/// Maximum amount of CPU that can be accumulated in [`game::cpu::bucket`] per
/// shard
///
/// [Code reference](https://github.com/screeps/driver/blob/97a9e51d124c7170429caa1621096f0f4d888d72/lib/index.js#L26)
///
/// [`game::cpu::bucket`]: crate::game::cpu::bucket
pub const CPU_BUCKET_MAX: u32 = 10_000;

/// Time (in milliseconds) that the [`cpu::set_shard_limits`] function needs to
/// cool down between calls
///
/// [Documentation reference](https://github.com/screeps/docs/blob/06a2a1dfbab14b9c4fdffe7a70ee2f55ea6b430c/api/source/Game.md?plain=1#L262)
/// (code is closed-source)
///
/// [`cpu::set_shard_limits`]: crate::game::cpu::set_shard_limits
pub const CPU_SET_SHARD_LIMITS_COOLDOWN: u32 = 12 * 3600 * 1000;

/// Maximum value of [`cpu::tick_limit`], the amount of CPU time (in
/// milliseconds) available this tick, including available time from the bucket
/// ([`game::cpu::bucket`])
///
/// [Code reference](https://github.com/screeps/driver/blob/97a9e51d124c7170429caa1621096f0f4d888d72/lib/index.js#L25)
///
/// [`cpu::tick_limit`]: crate::game::cpu::tick_limit
/// [`game::cpu::bucket`]: crate::game::cpu::bucket
pub const CPU_TICK_LIMIT_MAX: u32 = 500;

/// [`Creep::hits_max`] gained for each [`Part`] in a given creep's
/// [`Creep::body`]
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/spawns/create-creep.js#L89)
///
/// [`Creep::hits_max`]: crate::objects::Creep::hits_max
/// [`Part`]: crate::constants::Part
/// [`Creep::body`]: crate::objects::Creep::body
pub const CREEP_HITS_PER_PART: u32 = 100;

/// Maximum length (in UTF-16 units) of the string input to
/// [`StuctureSpawn::spawn_creep`] for the name of a [`Creep`]
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/structures.js#L1069)
///
/// [`StuctureSpawn::spawn_creep`]: crate::objects::StructureSpawn::spawn_creep
/// [`Creep`]: crate::objects::Creep
pub const CREEP_NAME_MAX_LENGTH: u32 = 100;

/// Maximum length (in UTF-16 units) of string input to [`Creep::say`] and
/// [`PowerCreep::say`]
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/creeps/say.js#L19)
///
/// [`Creep::say`]: crate::objects::Creep::say
/// [`PowerCreep::say`]: crate::objects::PowerCreep::say
pub const CREEP_SAY_MAX_LENGTH: u32 = 10;

/// Maximum length (in UTF-16 units) of names of [`Flag`] objects
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/rooms.js#L1015)
///
/// [`Flag`]: crate::objects::Flag
pub const FLAG_NAME_MAX_LENGTH: u32 = 100;

/// The cost of a single 'intent' (in milliseconds), a CPU penalty charged for
/// most successful API calls which change the game state ([`Creep::pull`],
/// [`Creep::say`], and [`PowerCreep::say`] are excepted)
///
/// [Code reference](https://github.com/screeps/driver/blob/97a9e51d124c7170429caa1621096f0f4d888d72/lib/runtime/runtime.js#L52)
///
/// [`Creep::pull`]: crate::objects::Creep::pull
/// [`Creep::say`]: crate::objects::Creep::say
/// [`PowerCreep::say`]: crate::objects::PowerCreep::say
pub const INTENT_CPU_COST: f64 = 0.2;

/// Maximum size (in UTF-16 units) of data input to
/// [`inter_shard_memory::set_local`]
///
/// [Documentation reference](https://github.com/screeps/docs/blob/06a2a1dfbab14b9c4fdffe7a70ee2f55ea6b430c/api/source/InterShardMemory.md?plain=1#L7)
/// (code is closed-source)
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
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/labs/run-reaction.js#L26)
///
/// [`StructureLab`]: crate::objects::StructureLab
/// [`StructureLab::run_reaction`]: crate::objects::StructureLab::run_reaction
/// [`StructureLab::reverse_reaction`]: crate::objects::StructureLab::reverse_reaction
pub const LAB_REACTION_RANGE: u32 = 2;

/// The maximum size (in UTF-16 units) of the serialized [`MapVisual`] data
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/console.js#L42-L45)
///
/// [`MapVisual`]: crate::objects::MapVisual
pub const MAP_VISUAL_SIZE_LIMIT: u32 = 1000 * 1024;

/// The maximum number of times that you can use [`game::market::deal`] in a
/// tick
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/market.js#L149)
///
/// [`game::market::deal`]: crate::game::market::deal
pub const MARKET_MAX_DEALS_PER_TICK: u32 = 10;

/// Maximum size (in UTF-16 units) of the string contents allowed in
/// [`raw_memory::segments`]
///
/// [Code reference](https://github.com/screeps/driver/blob/97a9e51d124c7170429caa1621096f0f4d888d72/lib/runtime/runtime.js#L255)
///
/// [`raw_memory::segments`]: crate::raw_memory::segments
pub const MEMORY_SEGMENT_SIZE_LIMIT: u32 = 100 * 1024;

/// Maximum active memory segments to be readable next tick allowed in
/// [`raw_memory::set_active_segments`]
///
/// [Code reference](https://github.com/screeps/driver/blob/97a9e51d124c7170429caa1621096f0f4d888d72/lib/runtime/runtime.js#L125)
///
/// [`raw_memory::set_active_segments`]: crate::raw_memory::set_active_segments
pub const MEMORY_SEGMENT_ACTIVE_LIMIT: u32 = 10;

/// Maximum size (in UTF-16 units) of the string contents allowed in serialized
/// memory or [`raw_memory::set`]
///
/// [Code reference](https://github.com/screeps/driver/blob/97a9e51d124c7170429caa1621096f0f4d888d72/lib/runtime/runtime.js#L104)
///
/// [`raw_memory::set`]: crate::raw_memory::set
pub const MEMORY_SIZE_LIMIT: u32 = 2 * 1024 * 1024;

/// Fatigue points added per body points that generate them when stepping onto a
/// tile with a [`StructureRoad`]
///
/// [`StructureRoad`]: crate::objects::StructureRoad
pub const MOVE_COST_ROAD: u32 = 1;

/// Fatigue points added per body points that generate them when stepping onto a
/// tile with [`Terrain::Plain`]
///
/// [`Terrain::Plain`]: crate::constants::Terrain::Plain
pub const MOVE_COST_PLAIN: u32 = 2;

/// Fatigue points added per body points that generate them when stepping onto a
/// tile with [`Terrain::Swamp`]
///
/// [`Terrain::Swamp`]: crate::constants::Terrain::Swamp
pub const MOVE_COST_SWAMP: u32 = 10;

/// Fatigue points removed per effective [`Part::Move`] per tick
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/movement.js#L204)
///
/// [`Part::Move`]: crate::constants::Part::Move
pub const MOVE_POWER: u32 = 2;

/// Maximum length (in UTF-16 units) of message content sent to
/// [`game::notify`].
///
/// Note that the official documentation reflects a limit of 1000, but a limit
/// of 500 is enforced in the driver (truncating to that length if a longer
/// string is input).
///
/// [Code reference](https://github.com/screeps/driver/blob/e691bd3ee843cb12ac4bedc68397b2b92709f622/lib/index.js#L208)
///
/// [`game::notify`]: crate::game::notify
pub const NOTIFY_MAX_LENGTH: u32 = 500;

/// Maximum carry capacity of a [`PowerCreep`] per level
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/global-intents/power/upgradePowerCreep.js#L46)
///
/// [`PowerCreep`]: crate::objects::PowerCreep
pub const POWER_CREEP_CARRY_CAPACITY_PER_LEVEL: u32 = 100;

/// Maximum hits of a [`PowerCreep`] per level
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/global-intents/power/upgradePowerCreep.js#L47)
///
/// [`PowerCreep`]: crate::objects::PowerCreep
pub const POWER_CREEP_HITS_PER_LEVEL: u32 = 1000;

/// Maximum length (in UTF-16 units) of names input to [`PowerCreep::create`] or
/// [`AccountPowerCreep::rename`]
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/power-creeps.js#L396)
///
/// [`PowerCreep::create`]: crate::objects::PowerCreep::create
/// [`AccountPowerCreep::rename`]: crate::objects::AccountPowerCreep::rename
pub const POWER_CREEP_NAME_MAX_LENGTH: u32 = 100;

/// The range of all ranged actions of [`Creep`] objects
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/creeps/rangedAttack.js#L21)
///
/// [`Creep`]: crate::objects::Creep
pub const CREEP_RANGED_ACTION_RANGE: u8 = 3;

/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 1
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/creeps/rangedMassAttack.js#L32)
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_1: u32 = 10;
/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 2
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/creeps/rangedMassAttack.js#L32)
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_2: u32 = 4;
/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 3
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/creeps/rangedMassAttack.js#L32)
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_3: u32 = 1;

/// The maximum size (500 KiB) of the serialized [`RoomVisual`] data for each
/// room
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/console.js#L42-L45)
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
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/rooms.js#L1042)
///
/// [`StructureSpawn`]: crate::objects::StructureSpawn
/// [`ConstructionSite`]: crate::objects::ConstructionSite
pub const SPAWN_NAME_MAX_LENGTH: u32 = 100;

/// Exponential scaling rate for [`StructureTerminal`] energy costs
///
/// Energy cost to send resources with a terminal is determined by the formula:
///
/// ```js
/// Math.ceil(resource_amount * (1 - Math.exp(-range / 30)))
/// ```
///
/// [Code reference](https://github.com/screeps/engine/blob/c682c00b058de21e927c3a6c42fadb34c9745767/src/utils.js#L658)
///
/// [`StructureTerminal`]: crate::objects::StructureTerminal
pub const TERMINAL_SEND_COST_SCALE: u32 = 30;
