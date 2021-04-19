use wasm_bindgen::prelude::*;
use crate::{objects::*};
use crate::enums::StructureObject;

/// Translates `FIND_*` constants.
#[wasm_bindgen]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash,
)]
#[repr(u16)]
pub enum Find {
    /// Find all exit positions at the top of the room
    ExitTop = 1,
    ExitRight = 3,
    ExitBottom = 5,
    ExitLeft = 7,
    Exit = 10,
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
    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,
    Deposits = 122,
    Ruins = 123,
    // todo these seem to not work when conditionally compiled out - they're not hurting to leave
    // in but need to figure that out
    //#[cfg(feature = "enable-score")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainers = 10011,
    //#[cfg(feature = "enable-score")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollectors = 10012,
    //#[cfg(feature = "enable-symbols")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    SymbolContainers = 10021,
    //#[cfg(feature = "enable-symbols")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    SymbolDecoders = 10022,
}

/// Trait representing things which can be used in the 'find' function.
///
/// Typically used with zero-sized structs in the
/// [`find`][crate::constants::find] module.
pub trait FindConstant {
    type Item: From<JsValue>;

    fn find_code(&self) -> Find;
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
    Debug, PartialEq, Eq, Clone, Copy, Hash,
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
    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,
    Deposits = 122,
    Ruins = 123,
}

impl Into<Find> for RoomObject {
    fn into(self) -> Find {
        match self {
            RoomObject::Creeps => Find::Creeps,
            RoomObject::MyCreeps => Find::MyCreeps,
            RoomObject::HostileCreeps => Find::HostileCreeps,
            RoomObject::SourcesActive => Find::SourcesActive,
            RoomObject::Sources => Find::Sources,
            RoomObject::DroppedResources => Find::DroppedResources,
            RoomObject::Structures => Find::Structures,
            RoomObject::MyStructures => Find::MyStructures,
            RoomObject::HostileStructures => Find::HostileStructures,
            RoomObject::Flags => Find::Flags,
            RoomObject::ConstructionSites => Find::ConstructionSites,
            RoomObject::MySpawns => Find::MySpawns,
            RoomObject::HostileSpawns => Find::HostileSpawns,
            RoomObject::MyConstructionSites => Find::MyConstructionSites,
            RoomObject::HostileConstructionSites => Find::HostileConstructionSites,
            RoomObject::Minerals => Find::Minerals,
            RoomObject::Nukes => Find::Nukes,
            RoomObject::Tombstones => Find::Tombstones,
            RoomObject::PowerCreeps => Find::PowerCreeps,
            RoomObject::MyPowerCreeps => Find::MyPowerCreeps,
            RoomObject::HostilePowerCreeps => Find::HostilePowerCreeps,
            RoomObject::Deposits => Find::Deposits,
            RoomObject::Ruins => Find::Ruins,
        }
    }
}

impl FindConstant for RoomObject {
    type Item = crate::objects::RoomObject;

    #[inline]
    fn find_code(&self) -> Find {
        (*self).into()
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash,
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

impl Into<Find> for Exit {
    fn into(self) -> Find {
        match self {
            Exit::Top => Find::ExitTop,
            Exit::Right => Find::ExitRight,
            Exit::Bottom => Find::ExitBottom,
            Exit::Left => Find::ExitLeft,
            Exit::All => Find::Exit
        }
    }
}

impl FindConstant for Exit {
    //TODO: wiarchbe: Check this is correct?
    type Item = RoomPosition;

    #[inline]
    fn find_code(&self) -> Find {
        (*self).into()
    }
}

//TODO: wiarchbe: Add back in calculated doc.
macro_rules! typesafe_find_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;            
            impl FindConstant for $constant_name {
                type Item = $result;               

                #[inline]
                fn find_code(&self) -> Find {
                    $value
                }
            }
        )*
    );
}

typesafe_find_constants! {
    pub struct CREEPS = (Find::Creeps, Creep);
    pub struct MY_CREEPS = (Find::MyCreeps, Creep);
    pub struct HOSTILE_CREEPS = (Find::HostileCreeps, Creep);
    pub struct SOURCES_ACTIVE = (Find::SourcesActive, Source);
    pub struct SOURCES = (Find::Sources, Source);
    pub struct DROPPED_RESOURCES = (Find::DroppedResources, Resource);
    pub struct STRUCTURES = (Find::Structures, StructureObject);
    pub struct MY_STRUCTURES = (Find::MyStructures, StructureObject);
    pub struct HOSTILE_STRUCTURES = (Find::HostileStructures, StructureObject);
    pub struct FLAGS = (Find::Flags, Flag);
    pub struct CONSTRUCTION_SITES = (Find::ConstructionSites, ConstructionSite);
    pub struct MY_SPAWNS = (Find::MySpawns, StructureSpawn);
    pub struct HOSTILE_SPAWNS = (Find::HostileSpawns, StructureSpawn);
    pub struct MY_CONSTRUCTION_SITES = (Find::MyConstructionSites, ConstructionSite);
    pub struct HOSTILE_CONSTRUCTION_SITES = (Find::HostileConstructionSites, ConstructionSite);
    pub struct MINERALS = (Find::Minerals, Mineral);
    pub struct NUKES = (Find::Nukes, Nuke);
    pub struct TOMBSTONES = (Find::Tombstones, Tombstone);
    pub struct POWER_CREEPS = (Find::PowerCreeps, PowerCreep);
    pub struct MY_POWER_CREEPS = (Find::MyPowerCreeps, PowerCreep);
    pub struct HOSTILE_POWER_CREEPS = (Find::HostilePowerCreeps, PowerCreep);
    pub struct DEPOSITS = (Find::Deposits, Deposit);
    pub struct RUINS = (Find::Ruins, Ruin);
    pub struct EXIT_TOP = (Find::ExitTop, RoomPosition);
    pub struct EXIT_RIGHT = (Find::ExitRight, RoomPosition);
    pub struct EXIT_BOTTOM = (Find::ExitBottom, RoomPosition);
    pub struct EXIT_LEFT = (Find::ExitLeft, RoomPosition);
    pub struct EXIT = (Find::Exit, RoomPosition);
}