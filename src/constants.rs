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
//! # Notes on Deserialization
//!
//! There are two general types of enum constants in this file. Some are
//! represented by integers in the game - and those are represented by integers
//! in screeps. Their [`serde::Deserialize`], `TryFrom<Value>` and
//! [`num_traits::FromPrimitive`] implementations will convert from these
//! integers.
//!
//! The other type is enums represented by strings in the game, but integers in
//! this repository. This change in representation is done for efficiency, as
//! transferring strings from JavaScript to Rust is much slower than a single
//! integer.
//!
//! This second type of enum will also implement [`serde::Deserialize`],
//! `TryFrom<Value>`, but these will not convert from the made-up integer
//! values, and will fail converting from the constant strings from the game.
//!
//! To convert from constant strings, you have two options, depending on the
//! context.
//!
//! If you need to manually consume from a value in JavaScript, there are two
//! utility JavaScript functions per enum. They generally take the form
//! `__TYPE_num_to_str` and `__TYPE_str_to_num`. For example,
//! `__structure_type_num_to_str` and `__structure_type_str_to_num` convert
//! between [`StructureType`] integer representations and string
//! representations. See documentation on enums for more conversion functions.
//!
//! To use these, call the functions in JavaScript, like so:
//!
//! ```no_run
//! use screeps::{game, traits::TryInto, StructureType};
//! use stdweb::js;
//!
//! let spawns = game::spawns::values();
//! let r: StructureType = (js! {
//!     return __structure_type_str_to_num(@{spawns[0].as_ref()}.structureType);
//! })
//! .try_into()
//! .expect("expected structure type to convert successfully");
//! ```
//!
//! If you need to consume strings already in Rust, use either the [`FromStr`]
//! trait, or one of the `deserialize_from_str` functions on each of these
//! constants.
//!
//! [the game constants]: https://github.com/screeps/common/blob/master/lib/constants.js
//! [`FromStr`]: std::str::FromStr
use std::{borrow::Cow, fmt, str::FromStr};

use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use parse_display::FromStr;
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};
use stdweb::Value;

use crate::macros::*;

pub use self::find::FindConstant;

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(i16)]
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
}

impl ReturnCode {
    /// Turns this return code into a result.
    ///
    /// `ReturnCode::Ok` is turned into `Result::Ok`, all other codes are turned
    /// into `Result::Err(code)`
    #[inline]
    pub fn as_result(self) -> Result<(), Self> {
        match self {
            ReturnCode::Ok => Ok(()),
            other => Err(other),
        }
    }
}

js_deserializable!(ReturnCode);

/// Constants for use with the [`Room::find`] function.
///
/// *Note:* Types in this module have purposefully ambiguous names, and are
/// intended to be used as, for example, `find::CREEPS`, not `CREEPS`.
///
/// You can do this my importing the module itself, rather than any individual
/// constant, and then just referring to the constants relative to the module.
///
/// # Example
///
/// ```no_run
/// use screeps::{find, game, Room};
///
/// let room: Room = game::rooms::get("E23S55".parse().unwrap()).unwrap();
///
/// let creeps = room.find(find::CREEPS);
/// # let _ = creeps;
/// ```
///
/// [`Room::find`]: crate::Room::find
/// [`objects::RoomObject`]: crate::RoomObject
pub mod find {
    use num_derive::FromPrimitive;
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use stdweb::Reference;

    use crate::{
        local::Position,
        objects::{
            ConstructionSite, Creep, Flag, Mineral, Nuke, OwnedStructure, PowerCreep, Resource,
            Source, Structure, StructureSpawn, Tombstone,
        },
        traits::FromExpectedType,
    };

    /// Trait representing things which can be used in the 'find' function.
    ///
    /// Typically used with zero-sized structs in the
    /// [`find`][crate::constants::find] module.
    pub unsafe trait FindConstant {
        type Item: FromExpectedType<Reference>;

        fn find_code(&self) -> i16;
    }

