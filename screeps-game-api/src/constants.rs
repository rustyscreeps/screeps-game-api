//! Constants, most copied from [the game constants].
//!
//! Last updated on 2019-07-13, `068343753adf090fd1991944d2292be9e236b7dd` from
//! <https://github.com/screeps/common/commits/master/lib/constants.js>.
//!
//! Currently missing:
//! - FIND_DROPPED_ENERGY (deprecated in Screeps)
//! - OBSTACLE_OBJECT_TYPES
//! - WORLD_WIDTH / WORLD_HEIGHT (deprecated in Screeps)
//! - CONTROLLER_STRUCTURES
//! - REACTIONS
//! - BOOSTS
//! - POWER_INFO
//!
//! [the game constants]: https://github.com/screeps/common/blob/master/lib/constants.js
use std::fmt;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use stdweb::{Number, Reference, Value, __js_deserializable_serde_boilerplate, js_deserializable};

use crate::{
    objects::RoomObject,
    traits::{FromExpectedType, TryFrom, TryInto},
    ConversionError,
};

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(i32)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
    Other = 42,
}

impl ReturnCode {
    /// Turns this return code into a result.
    ///
    /// `ReturnCode::Ok` is turned into `Result::Ok`, all other codes are turned
    /// into `Result::Err(code)`
    pub fn as_result(self) -> Result<(), Self> {
        match self {
            ReturnCode::Ok => Ok(()),
            other => Err(other),
        }
    }
}

impl TryFrom<i32> for ReturnCode {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Self::from_i32(v).ok_or(v)
    }
}

impl TryFrom<Number> for ReturnCode {
    type Error = <ReturnCode as TryFrom<Value>>::Error;
    fn try_from(v: Number) -> Result<Self, Self::Error> {
        Value::Number(v).try_into()
    }
}

js_deserializable!(ReturnCode);

pub unsafe trait FindConstant {
    type Item: FromExpectedType<Reference>;

    fn find_code(&self) -> i32;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum FindObject {
    Creeps = 101,
    MyCreeps = 102,
    HostileCreeps = 103,
    SourcesActive = 104,
    Sources = 105,
    DroppedResources = 106,
    Structures = 107,
    MyStructures = 108,
    HostileStructures = 109,
    Flags = 110,
    ConstructionSites = 111,
    MySpawns = 112,
    HostileSpawns = 113,
    MyConstructionSites = 114,
    HostileConstructionSites = 115,
    Minerals = 116,
    Nukes = 117,
    Tombstones = 118,
}

unsafe impl FindConstant for FindObject {
    type Item = RoomObject;

    fn find_code(&self) -> i32 {
        *self as i32
    }
}

pub mod find {
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;
    use serde_repr::Deserialize_repr;

    use super::FindConstant;
    use crate::{
        objects::{
            ConstructionSite, Creep, Flag, Mineral, Nuke, OwnedStructure, PowerCreep, Resource,
            RoomPosition, Source, Structure, StructureSpawn, Tombstone,
        },
        traits::TryFrom,
    };

    #[derive(Copy, Clone, Debug, FromPrimitive, Deserialize_repr, PartialEq, Eq, Hash)]
    #[repr(i32)]
    pub enum Exit {
        Top = 1,
        Right = 3,
        Bottom = 5,
        Left = 7,
        All = 10,
    }

    impl Exit {
        pub fn top() -> Self {
            Exit::Top
        }

        pub fn right() -> Self {
            Exit::Right
        }

        pub fn bottom() -> Self {
            Exit::Bottom
        }

        pub fn left() -> Self {
            Exit::Left
        }

        pub fn all() -> Self {
            Exit::All
        }
    }

    impl TryFrom<i32> for Exit {
        type Error = i32;
        fn try_from(v: i32) -> Result<Self, Self::Error> {
            Self::from_i32(v).ok_or(v)
        }
    }

    unsafe impl FindConstant for Exit {
        type Item = RoomPosition;

        fn find_code(&self) -> i32 {
            *self as i32
        }
    }

