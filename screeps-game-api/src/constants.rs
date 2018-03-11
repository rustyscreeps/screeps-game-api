//! Constants, most copied from [the game constants](https://github.com/screeps/common/blob/master/lib/constants.js).
//!
//! Last updated on 2018-03-06, `c3372fd` on https://github.com/screeps/common/commits/master/lib/constants.js.
use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

use objects::RoomObject;

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}

impl TryFrom<Value> for ReturnCode {
    type Error = <i32 as TryFrom<Value>>::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;
        let x: i32 = v.try_into()?;
        Ok(Self::from_i32(x).unwrap_or_else(|| {
            error!("encountered a return code we don't know: {}", x);
            ReturnCode::Other
        }))
    }
}

pub unsafe trait FindConstant {
    type Item: TryFrom<Value, Error = <Reference as TryFrom<Value>>::Error>;

    fn find_code(&self) -> i32;
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    use stdweb::unstable::TryFrom;
    use super::FindConstant;

    use objects::{ConstructionSite, Creep, Flag, Mineral, Nuke, OwnedStructure, Resource,
                  RoomPosition, Source, Structure, StructureSpawn, Tombstone};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Exit(i32);

    impl Exit {
        pub fn top() -> Self {
            Exit(1)
        }

        pub fn right() -> Self {
            Exit(3)
        }

        pub fn bottom() -> Self {
            Exit(5)
        }

        pub fn left() -> Self {
            Exit(7)
        }

        pub fn all() -> Self {
            Exit(10)
        }
    }

    impl TryFrom<i32> for Exit {
        type Error = i32;

        fn try_from(v: i32) -> Result<Exit, Self::Error> {
            match v {
                1 | 3 | 5 | 7 | 10 => Ok(Exit(v)),
                _ => Err(v),
            }
        }
    }

    unsafe impl FindConstant for Exit {
        type Item = RoomPosition;

        fn find_code(&self) -> i32 {
            self.0
        }
    }

    macro_rules! typesafe_find_constants {
        (
            $($constant_name:ident, $value:expr, $result:path;)*
        ) => (
            $(
                #[allow(bad_style)]
                pub struct $constant_name;
                unsafe impl FindConstant for $constant_name {
                    type Item = $result;

                    fn find_code(&self) -> i32 {
                        $value
                    }
                }
            )*
        );
    }

    typesafe_find_constants! {
        CREEPS, 101, Creep;
        MY_CREEPS, 102, Creep;
        HOSTILE_CREEPS, 103, Creep;
        SOURCES_ACTIVE, 104, Source;
        SOURCES, 105, Source;
        DROPPED_RESOUCES, 106, Resource;
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
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
}

impl TryFrom<Value> for Direction {
    type Error = <u32 as TryFrom<Value>>::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;

        let as_num = u32::try_from(v)?;

        Ok(Self::from_u32(as_num).unwrap_or_else(|| {
            panic!("encountered a direction code we don't know: {}", as_num);
        }))
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Terrain {
    Plain = 0,
    Wall = 1,
    Swamp = 2,
}

impl TryFrom<Value> for Terrain {
    type Error = <u32 as TryFrom<Value>>::Error;

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
                1 => Terrain::Wall,
                2 => Terrain::Swamp,
                // might not need this, but just in case we try
                // to decode a game-encoded number and '3' represents swamp + wall
                3 => Terrain::Wall,
                x => panic!("unknown terrain encoded integer {}", x),
            },
        };
        Ok(v)
    }
}

impl AsRef<str> for Terrain {
    fn as_ref(&self) -> &str {
        match *self {
            Terrain::Plain => "plain",
            Terrain::Wall => "wall",
            Terrain::Swamp => "swamp",
        }
    }
}