    /// Useful for finding any [`RoomObject`] with
    /// a dynamically-chosen find constant.
    ///
    /// If you know ahead of time what constant you'll use, then the
    /// all-upper-case constants in [this module][crate::constants::find] will
    /// be more helpful, and won't require casting the result types.
    ///
    /// *Note*: To avoid ambiguity with [`RoomObject`], you should refer to this
    /// enum as `find::RoomObject` rather than importing it directly.
    ///
    /// [`RoomObject`]: crate::objects::RoomObject
    #[derive(
        Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
    )]
    #[repr(i16)]
    pub enum RoomObject {
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

    unsafe impl FindConstant for RoomObject {
        type Item = crate::objects::RoomObject;

        #[inline]
        fn find_code(&self) -> i16 {
            *self as i16
        }
    }

    #[derive(
        Copy, Clone, Debug, FromPrimitive, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash,
    )]
    #[repr(i16)]
    pub enum Exit {
        Top = 1,
        Right = 3,
        Bottom = 5,
        Left = 7,
        All = 10,
    }

    impl Exit {
        #[inline]
        pub fn top() -> Self {
            Exit::Top
        }

        #[inline]
        pub fn right() -> Self {
            Exit::Right
        }

        #[inline]
        pub fn bottom() -> Self {
            Exit::Bottom
        }

        #[inline]
        pub fn left() -> Self {
            Exit::Left
        }

        #[inline]
        pub fn all() -> Self {
            Exit::All
        }
    }

    unsafe impl FindConstant for Exit {
        type Item = Position;

        #[inline]
        fn find_code(&self) -> i16 {
            *self as i16
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
        EXIT_TOP, Exit::Top as i16, Position;
        EXIT_RIGHT, Exit::Right as i16, Position;
        EXIT_BOTTOM, Exit::Bottom as i16, Position;
        EXIT_LEFT, Exit::Left as i16, Position;
        EXIT, Exit::All as i16, Position;
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
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
    #[inline]
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

/// Type used for when the game returns a direction to an exit.
///
/// Restricted more than `Direction` in that it can't be diagonal. Used as the
/// result of [`Room::find_exit_to`].
///
/// Can be converted to both [`find::Exit`] for immediate use of [`Room::find`]
/// and [`Direction`].
///
/// [`Room::find`]: crate::objects::Room::find
/// [`Room::find_exit_to`]: crate::objects::Room::find_exit_to
#[derive(
    Copy, Clone, Debug, FromPrimitive, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash,
)]
#[repr(u8)]
pub enum ExitDirection {
    Top = Direction::Top as u8,
    Right = Direction::Right as u8,
    Bottom = Direction::Bottom as u8,
    Left = Direction::Left as u8,
}

impl From<ExitDirection> for find::Exit {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => find::Exit::Top,
            ExitDirection::Right => find::Exit::Right,
            ExitDirection::Bottom => find::Exit::Bottom,
            ExitDirection::Left => find::Exit::Left,
        }
    }
}

impl From<ExitDirection> for Direction {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => Direction::Top,
            ExitDirection::Right => Direction::Right,
            ExitDirection::Bottom => Direction::Bottom,
            ExitDirection::Left => Direction::Left,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(u8)]
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

js_deserializable!(Color);

/// Terrain constant.
///
/// This constant is in a unique position of being represented both by strings
/// and by integers in various parts of the API.
///
/// *Note:* This constant's `TryFrom<Value>` and `Deserialize` implementations
/// _only work with the integer constants_. If you're ever consuming strings
/// such as `"plain"`, `"swamp"`, `"wall"`, you can use the
/// `__terrain_str_to_num` JavaScript function, [`FromStr`][std::str::FromStr]
/// or [`Look::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Deserialize_repr,
    Serialize_repr,
    FromPrimitive,
    FromStr,
)]
#[repr(u8)]
#[display(style = "snake_case")]
pub enum Terrain {
    Plain = 0,
    Wall = TERRAIN_MASK_WALL,
    Swamp = TERRAIN_MASK_SWAMP,
}

impl Terrain {
    /// Helper function for deserializing from a string rather than from an
    /// integer.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &r#""plain", "wall" or "swamp""#)
        })
    }
}

js_deserializable!(Terrain);