    typesafe_find_constants! {
        CREEPS, 101, Creep;
        MY_CREEPS, 102, Creep;
        HOSTILE_CREEPS, 103, Creep;
        SOURCES_ACTIVE, 104, Source;
        SOURCES, 105, Source;
        DROPPED_RESOURCES, 106, Resource;
        STRUCTURES, 107, Structure;
        MY_STRUCTURES, 108, OwnedStructure;
        HOSTILE_STRUCTURES, 109, OwnedStructure;
        FLAGS, 110, Flag;
        CONSTRUCTION_SITES, 111, ConstructionSite;
        MY_SPAWNS, 112, StructureSpawn;
        HOSTILE_SPAWNS, 113, StructureSpawn;
        MY_CONSTRUCTION_SITES, 114, ConstructionSite;
        HOSTILE_CONSTRUCTION_SITES, 115, ConstructionSite;
        MINERALS, 116, Mineral;
        NUKES, 117, Nuke;
        TOMBSTONES, 118, Tombstone;
        POWER_CREEPS, 119, PowerCreep;
        MY_POWER_CREEPS, 120, PowerCreep;
        HOSTILE_POWER_CREEPS, 121, PowerCreep;
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u32)]
pub enum Direction {
    Top = 1,
    TopRight = 2,
    Right = 3,
    BottomRight = 4,
    Bottom = 5,
    BottomLeft = 6,
    Left = 7,
    TopLeft = 8,
}

js_deserializable!(Direction);

impl ::std::ops::Neg for Direction {
    type Output = Direction;

    /// Negates this direction. Top goes to Bottom, TopRight goes to BottomLeft,
    /// etc.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::Direction::*;
    ///
    /// assert_eq!(-Top, Bottom);
    /// assert_eq!(-BottomRight, TopLeft);
    /// assert_eq!(-Left, Right);
    /// ```
    fn neg(self) -> Direction {
        use crate::Direction::*;

        match self {
            Top => Bottom,
            TopRight => BottomLeft,
            Right => Left,
            BottomRight => TopLeft,
            Bottom => Top,
            BottomLeft => TopRight,
            Left => Right,
            TopLeft => BottomRight,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Direction::Top => "↑",
            Direction::TopRight => "↗",
            Direction::Right => "→",
            Direction::BottomRight => "↘",
            Direction::Bottom => "↓",
            Direction::BottomLeft => "↙",
            Direction::Left => "←",
            Direction::TopLeft => "↖",
        };
        f.write_str(ch)
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(u32)]
pub enum Color {
    Red = 1,
    Purple = 2,
    Blue = 3,
    Cyan = 4,
    Green = 5,
    Yellow = 6,
    Orange = 7,
    Brown = 8,
    Grey = 9,
    White = 10,
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        c as u32
    }
}

js_deserializable!(Color);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum Terrain {
    Plain = 0,
    Wall = TERRAIN_MASK_WALL,
    Swamp = TERRAIN_MASK_SWAMP,
}

impl TryFrom<Value> for Terrain {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let v = match v {
            Value::String(s) => match &*s {
                "plain" => Terrain::Plain,
                "wall" => Terrain::Wall,
                "swamp" => Terrain::Swamp,
                _ => panic!("unknown terrain string {}", s),
            },
            other => match u32::try_from(other)? {
                0 => Terrain::Plain,
                // TERRAIN_MASK_WALL
                1 => Terrain::Wall,
                // TERRAIN_MASK_SWAMP
                2 => Terrain::Swamp,
                // TERRAIN_MASK_WALL | TERRAIN_MASK_SWAMP
                3 => Terrain::Wall,
                x => panic!("unknown terrain encoded integer {}", x),
            },
        };
        Ok(v)
    }
}

/// Internal enum representing each LOOK_* constant.
///
/// It's recommended to use the constants in the `look` module instead for type
/// safety.
///
/// In fact, I don't believe this can be used at all without resorting to
/// manually including JS code.
///
/// To use in JS: `__look_num_to_str(@{look as u32})` function
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[doc(hidden)]
#[serde(rename_all = "camelCase")]
pub enum Look {
    #[serde(rename = "creep")]
    Creeps = 0,
    #[serde(rename = "energy")]
    Energy = 1,
    #[serde(rename = "resource")]
    Resources = 2,
    #[serde(rename = "source")]
    Sources = 3,
    #[serde(rename = "mineral")]
    Minerals = 4,
    #[serde(rename = "structure")]
    Structures = 5,
    #[serde(rename = "flag")]
    Flags = 6,
    #[serde(rename = "constructionSite")]
    ConstructionSites = 7,
    #[serde(rename = "nuke")]
    Nukes = 8,
    #[serde(rename = "terrain")]
    Terrain = 9,
    #[serde(rename = "tombstone")]
    Tombstones = 10,
    #[serde(rename = "powerCreep")]
    PowerCreeps = 11,
}

