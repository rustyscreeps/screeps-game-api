//! Plain data constants and functions returning plain data.
use super::types::{ResourceType, StructureType};

pub const CREEP_LIFE_TIME: u32 = 1500;
pub const CREEP_CLAIM_LIFE_TIME: u32 = 600;
pub const CREEP_CORPSE_RATE: f32 = 0.2;
pub const CREEP_PART_MAX_ENERGY: u32 = 125;

pub const CARRY_CAPACITY: u32 = 50;
pub const HARVEST_POWER: u32 = 2;
pub const HARVEST_MINERAL_POWER: u32 = 1;
pub const REPAIR_POWER: u32 = 100;
pub const DISMANTLE_POWER: u32 = 50;
pub const BUILD_POWER: u32 = 5;
pub const ATTACK_POWER: u32 = 30;
pub const UPGRADE_CONTROLLER_POWER: u32 = 1;
pub const RANGED_ATTACK_POWER: u32 = 10;
pub const HEAL_POWER: u32 = 12;
pub const RANGED_HEAL_POWER: u32 = 4;
pub const REPAIR_COST: f32 = 0.01;
pub const DISMANTLE_COST: f32 = 0.005;

// *_HITS constants translated as StructureType::initial_hits().

pub const RAMPART_DECAY_AMOUNT: u32 = 300;
pub const RAMPART_DECAY_TIME: u32 = 100;
pub const RAMPART_HITS: u32 = 1;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL2: u32 = 300_000;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL3: u32 = 1_000_000;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL4: u32 = 3_000_000;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL5: u32 = 5_000_000;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL6: u32 = 30_000_000;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL7: u32 = 100_000_000;
// Consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL8: u32 = 300_000_000;

/// Translates the `RAMPART_HITS_MAX` constant
#[inline]
pub fn rampart_hits_max(rcl: u32) -> u32 {
    match rcl {
        r if r < 2 => 0,
        2 => RAMPART_HITS_MAX_RCL2,
        3 => RAMPART_HITS_MAX_RCL3,
        4 => RAMPART_HITS_MAX_RCL4,
        5 => RAMPART_HITS_MAX_RCL5,
        6 => RAMPART_HITS_MAX_RCL6,
        7 => RAMPART_HITS_MAX_RCL7,
        8 | _ => RAMPART_HITS_MAX_RCL8,
    }
}

pub const ENERGY_REGEN_TIME: u32 = 300;
pub const ENERGY_DECAY: u32 = 1000;

pub const SPAWN_HITS: u32 = 5000;
pub const SPAWN_ENERGY_START: u32 = 300;
pub const SPAWN_ENERGY_CAPACITY: u32 = 300;
pub const CREEP_SPAWN_TIME: u32 = 3;
pub const SPAWN_RENEW_RATIO: f32 = 1.2;

pub const SOURCE_ENERGY_CAPACITY: u32 = 3000;
pub const SOURCE_ENERGY_NEUTRAL_CAPACITY: u32 = 1500;
pub const SOURCE_ENERGY_KEEPER_CAPACITY: u32 = 4000;

pub const WALL_HITS: u32 = 1;
pub const WALL_HITS_MAX: u32 = 300_000_000;

pub const EXTENSION_HITS: u32 = 1000;

/// Translates the `EXTENSION_ENERGY_CAPACITY` constant.
#[inline]
pub fn extension_energy_capacity(rcl: u32) -> u32 {
    match rcl {
        r if r < 7 => 50,
        7 => 100,
        8 | _ => 200,
    }
}

pub const ROAD_HITS: u32 = 5000;
pub const ROAD_WEAROUT: u32 = 1;
pub const ROAD_WEAROUT_POWER_CREEP: u32 = 100;
pub const ROAD_DECAY_AMOUNT: u32 = 100;
pub const ROAD_DECAY_TIME: u32 = 1000;

pub const LINK_HITS: u32 = 1000;
pub const LINK_CAPACITY: u32 = 800;
pub const LINK_COOLDOWN: u32 = 1;
pub const LINK_LOSS_RATIO: f32 = 0.03;

pub const STORAGE_CAPACITY: u32 = 1_000_000;
pub const STORAGE_HITS: u32 = 10_000;
pub const CONSTRUCTION_COST_ROAD_SWAMP_RATIO: u32 = 5;
pub const CONSTRUCTION_COST_ROAD_WALL_RATIO: u32 = 150;

/// Translates the `CONTROLLER_LEVELS` constant.
///
/// Returns `Some` for levels 1-7, `None` for all others.
#[inline]
pub fn controller_levels(current_rcl: u32) -> Option<u32> {
    match current_rcl {
        1 => Some(200),
        2 => Some(45_000),
        3 => Some(135_000),
        4 => Some(405_000),
        5 => Some(1_215_000),
        6 => Some(3_645_000),
        7 => Some(10_935_000),
        _ => None,
    }
}