/// Internal enum representing each LOOK_* constant.
///
/// It's recommended to use the constants in the `look` module instead for type safety.
///
/// In fact, I don't believe this can be used at all without resorting to manually
/// including JS code.
///
/// To use in JS: `__look_num_to_str(@{look as i32})` function
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub enum Look {
    Creeps = 0,
    Energy = 1,
    Resources = 2,
    Sources = 3,
    Minerals = 4,
    Structures = 5,
    Flags = 6,
    ConstructionSites = 7,
    Nukes = 8,
    Terrain = 9,
    Tombstones = 10,
}

pub unsafe trait LookConstant {
    type Item: TryFrom<Value, Error = <Reference as TryFrom<Value>>::Error>;

    fn look_code(&self) -> Look;
}

pub mod look {
    use super::{Look, LookConstant};
    use {ConstructionSite, Creep, Flag, Mineral, Nuke, Resource, Source, Structure, Terrain,
         Tombstone};

    macro_rules! typesafe_look_constants {
        (
            $($constant_name:ident, $value:expr, $result:path;)*
        ) => (
            $(
                #[allow(bad_style)]
                pub struct $constant_name;
                unsafe impl LookConstant for $constant_name {
                    type Item = $result;

                    fn look_code(&self) -> Look {
                        $value
                    }
                }
            )*
        );
    }