js_deserializable!(Look);

pub unsafe trait LookConstant {
    type Item;

    fn convert_and_check_items(reference: Value) -> Vec<Self::Item>;

    fn look_code(&self) -> Look;
}

pub mod look {
    use super::{Look, LookConstant, Terrain};
    use crate::{
        objects::{
            ConstructionSite, Creep, Flag, Mineral, Nuke, PowerCreep, Resource, Source, Structure,
            Tombstone,
        },
        traits::{IntoExpectedType, TryInto},
    };

    typesafe_look_constants! {
        CREEPS, Look::Creeps, Creep, IntoExpectedType::into_expected_type;
        ENERGY, Look::Energy, Resource, IntoExpectedType::into_expected_type;
        RESOURCES, Look::Resources, Resource, IntoExpectedType::into_expected_type;
        SOURCES, Look::Sources, Source, IntoExpectedType::into_expected_type;
        MINERALS, Look::Minerals, Mineral, IntoExpectedType::into_expected_type;
        STRUCTURES, Look::Structures, Structure, IntoExpectedType::into_expected_type;
        FLAGS, Look::Flags, Flag, IntoExpectedType::into_expected_type;
        CONSTRUCTION_SITES, Look::ConstructionSites, ConstructionSite,
            IntoExpectedType::into_expected_type;
        NUKES, Look::Nukes, Nuke, IntoExpectedType::into_expected_type;
        TERRAIN, Look::Terrain, Terrain, TryInto::try_into;
        TOMBSTONES, Look::Tombstones, Tombstone, IntoExpectedType::into_expected_type;
        POWER_CREEPS, Look::PowerCreeps, PowerCreep, IntoExpectedType::into_expected_type;
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, FromPrimitive)]
#[serde(rename_all = "snake_case")]
pub enum Part {
    Move = 0,
    Work = 1,
    Carry = 2,
    Attack = 3,
    RangedAttack = 4,
    Tough = 5,
    Heal = 6,
    Claim = 7,
}

impl Part {
    /// Translates the `BODYPART_COST` constant.
    pub fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            Part::Claim => 600,
        }
    }
}

js_deserializable!(Part);

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
pub const SPAWN_RENEW_RATION: f32 = 1.2;

pub const SOURCE_ENERGY_CAPACITY: u32 = 3000;
pub const SOURCE_ENERGY_NEUTRAL_CAPACITY: u32 = 1500;
pub const SOURCE_ENERGY_KEEPER_CAPACITY: u32 = 4000;

pub const WALL_HITS: u32 = 1;
pub const WALL_HITS_MAX: u32 = 300_000_000;

pub const EXTENSION_HITS: u32 = 1000;

/// Translates the `EXTENSION_ENERGY_CAPACITY` constant.
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
pub const LINK_LOSS_RATION: f32 = 0.03;

pub const STORAGE_CAPACITY: u32 = 1_000_000;
pub const STORAGE_HITS: u32 = 10_000;

/// Translates `STRUCTURE_*` constants.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StructureType {
    Spawn = 0,
    Extension = 1,
    Road = 2,
    Wall = 3,
    Rampart = 4,
    KeeperLair = 5,
    Portal = 6,
    Controller = 7,
    Link = 8,
    Storage = 9,
    Tower = 10,
    Observer = 11,
    PowerBank = 12,
    PowerSpawn = 13,
    Extractor = 14,
    Lab = 15,
    Terminal = 16,
    Container = 17,
    Nuker = 18,
}

impl StructureType {
    /// Translates the `CONSTRUCTION_COST` constant.
    pub fn construction_cost(self) -> Option<u32> {
        use self::StructureType::*;

        let cost = match self {
            Spawn => 15_000,
            Extension => 3_000,
            Road => 300,
            Wall => 1,
            Rampart => 1,
            Link => 5_000,
            Storage => 30_000,
            Tower => 5_000,
            Observer => 8_000,
            PowerSpawn => 100_000,
            Extractor => 5_000,
            Lab => 50_000,
            Terminal => 100_000,
            Container => 5_000,
            Nuker => 100_000,
            KeeperLair | PowerBank | Portal | Controller => return None,
        };
        Some(cost)
    }