/// Internal enum representing each LOOK_* constant.
///
/// It's recommended to use the constants in the `look` module instead for type
/// safety.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__look_num_to_str` and
/// `__look_str_to_num` JavaScript functions, [`FromStr`][std::str::FromStr] or
/// [`Look::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
pub enum Look {
    #[display("creep")]
    Creeps = 0,
    #[display("energy")]
    Energy = 1,
    #[display("resource")]
    Resources = 2,
    #[display("source")]
    Sources = 3,
    #[display("mineral")]
    Minerals = 4,
    #[display("structure")]
    Structures = 5,
    #[display("flag")]
    Flags = 6,
    #[display("constructionSite")]
    ConstructionSites = 7,
    #[display("nuke")]
    Nukes = 8,
    #[display("terrain")]
    Terrain = 9,
    #[display("tombstone")]
    Tombstones = 10,
    #[display("powerCreep")]
    PowerCreeps = 11,
}

js_deserializable!(Look);

impl Look {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a known LOOK_* constant string")
        })
    }
}

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

/// Creep part types.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__part_num_to_str` and
/// `__part_str_to_num` JavaScript functions, [`FromStr`][std::str::FromStr] or
/// [`Part::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
#[display(style = "snake_case")]
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
    #[inline]
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

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in BODYPARTS_ALL",
            )
        })
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
pub const SPAWN_RENEW_RATION: f32 = 1.2;

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
pub const LINK_LOSS_RATION: f32 = 0.03;

pub const STORAGE_CAPACITY: u32 = 1_000_000;
pub const STORAGE_HITS: u32 = 10_000;

/// Translates `STRUCTURE_*` constants.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__structure_type_num_to_str` and
/// `__structure_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`StructureType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
#[display(style = "camelCase")]
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
    #[inline]
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

    #[inline]
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

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a known STRUCTURE_* constant string")
        })
    }
}

js_deserializable!(StructureType);

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

/// Translates the `DENSITY_*` constants.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Density {
    Low = 1,
    Moderate = 2,
    High = 3,
    Ultra = 4,
}

js_deserializable!(Density);

impl Density {
    /// Translates the `MINERAL_DENSITY` constant.
    #[inline]
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
    /// These are values intended for subsequent percentage checks
    /// in the order `Low` -> `Medium` -> `High` -> `Ultra`. Use the
    /// [`Density::iter_values`] iterator to iterate in this order.
    #[inline]
    pub fn probability(self) -> f32 {
        match self {
            Density::Low => 0.1,
            Density::Moderate => 0.5,
            Density::High => 0.9,
            Density::Ultra => 1.0,
        }
    }

    pub fn iter_values() -> impl Iterator<Item = Density> {
        <Density as enum_iterator::IntoEnumIterator>::into_enum_iter()
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
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__intershard_resource_type_num_to_str`
/// and `__intershard_resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or
/// [`IntershardResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
pub enum IntershardResourceType {
    #[display("token")]
    SubscriptionToken = 1,
}

impl IntershardResourceType {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in INTERSHARD_RESOURCES",
            )
        })
    }
}

js_deserializable!(IntershardResourceType);