    typesafe_look_constants! {
        CREEPS, Look::Creeps, Creep;
        ENERGY, Look::Energy, Resource;
        RESOURCES, Look::Resources, Resource;
        SOURCES, Look::Sources, Source;
        MINERALS, Look::Minerals, Mineral;
        STRUCTURES, Look::Structures, Structure;
        FLAGS, Look::Flags, Flag;
        CONSTRUCTION_SITES, Look::ConstructionSites, ConstructionSite;
        NUKES, Look::Nukes, Nuke;
        TERRAIN, Look::Terrain, Terrain;
        TOMBSTONES, Look::Tombstones, Tombstone;
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub fn cost(&self) -> i32 {
        // TODO: compile time feature to switch to dynamically for non-standard servers
        match *self {
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

impl TryFrom<u32> for Part {
    type Error = ();
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        let res = match x {
            0 => Part::Move,
            1 => Part::Work,
            2 => Part::Carry,
            3 => Part::Attack,
            4 => Part::RangedAttack,
            5 => Part::Tough,
            6 => Part::Heal,
            7 => Part::Claim,
            _ => return Err(()),
        };
        Ok(res)
    }
}

impl TryFrom<Value> for Part {
    type Error = <Value as TryInto<u32>>::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let x: u32 = v.try_into()?;
        Ok(Self::try_from(x)
            .unwrap_or_else(|()| panic!("JavaScript gave unknown part constant {}", x)))
    }
}

pub const CREEP_LIFE_TIME: i32 = 1500;
pub const CREEP_CLAIM_LIFE_TIME: i32 = 500;
pub const CREEP_CORPSE_RATE: f32 = 0.2;

pub const CARRY_CAPACITY: i32 = 50;

pub const HARVEST_POWER: i32 = 2;
pub const HARVEST_MINERAL_POWER: i32 = 1;
pub const REPAIR_POWER: i32 = 100;
pub const DISMANTLE_POWER: i32 = 50;
pub const BUILD_POWER: i32 = 5;
pub const ATTACK_POWER: i32 = 30;
pub const UPGRADE_CONTROLLER_POWER: i32 = 1;
pub const RANGED_ATTACK_POWER: i32 = 10;
pub const HEAL_POWER: i32 = 12;
pub const RANGED_HEAL_POWER: i32 = 4;

pub const REPAIR_COST: f32 = 0.01;
pub const DISMANTLE_COST: f32 = 0.005;

pub const RAMPART_DECAY_AMOUNT: i32 = 300;
pub const RAMPART_DECAY_TIME: i32 = 100;

pub const RAMPART_HITS_MAX_RCL2: i32 = 300_000;
pub const RAMPART_HITS_MAX_RCL3: i32 = 1_000_000;
pub const RAMPART_HITS_MAX_RCL4: i32 = 3_000_000;
pub const RAMPART_HITS_MAX_RCL5: i32 = 5_000_000;
pub const RAMPART_HITS_MAX_RCL6: i32 = 30_000_000;
pub const RAMPART_HITS_MAX_RCL7: i32 = 100_000_000;
pub const RAMPART_HITS_MAX_RCL8: i32 = 300_000_000;

pub fn rampart_hits_max(rcl: i32) -> i32 {
    match rcl {
        r if r < 2 => 0,
        2 => RAMPART_HITS_MAX_RCL2,
        3 => RAMPART_HITS_MAX_RCL3,
        4 => RAMPART_HITS_MAX_RCL4,
        5 => RAMPART_HITS_MAX_RCL5,
        6 => RAMPART_HITS_MAX_RCL6,
        7 => RAMPART_HITS_MAX_RCL7,
        8 => RAMPART_HITS_MAX_RCL8,
        _ => RAMPART_HITS_MAX_RCL8,
    }
}

pub const ENERGY_REGEN_TIME: i32 = 300;
pub const ENERGY_DECAY: i32 = 1000;

pub const SPAWN_ENERGY_START: i32 = 300;
pub const SPAWN_ENERGY_CAPACITY: i32 = 300;
pub const CREEP_SPAWN_TIME: i32 = 3;
pub const SPAWN_RENEW_RATION: f32 = 1.2;

pub const SOURCE_ENERGY_CAPACITY: i32 = 3000;
pub const SOURCE_ENERGY_NEUTRAL_CAPACITY: i32 = 1500;
pub const SOURCE_ENERGY_KEEPER_CAPACITY: i32 = 4000;

pub const WALL_HITS_MAX: i32 = 300_000_000;

pub fn extension_energy_capacity(rcl: i32) -> i32 {
    match rcl {
        r if r < 7 => 50,
        7 => 100,
        8 => 200,
        _ => 200,
    }
}

pub const ROAD_WEAROUT: i32 = 1;
pub const ROAD_DECAY_AMOUNT: i32 = 100;
pub const ROAD_DECAY_TIME: i32 = 1000;

pub const LINK_CAPACITY: i32 = 800;
pub const LINK_COOLDOWN: i32 = 1;
pub const LINK_LOSS_RATION: f32 = 0.03;

pub const STORAGE_CAPACITY: i32 = 1_000_000;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    pub fn construction_cost(&self) -> i32 {
        use self::StructureType::*;

        match *self {
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
            KeeperLair | PowerBank | Portal | Controller => 0,
        }
    }

    pub fn initial_hits(&self) -> i32 {
        use self::StructureType::*;

        match *self {
            Spawn => 5000,
            Extension => 1000,
            Road => 5000,
            Wall => 1,
            Rampart => 1,
            Link => 1000,
            Storage => 10_000,
            Tower => 3000,
            Observer => 500,
            PowerBank => 2_000_000,
            PowerSpawn => 5000,
            Extractor => 500,
            Lab => 500,
            Terminal => 3000,
            Container => 250_000,
            Nuker => 1000,
            KeeperLair | Portal | Controller => 0,
        }
    }
}

impl TryFrom<Value> for StructureType {
    type Error = <i32 as TryFrom<Value>>::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let x: i32 = v.try_into()?;
        Ok(match x {
            0 => StructureType::Spawn,
            1 => StructureType::Extension,
            2 => StructureType::Road,
            3 => StructureType::Wall,
            4 => StructureType::Rampart,
            5 => StructureType::KeeperLair,
            6 => StructureType::Portal,
            7 => StructureType::Controller,
            8 => StructureType::Link,
            9 => StructureType::Storage,
            10 => StructureType::Tower,
            11 => StructureType::Observer,
            12 => StructureType::PowerBank,
            13 => StructureType::PowerSpawn,
            14 => StructureType::Extractor,
            15 => StructureType::Lab,
            16 => StructureType::Terminal,
            17 => StructureType::Container,
            18 => StructureType::Nuker,
            _ => panic!("unknown structure type integer {}", x),
        })
    }
}