/// Translates the `CONTROLLER_DOWNGRADE` constant.
///
/// Returns `Some` for levels 1-8, `None` for all others.
#[inline]
pub fn controller_downgrade(rcl: u32) -> Option<u32> {
    match rcl {
        1 => Some(20_000),
        2 => Some(10_000),
        3 => Some(20_000),
        4 => Some(40_000),
        5 => Some(80_000),
        6 => Some(120_000),
        7 => Some(150_000),
        8 => Some(200_000),
        _ => None,
    }
}

pub const CONTROLLER_DOWNGRADE_RESTORE: u32 = 100;
pub const CONTROLLER_DOWNGRADE_SAFEMODE_THRESHOLD: u32 = 5000;
pub const CONTROLLER_CLAIM_DOWNGRADE: u32 = 300;
pub const CONTROLLER_RESERVE: u32 = 1;
pub const CONTROLLER_RESERVE_MAX: u32 = 5000;
pub const CONTROLLER_MAX_UPGRADE_PER_TICK: u32 = 15;
pub const CONTROLLER_ATTACK_BLOCKED_UPGRADE: u32 = 1000;
pub const CONTROLLER_NUKE_BLOCKED_UPGRADE: u32 = 200;

pub const SAFE_MODE_DURATION: u32 = 20_000;
pub const SAFE_MODE_COOLDOWN: u32 = 50_000;
pub const SAFE_MODE_COST: u32 = 1000;

pub const TOWER_HITS: u32 = 3000;
pub const TOWER_CAPACITY: u32 = 1000;
pub const TOWER_ENERGY_COST: u32 = 10;
pub const TOWER_POWER_ATTACK: u32 = 600;
pub const TOWER_POWER_HEAL: u32 = 400;
pub const TOWER_POWER_REPAIR: u32 = 800;
pub const TOWER_OPTIMAL_RANGE: u32 = 5;
pub const TOWER_FALLOFF_RANGE: u32 = 20;
pub const TOWER_FALLOFF: f32 = 0.75;

pub const OBSERVER_HITS: u32 = 500;
pub const OBSERVER_RANGE: u32 = 10;

pub const POWER_BANK_HITS: u32 = 2_000_000;
pub const POWER_BANK_CAPACITY_MAX: u32 = 5000;
pub const POWER_BANK_CAPACITY_MIN: u32 = 500;
pub const POWER_BANK_CAPACITY_CRIT: f32 = 0.3;
pub const POWER_BANK_DECAY: u32 = 5000;
pub const POWER_BANK_HIT_BACK: f32 = 0.5;

pub const POWER_SPAWN_HITS: u32 = 1;
pub const POWER_SPAWN_ENERGY_CAPACITY: u32 = 5000;
pub const POWER_SPAWN_POWER_CAPACITY: u32 = 100;
pub const POWER_SPAWN_ENERGY_RATIO: u32 = 50;

pub const EXTRACTOR_HITS: u32 = 500;
pub const EXTRACTOR_COOLDOWN: u32 = 5;

pub const LAB_HITS: u32 = 500;
pub const LAB_MINERAL_CAPACITY: u32 = 3000;
pub const LAB_ENERGY_CAPACITY: u32 = 2000;
pub const LAB_BOOST_ENERGY: u32 = 20;
pub const LAB_BOOST_MINERAL: u32 = 30;
pub const LAB_UNBOOST_ENERGY: u32 = 0;
pub const LAB_UNBOOST_MINERAL: u32 = 15;

pub const LAB_REACTION_AMOUNT: u32 = 5;

pub const GCL_POW: f32 = 2.4;
pub const GCL_MULTIPLY: u32 = 1_000_000;
pub const GCL_NOVICE: u32 = 3;

pub const TERRAIN_MASK_WALL: u8 = 1;
pub const TERRAIN_MASK_SWAMP: u8 = 2;
pub const TERRAIN_MASK_LAVA: u8 = 4;

pub const MAX_CONSTRUCTION_SITES: u32 = 100;
pub const MAX_CREEP_SIZE: u32 = 50;

pub const MINERAL_REGEN_TIME: u32 = 50_000;

/// Translates the `MINERAL_MIN_AMOUNT` constant.
///
/// Currently always returns 35_000 for all minerals...
#[inline]
pub fn mineral_min_amount(mineral: ResourceType) -> Option<u32> {
    match mineral {
        ResourceType::Hydrogen => Some(35_000),
        ResourceType::Oxygen => Some(35_000),
        ResourceType::Lemergium => Some(35_000),
        ResourceType::Keanium => Some(35_000),
        ResourceType::Zynthium => Some(35_000),
        ResourceType::Utrium => Some(35_000),
        ResourceType::Catalyst => Some(35_000),
        _ => None,
    }
}

pub const MINERAL_RANDOM_FACTOR: u32 = 2;
pub const MINERAL_DENSITY_CHANGE: f32 = 0.05;

