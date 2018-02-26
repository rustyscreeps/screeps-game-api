use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

use api::objects::RoomObject;

#[repr(i32)]
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

    use api::objects::{ConstructionSite, Creep, Flag, Mineral, Nuke, OwnedStructure, Resource,
                       RoomPosition, Source, Structure, StructureSpawn};

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
    }
}

impl TryFrom<Value> for Part {
    type Error = <Value as TryInto<u32>>::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let x: u32 = (js!(
            switch (@{v}) {
                case MOVE: return 0;
                case WORK: return 1;
                case CARRY: return 2;
                case ATTACK: return 3;
                case RANGED_ATTACK: return 4;
                case TOUGH: return 5;
                case HEAL: return 6;
                case CLAIM: return 7;
                default: return -1;
            }
        )).try_into()?;
        let res = match x {
            0 => Part::Move,
            1 => Part::Work,
            2 => Part::Carry,
            3 => Part::Attack,
            4 => Part::RangedAttack,
            5 => Part::Tough,
            6 => Part::Heal,
            7 => Part::Claim,
            _ => unreachable!(),
        };
        Ok(res)
    }
}

enum_from_primitive! {
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
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
}

impl TryFrom<Value> for ReturnCode {
    type Error = <Value as TryInto<i32>>::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        use num_traits::FromPrimitive;
        let x: i32 = v.try_into()?;
        Ok(Self::from_i32(x).unwrap())
    }
}

enum_from_primitive! {
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