pub const CONSTRUCTION_COST_ROAD_SWAMP_RATIO: i32 = 5;

/// Accepts levels 0-7. any other results in 0.
pub fn controller_levels(current_rcl: i32) -> i32 {
    match current_rcl {
        1 => 200,
        2 => 45_000,
        3 => 135_000,
        4 => 405_000,
        5 => 1_215_000,
        6 => 3_645_000,
        7 => 10_935_000,
        _ => 0,
    }
}

// TODO: controller_*

pub const SAFE_MODE_DURATION: i32 = 20_000;
pub const SAFE_MODE_COOLDOWN: i32 = 50_000;
pub const SAFE_MODE_COST: i32 = 1000;

pub const TOWER_CAPACITY: i32 = 1000;
pub const TOWER_ENERGY_COST: i32 = 10;
pub const TOWER_POWER_ATTACK: i32 = 600;
pub const TOWER_POWER_HEAL: i32 = 400;
pub const TOWER_POWER_REPAIR: i32 = 800;
pub const TOWER_OPTIMAL_RANGE: i32 = 5;
pub const TOWER_FALLOFF_RANGE: i32 = 20;
pub const TOWER_FALLOFF: f32 = 0.75;

pub const OBSERVER_RANGE: i32 = 10;

pub const POWER_BANK_CAPACITY_MAX: i32 = 5000;
pub const POWER_BANK_CAPACITY_MIN: i32 = 500;
pub const POWER_BANK_CAPACITY_CRIT: f32 = 0.3;
pub const POWER_BANK_DECAY: i32 = 5000;
pub const POWER_BANK_HIT_BACK: f32 = 0.5;

pub const POWER_SPAWN_ENERGY_CAPACITY: i32 = 5000;
pub const POWER_SPAWN_POWER_CAPACITY: i32 = 100;
pub const POWER_SPAWN_ENERGY_RATIO: i32 = 50;

pub const EXTRACTOR_COOLDOWN: i32 = 5;

pub const LAB_MINERAL_CAPACITY: i32 = 3000;
pub const LAB_ENERGY_CAPACITY: i32 = 2000;
pub const LAB_BOOST_ENERGY: i32 = 20;
pub const LAB_BOOST_MINERAL: i32 = 30;

pub const LAB_REACTION_AMOUNT: i32 = 5;

pub const GCL_POW: f32 = 2.4;
pub const GCL_MULTIPLY: i32 = 1000000;
pub const GCL_NOVICE: i32 = 3;

pub const TERRAIN_MASK_WALL: i32 = 1;
pub const TERRAIN_MASK_SWAMP: i32 = 2;
pub const TERRAIN_MASK_LAVA: i32 = 4;

pub const MAX_CONSTRUCTION_SITES: i32 = 100;
pub const MAX_CREEP_SIZE: i32 = 50;

pub const MINERAL_REGEN_TIME: i32 = 50_000;

// TODO: MINERAL_* constants

pub const DENSITY_LOW: i32 = 1;
pub const DENSITY_MODERATE: i32 = 2;
pub const DENSITY_HIGH: i32 = 3;
pub const DENSITY_ULTRA: i32 = 4;

pub const TERMINAL_CAPACITY: i32 = 300000;
pub const TERMINAL_HITS: i32 = 3000;
pub const TERMINAL_SEND_COST: f32 = 0.1;
pub const TERMINAL_MIN_SEND: i32 = 100;
pub const TERMINAL_COOLDOWN: i32 = 10;