pub const DEPOSIT_EXHAUST_MULTIPLY: f32 = 0.001;
pub const DEPOSIT_EXHAUST_POW: f32 = 1.2;
pub const DEPOSIT_DECAY_TIME: u32 = 50_000;

pub const TERMINAL_HITS: u32 = 3000;
pub const TERMINAL_CAPACITY: u32 = 300_000;
pub const TERMINAL_SEND_COST: f32 = 0.1;
pub const TERMINAL_MIN_SEND: u32 = 100;
pub const TERMINAL_COOLDOWN: u32 = 10;

pub const CONTAINER_HITS: u32 = 250_000;
pub const CONTAINER_CAPACITY: u32 = 2000;
pub const CONTAINER_DECAY: u32 = 5000;
pub const CONTAINER_DECAY_TIME: u32 = 100;
pub const CONTAINER_DECAY_TIME_OWNED: u32 = 500;

pub const NUKER_HITS: u32 = 1000;
pub const NUKER_COOLDOWN: u32 = 100_000;
pub const NUKER_ENERGY_CAPACITY: u32 = 300_000;
pub const NUKER_GHODIUM_CAPACITY: u32 = 5000;
pub const NUKE_LAND_TIME: u32 = 50_000;
pub const NUKE_RANGE: u32 = 10;
pub const NUKE_DAMAGE_RANGE_0: u32 = 10_000_000;
pub const NUKE_DAMAGE_RANGE_2: u32 = 5_000_000;

pub const FACTORY_HITS: u32 = 1000;
pub const FACTORY_CAPACITY: u32 = 50_000;

pub const TOMBSTONE_DECAY_PER_PART: u32 = 5;
pub const TOMBSTONE_DECAY_POWER_CREEP: u32 = 500;

pub const RUIN_DECAY: u32 = 500;

#[inline]
pub fn ruin_decay_structures(structure_type: StructureType) -> Option<u32> {
    match structure_type {
        StructureType::PowerBank => Some(10),
        _ => None,
    }
}

pub const PORTAL_DECAY: u32 = 30_000;

// ORDER_SELL / ORDER_BUY defined in `src/game.rs`

pub const MARKET_FEE: f32 = 0.05;

pub const FLAGS_LIMIT: u32 = 10_000;
pub const PORTAL_UNSTABLE: u32 = 10 * 24 * 3600 * 1000;
pub const PORTAL_MIN_TIMEOUT: u32 = 12 * 24 * 3600 * 1000;
pub const PORTAL_MAX_TIMEOUT: u32 = 22 * 24 * 3600 * 1000;

pub const POWER_BANK_RESPAWN_TIME: u32 = 50000;

pub const INVADERS_ENERGY_GOAL: u32 = 100000;

pub const SYSTEM_USERNAME: &str = "Screeps";

pub const SIGN_PLANNED_AREA: &str = "A new Novice or Respawn Area is being planned somewhere \
     in this sector. Please make sure all important rooms are reserved.";

// EVENT_* constants in src/objects/impls/room.rs

pub const POWER_LEVEL_MULTIPLY: u32 = 1000;
pub const POWER_LEVEL_POW: u32 = 2;
pub const POWER_CREEP_SPAWN_COOLDOWN: u32 = 8 * 3600 * 1000;
pub const POWER_CREEP_DELETE_COOLDOWN: u32 = 24 * 3600 * 1000;
pub const POWER_CREEP_MAX_LEVEL: u32 = 25;
pub const POWER_CREEP_LIFE_TIME: u32 = 5000;

pub const INVADER_CORE_HITS: u32 = 100_000;
pub const INVADER_CORE_EXPAND_TIME: u32 = 2000;

#[inline]
pub fn invader_core_expand_time(core_level: u32) -> Option<u32> {
    match core_level {
        1 => Some(4000),
        2 => Some(3500),
        3 => Some(3000),
        4 => Some(2500),
        5 => Some(2000),
        _ => None,
    }
}

pub const INVADER_CORE_CONTROLLER_POWER: u32 = 2;
pub const INVADER_CORE_CONTROLLER_DOWNGRADE: u32 = 5000;

#[inline]
pub fn invader_core_creep_spawn_time(core_level: u32) -> Option<u32> {
    match core_level {
        0 | 1 => Some(0),
        2 => Some(6),
        3 => Some(3),
        4 => Some(2),
        5 => Some(1),
        _ => None,
    }
}

pub const STRONGHOLD_DECAY_TICKS: u32 = 75_000;

#[inline]
pub fn stronghold_rampart_hits(core_level: u32) -> Option<u32> {
    match core_level {
        0 => Some(0),
        1 => Some(50_000),
        2 => Some(200_000),
        3 => Some(500_000),
        4 => Some(1_000_000),
        5 => Some(2_000_000),
        _ => None,
    }
}