/// Resource type constant for all possible types of resources.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__resource_type_num_to_str`
/// and `__resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`ResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u16)]
pub enum ResourceType {
    /// `"energy"`
    #[display("energy")]
    Energy = 1,
    /// `"power"`
    #[display("power")]
    Power = 2,
    /// `"H"`
    #[display("H")]
    Hydrogen = 3,
    /// `"O"`
    #[display("O")]
    Oxygen = 4,
    /// `"U"`
    #[display("U")]
    Utrium = 5,
    /// `"L"`
    #[display("L")]
    Lemergium = 6,
    /// `"K"`
    #[display("K")]
    Keanium = 7,
    /// `"Z"`
    #[display("Z")]
    Zynthium = 8,
    /// `"X"`
    #[display("X")]
    Catalyst = 9,
    /// `"G"`
    #[display("G")]
    Ghodium = 10,
    /// `"OH"`
    #[display("OH")]
    Hydroxide = 11,
    /// `"ZK"`
    #[display("ZK")]
    ZynthiumKeanite = 12,
    /// `"UL"`
    #[display("UL")]
    UtriumLemergite = 13,
    /// `"UH"`
    #[display("UH")]
    UtriumHydride = 14,
    /// `"UO"`
    #[display("UO")]
    UtriumOxide = 15,
    /// `"KH"`
    #[display("KH")]
    KeaniumHydride = 16,
    /// `"KO"`
    #[display("KO")]
    KeaniumOxide = 17,
    /// `"LH"`
    #[display("LH")]
    LemergiumHydride = 18,
    /// `"LO"`
    #[display("LO")]
    LemergiumOxide = 19,
    /// `"ZH"`
    #[display("ZH")]
    ZynthiumHydride = 20,
    /// `"ZO"`
    #[display("ZO")]
    ZynthiumOxide = 21,
    /// `"GH"`
    #[display("GH")]
    GhodiumHydride = 22,
    /// `"GO"`
    #[display("GO")]
    GhodiumOxide = 23,
    /// `"UH2O"`
    #[display("UH2O")]
    UtriumAcid = 24,
    /// `"UHO2"`
    #[display("UHO2")]
    UtriumAlkalide = 25,
    /// `"KH2O"`
    #[display("KH2O")]
    KeaniumAcid = 26,
    /// `"KHO2"`
    #[display("KHO2")]
    KeaniumAlkalide = 27,
    /// `"LH2O"`
    #[display("LH2O")]
    LemergiumAcid = 28,
    /// `"LHO2"`
    #[display("LHO2")]
    LemergiumAlkalide = 29,
    /// `"ZH2O"`
    #[display("ZH2O")]
    ZynthiumAcid = 30,
    /// `"ZHO2"`
    #[display("ZHO2")]
    ZynthiumAlkalide = 31,
    /// `"GH2O"`
    #[display("GH2O")]
    GhodiumAcid = 32,
    /// `"GHO2"`
    #[display("GHO2")]
    GhodiumAlkalide = 33,
    /// `"XUH2O"`
    #[display("XUH2O")]
    CatalyzedUtriumAcid = 34,
    /// `"XUHO2"`
    #[display("XUHO2")]
    CatalyzedUtriumAlkalide = 35,
    /// `"XKH2O"`
    #[display("XKH2O")]
    CatalyzedKeaniumAcid = 36,
    /// `"XKHO2"`
    #[display("XKHO2")]
    CatalyzedKeaniumAlkalide = 37,
    /// `"XLH2O"`
    #[display("XLH2O")]
    CatalyzedLemergiumAcid = 38,
    /// `"XLHO2"`
    #[display("XLHO2")]
    CatalyzedLemergiumAlkalide = 39,
    /// `"XZH2O"`
    #[display("XZH2O")]
    CatalyzedZynthiumAcid = 40,
    /// `"XZHO2"`
    #[display("XZHO2")]
    CatalyzedZynthiumAlkalide = 41,
    /// `"XGH2O"`
    #[display("XGH2O")]
    CatalyzedGhodiumAcid = 42,
    /// `"XGHO2"`
    #[display("XGHO2")]
    CatalyzedGhodiumAlkalide = 43,
    /// `"ops"`
    #[display("ops")]
    Ops = 44,
}

impl ResourceType {
    /// Translates the `REACTION_TIME` constant.
    #[inline]
    pub fn reaction_time(self) -> Option<u32> {
        use crate::ResourceType::*;
        let time = match self {
            Energy | Power | Hydrogen | Oxygen | Utrium | Lemergium | Keanium | Zynthium
            | Catalyst | Ops => return None,
            // these comments copied directly from JavaScript 'constants.js' file.
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
            ZynthiumHydride => 20,
            // ZH2O: 40,
            ZynthiumAcid => 40,
            // XZH2O: 160,
            CatalyzedZynthiumAcid => 160,
            // ZO: 10,
            ZynthiumOxide => 10,
            // ZHO2: 5,
            ZynthiumAlkalide => 5,
            // XZHO2: 60,
            CatalyzedZynthiumAlkalide => 60,
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

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in RESOURCES_ALL",
            )
        })
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

/// Translates the `PWR_*` constants.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
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