pub const CONTAINER_HITS: i32 = 250000;
pub const CONTAINER_CAPACITY: i32 = 2000;
pub const CONTAINER_DECAY: i32 = 5000;
pub const CONTAINER_DECAY_TIME: i32 = 100;
pub const CONTAINER_DECAY_TIME_OWNED: i32 = 500;

pub const NUKER_HITS: i32 = 1000;
pub const NUKER_COOLDOWN: i32 = 100000;
pub const NUKER_ENERGY_CAPACITY: i32 = 300000;
pub const NUKER_GHODIUM_CAPACITY: i32 = 5000;
pub const NUKE_LAND_TIME: i32 = 50000;
pub const NUKE_RANGE: i32 = 10;

pub const TOMBSTONE_DECAY_PER_PART: i32 = 5;

pub const PORTAL_DECAY: i32 = 30000;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    /// `"energy"`
    Energy = 1,
    /// `"power"`
    Power = 2,
    /// `"H"`
    Hydrogen = 3,
    /// `"O"`
    Oxygen = 4,
    /// `"U"`
    Utrium = 5,
    /// `"L"`
    Lemergium = 6,
    /// `"K"`
    Keanium = 7,
    /// `"Z"`
    Zynthium = 8,
    /// `"X"`
    Catalyst = 9,
    /// `"G"`
    Ghodium = 10,

    /// `"OH"`
    Hydroxide = 11,
    /// `"ZK"`
    ZynthiumKeanite = 12,
    /// `"UL"`
    UtriumLemergite = 13,

    /// `"UH"`
    UtriumHydride = 14,
    /// `"UO"`
    UtriumOxide = 15,
    /// `"KH"`
    KeaniumHydride = 16,
    /// `"KO"`
    KeaniumOxide = 17,
    /// `"LH"`
    LemergiumHydride = 18,
    /// `"LO"`
    LemergiumOxide = 19,
    /// `"ZH"`
    ZynthiumHydride = 20,
    /// `"ZO"`
    ZynthiumOxide = 21,
    /// `"GH"`
    GhodiumHydride = 22,
    /// `"GO"`
    GhodiumOxide = 23,
    /// `"UH2O"`
    UtriumAcid = 24,
    /// `"UHO2"`
    UtriumAlkalide = 25,
    /// `"KH2O"`
    KeaniumAcid = 26,
    /// `"KHO2"`
    KeaniumAlkalide = 27,
    /// `"LH2O"`
    LemergiumAcid = 28,
    /// `"LHO2"`
    LemergiumAlkalide = 29,
    /// `"ZH2O"`
    ZynthiumAcid = 30,
    /// `"ZHO2"`
    ZynthiumAlkalide = 31,
    /// `"GH2O"`
    GhodiumAcid = 32,
    /// `"GHO2"`
    GhodiumAlkalide = 33,
    /// `"XUH2O"`
    CatalyzedUtriumAcid = 34,
    /// `"XUHO2"`
    CatalyzedUtriumAlkalide = 35,
    /// `"XKH2O"`
    CatalyzedKeaniumAcid = 36,
    /// `"XKHO2"`
    CatalyzedKeaniumAlkalide = 37,
    /// `"XLH2O"`
    CatalyzedLemergiumAcid = 38,
    /// `"XLHO2"`
    CatalyzedLemergiumAlkalide = 39,
    /// `"XZH2O"`
    CatalyzedZynthiumAcid = 40,
    /// `"XZHO2"`
    CatalyzedZynthiumAlkalide = 41,
    /// `"XGH2O"`
    CatalyzedGhodiumAcid = 42,
    /// `"XGHO2"`
    CatalyzedGhodiumAlkalide = 43,
}