    pub fn initial_hits(self) -> Option<u32> {
        use self::StructureType::*;

        let hits = match self {
            Spawn => SPAWN_HITS,
            Extension => EXTENSION_HITS,
            Road => ROAD_HITS,
            Wall => WALL_HITS,
            Rampart => RAMPART_HITS,
            Link => LINK_HITS,
            Storage => STORAGE_HITS,
            Tower => TOWER_HITS,
            Observer => OBSERVER_HITS,
            PowerBank => POWER_BANK_HITS,
            PowerSpawn => POWER_SPAWN_HITS,
            Extractor => EXTENSION_HITS,
            Lab => LAB_HITS,
            Terminal => TOWER_HITS,
            Container => CONTAINER_HITS,
            Nuker => NUKER_HITS,
            KeeperLair | Portal | Controller => return None,
        };
        Some(hits)
    }
}

js_deserializable!(StructureType);

pub const CONSTRUCTION_COST_ROAD_SWAMP_RATIO: u32 = 5;
pub const CONSTRUCTION_COST_ROAD_WALL_RATIO: u32 = 150;

/// Translates the `CONTROLLER_LEVELS` constant.
///
/// Accepts levels 1-7.
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
/// Accepts levels 1-7.
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

pub const TERRAIN_MASK_WALL: u32 = 1;
pub const TERRAIN_MASK_SWAMP: u32 = 2;
pub const TERRAIN_MASK_LAVA: u32 = 4;

pub const MAX_CONSTRUCTION_SITES: u32 = 100;
pub const MAX_CREEP_SIZE: u32 = 50;

pub const MINERAL_REGEN_TIME: u32 = 50_000;

/// Translates the `MINERAL_MIN_AMOUNT` constant.
///
/// Currently always returns 35_000 for all minerals...
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

/// Translates the `DENSITY_*` constants.
#[repr(u32)]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum Density {
    Low = 1,
    Moderate = 2,
    High = 3,
    Ultra = 4,
}

js_deserializable!(Density);

impl Density {
    /// Translates the `MINERAL_DENSITY` constant.
    pub fn amount(self) -> u32 {
        match self {
            Density::Low => 15_000,
            Density::Moderate => 35_000,
            Density::High => 70_000,
            Density::Ultra => 100_000,
        }
    }

    /// Translates the `MINERAL_DENSITY_PROBABILITY` constant.
    ///
    /// All values are between 0 and 1, but the total is roughly `2.5` so these
    /// aren't percentages.
    pub fn probabilitiy(self) -> f32 {
        match self {
            Density::Low => 0.1,
            Density::Moderate => 0.5,
            Density::High => 0.9,
            Density::Ultra => 1.0,
        }
    }
}

pub const MINERAL_DENSITY_CHANGE: f32 = 0.05;

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

pub const TOMBSTONE_DECAY_PER_PART: u32 = 5;
pub const TOMBSTONE_DECAY_POWER_CREEP: u32 = 500;

pub const PORTAL_DECAY: u32 = 30_000;

// ORDER_SELL / ORDER_BUY defined in `src/game.rs`

pub const MARKET_FEE: f32 = 0.05;

pub const FLAGS_LIMIT: u32 = 10_000;

/// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(u32)]
pub enum IntershardResourceType {
    #[serde(rename = "token")]
    SubscriptionToken = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceType {
    /// `"energy"`
    #[serde(rename = "energy")]
    Energy = 1,
    /// `"power"`
    #[serde(rename = "power")]
    Power = 2,
    /// `"H"`
    #[serde(rename = "H")]
    Hydrogen = 3,
    /// `"O"`
    #[serde(rename = "O")]
    Oxygen = 4,
    /// `"U"`
    #[serde(rename = "U")]
    Utrium = 5,
    /// `"L"`
    #[serde(rename = "L")]
    Lemergium = 6,
    /// `"K"`
    #[serde(rename = "K")]
    Keanium = 7,
    /// `"Z"`
    #[serde(rename = "Z")]
    Zynthium = 8,
    /// `"X"`
    #[serde(rename = "X")]
    Catalyst = 9,
    /// `"G"`
    #[serde(rename = "G")]
    Ghodium = 10,
    /// `"OH"`
    #[serde(rename = "OH")]
    Hydroxide = 11,
    /// `"ZK"`
    #[serde(rename = "ZK")]
    ZynthiumKeanite = 12,
    /// `"UL"`
    #[serde(rename = "UL")]
    UtriumLemergite = 13,
    /// `"UH"`
    #[serde(rename = "UH")]
    UtriumHydride = 14,
    /// `"UO"`
    #[serde(rename = "UO")]
    UtriumOxide = 15,
    /// `"KH"`
    #[serde(rename = "KH")]
    KeaniumHydride = 16,
    /// `"KO"`
    #[serde(rename = "KO")]
    KeaniumOxide = 17,
    /// `"LH"`
    #[serde(rename = "LH")]
    LemergiumHydride = 18,
    /// `"LO"`
    #[serde(rename = "LO")]
    LemergiumOxide = 19,
    /// `"ZH"`
    #[serde(rename = "ZH")]
    ZynthiumHydride = 20,
    /// `"ZO"`
    #[serde(rename = "ZO")]
    ZynthiumOxide = 21,
    /// `"GH"`
    #[serde(rename = "GH")]
    GhodiumHydride = 22,
    /// `"GO"`
    #[serde(rename = "GO")]
    GhodiumOxide = 23,
    /// `"UH2O"`
    #[serde(rename = "UH2O")]
    UtriumAcid = 24,
    /// `"UHO2"`
    #[serde(rename = "UHO2")]
    UtriumAlkalide = 25,
    /// `"KH2O"`
    #[serde(rename = "KH2O")]
    KeaniumAcid = 26,
    /// `"KHO2"`
    #[serde(rename = "KHO2")]
    KeaniumAlkalide = 27,
    /// `"LH2O"`
    #[serde(rename = "LH2O")]
    LemergiumAcid = 28,
    /// `"LHO2"`
    #[serde(rename = "LHO2")]
    LemergiumAlkalide = 29,
    /// `"ZH2O"`
    #[serde(rename = "ZH2O")]
    ZynthiumAcid = 30,
    /// `"ZHO2"`
    #[serde(rename = "ZHO2")]
    ZynthiumAlkalide = 31,
    /// `"GH2O"`
    #[serde(rename = "GH2O")]
    GhodiumAcid = 32,
    /// `"GHO2"`
    #[serde(rename = "GHO2")]
    GhodiumAlkalide = 33,
    /// `"XUH2O"`
    #[serde(rename = "XUH2O")]
    CatalyzedUtriumAcid = 34,
    /// `"XUHO2"`
    #[serde(rename = "XUHO2")]
    CatalyzedUtriumAlkalide = 35,
    /// `"XKH2O"`
    #[serde(rename = "XKH2O")]
    CatalyzedKeaniumAcid = 36,
    /// `"XKHO2"`
    #[serde(rename = "XKHO2")]
    CatalyzedKeaniumAlkalide = 37,
    /// `"XLH2O"`
    #[serde(rename = "XLH2O")]
    CatalyzedLemergiumAcid = 38,
    /// `"XLHO2"`
    #[serde(rename = "XLHO2")]
    CatalyzedLemergiumAlkalide = 39,
    /// `"XZH2O"`
    #[serde(rename = "XZH2O")]
    CatalyzedZynthiumAcid = 40,
    /// `"XZHO2"`
    #[serde(rename = "XZHO2")]
    CatalyzedZynthiumAlkalide = 41,
    /// `"XGH2O"`
    #[serde(rename = "XGH2O")]
    CatalyzedGhodiumAcid = 42,
    /// `"XGHO2"`
    #[serde(rename = "XGHO2")]
    CatalyzedGhodiumAlkalide = 43,
    /// `"ops"`
    #[serde(rename = "ops")]
    Ops = 44,
}

impl ResourceType {
    /// Translates the `REACTION_TIME` constant.
    pub fn reaction_time(self) -> Option<u32> {
        use crate::ResourceType::*;
        let time = match self {
            Energy | Power | Hydrogen | Oxygen | Utrium | Lemergium | Keanium | Zynthium
            | Catalyst | Ops => return None,
            // OH: 20,
            Hydroxide => 20,
            // ZK: 5,
            ZynthiumKeanite => 5,
            // UL: 5,
            UtriumLemergite => 5,
            // G: 5,
            Ghodium => 5,
            // UH: 10,
            UtriumHydride => 10,
            // UH2O: 5,
            UtriumAcid => 5,
            // XUH2O: 60,
            CatalyzedUtriumAcid => 60,
            // UO: 10,
            UtriumOxide => 10,
            // UHO2: 5,
            UtriumAlkalide => 5,
            // XUHO2: 60,
            CatalyzedUtriumAlkalide => 60,
            // KH: 10,
            KeaniumHydride => 10,
            // KH2O: 5,
            KeaniumAcid => 5,
            // XKH2O: 60,
            CatalyzedKeaniumAcid => 60,
            // KO: 10,
            KeaniumOxide => 10,
            // KHO2: 5,
            KeaniumAlkalide => 5,
            // XKHO2: 60,
            CatalyzedKeaniumAlkalide => 60,
            // LH: 15,
            LemergiumHydride => 15,
            // LH2O: 10,
            LemergiumAcid => 10,
            // XLH2O: 65,
            CatalyzedLemergiumAcid => 65,
            // LO: 10,
            LemergiumOxide => 10,
            // LHO2: 5,
            LemergiumAlkalide => 5,
            // XLHO2: 60,
            CatalyzedLemergiumAlkalide => 60,
            // ZH: 20,
            ZynthiumHydride => 50,
            // ZH2O: 40,
            ZynthiumAcid => 100,
            // XZH2O: 160,
            CatalyzedZynthiumAcid => 180,
            // ZO: 10,
            ZynthiumOxide => 10,
            // ZHO2: 5,
            ZynthiumAlkalide => 5,
            // XZHO2: 60,
            CatalyzedZynthiumAlkalide => 80,
            // GH: 10,
            GhodiumHydride => 10,
            // GH2O: 15,
            GhodiumAcid => 15,
            // XGH2O: 80,
            CatalyzedGhodiumAcid => 80,
            // GO: 10,
            GhodiumOxide => 10,
            // GHO2: 30,
            GhodiumAlkalide => 30,
            // XGHO2: 150,
            CatalyzedGhodiumAlkalide => 150,
        };
        Some(time)
    }
}

js_deserializable!(ResourceType);

pub const PORTAL_UNSTABLE: u32 = 10 * 24 * 3600 * 1000;
pub const PORTAL_MIN_TIMEOUT: u32 = 12 * 24 * 3600 * 1000;
pub const PORTAL_MAX_TIMEOUT: u32 = 22 * 24 * 3600 * 1000;

pub const POWER_BANK_RESPAWN_TIME: u32 = 50000;

pub const INVADERS_ENERGY_GOAL: u32 = 100000;

pub const SYSTEM_USERNAME: &str = "Screeps";

pub const SIGN_PLANNED_AREA: &str =
    "A new Novice or Respawn Area is being planned somewhere \
     in this sector. Please make sure all important rooms are reserved.";

// EVENT_* constants in src/objects/impls/room.rs

pub const POWER_LEVEL_MULTIPLY: u32 = 1000;
pub const POWER_LEVEL_POW: u32 = 2;
pub const POWER_CREEP_SPAWN_COOLDOWN: u32 = 8 * 3600 * 1000;
pub const POWER_CREEP_DELETE_COOLDOWN: u32 = 24 * 3600 * 1000;
pub const POWER_CREEP_MAX_LEVEL: u32 = 25;
pub const POWER_CREEP_LIFE_TIME: u32 = 5000;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PowerClass {
    Operator,
}

/// Traslates the `PWR_*` constants.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u32)]
pub enum PowerType {
    GenerateOps = 1,
    OperateSpawn = 2,
    OperateTower = 3,
    OperateStorage = 4,
    OperateLab = 5,
    OperateExtension = 6,
    OperateObserve = 7,
    OperateTerminal = 8,
    DisruptSpawn = 9,
    DisruptTower = 10,
    Shield = 12,
    RegenSource = 13,
    RegenMineral = 14,
    DisruptTerminal = 15,
    OperatePower = 16,
    Fortify = 17,
    OperateController = 18,
    OperateFactory = 19,
}

js_deserializable!(PowerType);