impl ResourceType {
    /// Returns `REACTION_TIME` for this resource. 0 for energy and base minerals.
    pub fn reaction_time(&self) -> i32 {
        use ResourceType::*;
        match *self {
            Energy | Power | Hydrogen | Oxygen | Utrium | Lemergium | Keanium | Zynthium
            | Catalyst | Ghodium => 0,
            Hydroxide => 20,
            ZynthiumKeanite => 5,
            UtriumLemergite => 10,
            UtriumHydride => 10,
            UtriumAcid => 5,
            CatalyzedUtriumAcid => 60,
            UtriumOxide => 10,
            UtriumAlkalide => 5,
            CatalyzedUtriumAlkalide => 60,
            KeaniumHydride => 10,
            KeaniumAcid => 5,
            CatalyzedKeaniumAcid => 60,
            KeaniumOxide => 10,
            KeaniumAlkalide => 5,
            CatalyzedKeaniumAlkalide => 60,
            LemergiumHydride => 15,
            LemergiumAcid => 10,
            CatalyzedLemergiumAcid => 65,
            LemergiumOxide => 10,
            LemergiumAlkalide => 5,
            CatalyzedLemergiumAlkalide => 60,
            ZynthiumHydride => 50,
            ZynthiumAcid => 100,
            CatalyzedZynthiumAcid => 180,
            ZynthiumOxide => 10,
            ZynthiumAlkalide => 5,
            CatalyzedZynthiumAlkalide => 80,
            GhodiumHydride => 10,
            GhodiumAcid => 15,
            CatalyzedGhodiumAcid => 80,
            GhodiumOxide => 10,
            GhodiumAlkalide => 15,
            CatalyzedGhodiumAlkalide => 90,
        }
    }
}

impl TryFrom<Value> for ResourceType {
    type Error = <i32 as TryFrom<Value>>::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let x: i32 = v.try_into()?;
        Ok(match x {
            1 => ResourceType::Energy,
            2 => ResourceType::Power,
            3 => ResourceType::Hydrogen,
            4 => ResourceType::Oxygen,
            5 => ResourceType::Utrium,
            6 => ResourceType::Lemergium,
            7 => ResourceType::Keanium,
            8 => ResourceType::Zynthium,
            9 => ResourceType::Catalyst,
            10 => ResourceType::Ghodium,
            11 => ResourceType::Hydroxide,
            12 => ResourceType::ZynthiumKeanite,
            13 => ResourceType::UtriumLemergite,
            14 => ResourceType::UtriumHydride,
            15 => ResourceType::UtriumOxide,
            16 => ResourceType::KeaniumHydride,
            17 => ResourceType::KeaniumOxide,
            18 => ResourceType::LemergiumHydride,
            19 => ResourceType::LemergiumOxide,
            20 => ResourceType::ZynthiumHydride,
            21 => ResourceType::ZynthiumOxide,
            22 => ResourceType::GhodiumHydride,
            23 => ResourceType::GhodiumOxide,
            24 => ResourceType::UtriumAcid,
            25 => ResourceType::UtriumAlkalide,
            26 => ResourceType::KeaniumAcid,
            27 => ResourceType::KeaniumAlkalide,
            28 => ResourceType::LemergiumAcid,
            29 => ResourceType::LemergiumAlkalide,
            30 => ResourceType::ZynthiumAcid,
            31 => ResourceType::ZynthiumAlkalide,
            32 => ResourceType::GhodiumAcid,
            33 => ResourceType::GhodiumAlkalide,
            34 => ResourceType::CatalyzedUtriumAcid,
            35 => ResourceType::CatalyzedUtriumAlkalide,
            36 => ResourceType::CatalyzedKeaniumAcid,
            37 => ResourceType::CatalyzedKeaniumAlkalide,
            38 => ResourceType::CatalyzedLemergiumAcid,
            39 => ResourceType::CatalyzedLemergiumAlkalide,
            40 => ResourceType::CatalyzedZynthiumAcid,
            41 => ResourceType::CatalyzedZynthiumAlkalide,
            42 => ResourceType::CatalyzedGhodiumAcid,
            43 => ResourceType::CatalyzedGhodiumAlkalide,
            _ => panic!("unknown resource type integer {}", x),
        })
    }
}
