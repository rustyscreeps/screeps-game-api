//! Constants for use with the [`Room::find`] function.
//!
//! *Note:* Types in this module have purposefully ambiguous names, and are
//! intended to be used as, for example, `find::CREEPS`, not `CREEPS`.
//!
//! You can do this my importing the module itself, rather than any individual
//! constant, and then just referring to the constants relative to the module.
//!
//! # Example
//!
//! ```no_run
//! use screeps::{find, game, Room};
//!
//! let room: Room = game::rooms().get("E23S55".parse().unwrap()).unwrap();
//!
//! let creeps = room.find(find::CREEPS, None);
//! # let _ = creeps;
//! ```
//!
//! [`Room::find`]: crate::Room::find
//! [`objects::RoomObject`]: crate::RoomObject
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{enums::StructureObject, objects::*};

/// Translates `FIND_*` constants for interal API calls
///
/// Unless you're storing the type of find constant to be used for a call, you
/// likely want the constants which implement the `FindConstant` trait to make
/// calls to find methods.
///
/// This is hidden from the documentation to avoid confusion due to its narrow
/// use case, but wasm_bindgen requires it remain public.
#[doc(hidden)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
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
    //#[cfg(feature = "score")]
    ScoreContainers = 10011,
    //#[cfg(feature = "score")]
    ScoreCollectors = 10012,
    //#[cfg(feature = "symbols")]
    SymbolContainers = 10021,
    //#[cfg(feature = "symbols")]
    SymbolDecoders = 10022,
    //#[cfg(feature = "thorium")]
    Reactors = 10051,
}

/// Trait representing things which can be used in the 'find' function.
///
/// Typically used with zero-sized structs in the
/// [`find`][crate::constants::find] module.
pub trait FindConstant {
    type Item: From<JsValue>;

    fn convert_and_check_item(reference: JsValue) -> Self::Item;

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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    #[cfg(feature = "score")]
    ScoreContainers = 10011,
    #[cfg(feature = "score")]
    ScoreCollectors = 10012,
    #[cfg(feature = "symbols")]
    SymbolContainers = 10021,
    #[cfg(feature = "symbols")]
    SymbolDecoders = 10022,
    #[cfg(feature = "thorium")]
    Reactors = 10051,
}

impl From<RoomObject> for Find {
    fn from(obj: RoomObject) -> Find {
        match obj {
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
            #[cfg(feature = "score")]
            RoomObject::ScoreContainers => Find::ScoreContainers,
            #[cfg(feature = "score")]
            RoomObject::ScoreCollectors => Find::ScoreCollectors,
            #[cfg(feature = "symbols")]
            RoomObject::SymbolContainers => Find::SymbolContainers,
            #[cfg(feature = "symbols")]
            RoomObject::SymbolDecoders => Find::SymbolDecoders,
            #[cfg(feature = "thorium")]
            RoomObject::Reactors => Find::Reactors,
        }
    }
}

impl FindConstant for RoomObject {
    type Item = crate::objects::RoomObject;

    fn convert_and_check_item(reference: JsValue) -> Self::Item {
        Into::into(reference)
    }

    #[inline]
    fn find_code(&self) -> Find {
        (*self).into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl From<Exit> for Find {
    fn from(exit: Exit) -> Find {
        match exit {
            Exit::Top => Find::ExitTop,
            Exit::Right => Find::ExitRight,
            Exit::Bottom => Find::ExitBottom,
            Exit::Left => Find::ExitLeft,
            Exit::All => Find::Exit,
        }
    }
}

impl FindConstant for Exit {
    //TODO: wiarchbe: Check this is correct?
    type Item = RoomPosition;

    fn convert_and_check_item(reference: JsValue) -> Self::Item {
        Into::into(reference)
    }

    #[inline]
    fn find_code(&self) -> Find {
        (*self).into()
    }
}

//TODO: wiarchbe: Add back in calculated doc.
macro_rules! typesafe_find_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path, $conversion_method:expr);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;
            impl FindConstant for $constant_name {
                type Item = $result;

                fn convert_and_check_item(reference: JsValue) -> Self::Item {
                    $conversion_method(reference)
                }

                #[inline]
                fn find_code(&self) -> Find {
                    $value
                }
            }
        )*
    );
}

typesafe_find_constants! {
    pub struct CREEPS = (Find::Creeps, Creep, Into::into);
    pub struct MY_CREEPS = (Find::MyCreeps, Creep, Into::into);
    pub struct HOSTILE_CREEPS = (Find::HostileCreeps, Creep, Into::into);
    pub struct SOURCES_ACTIVE = (Find::SourcesActive, Source, Into::into);
    pub struct SOURCES = (Find::Sources, Source, Into::into);
    pub struct DROPPED_RESOURCES = (Find::DroppedResources, Resource, Into::into);
    pub struct STRUCTURES = (Find::Structures, StructureObject, Into::into);
    pub struct MY_STRUCTURES = (Find::MyStructures, StructureObject, Into::into);
    pub struct HOSTILE_STRUCTURES = (Find::HostileStructures, StructureObject, Into::into);
    pub struct FLAGS = (Find::Flags, Flag, Into::into);
    pub struct CONSTRUCTION_SITES = (Find::ConstructionSites, ConstructionSite, Into::into);
    pub struct MY_SPAWNS = (Find::MySpawns, StructureSpawn, Into::into);
    pub struct HOSTILE_SPAWNS = (Find::HostileSpawns, StructureSpawn, Into::into);
    pub struct MY_CONSTRUCTION_SITES = (Find::MyConstructionSites, ConstructionSite, Into::into);
    pub struct HOSTILE_CONSTRUCTION_SITES = (Find::HostileConstructionSites, ConstructionSite, Into::into);
    pub struct MINERALS = (Find::Minerals, Mineral, Into::into);
    pub struct NUKES = (Find::Nukes, Nuke, Into::into);
    pub struct TOMBSTONES = (Find::Tombstones, Tombstone, Into::into);
    pub struct POWER_CREEPS = (Find::PowerCreeps, PowerCreep, Into::into);
    pub struct MY_POWER_CREEPS = (Find::MyPowerCreeps, PowerCreep, Into::into);
    pub struct HOSTILE_POWER_CREEPS = (Find::HostilePowerCreeps, PowerCreep, Into::into);
    pub struct DEPOSITS = (Find::Deposits, Deposit, Into::into);
    pub struct RUINS = (Find::Ruins, Ruin, Into::into);
    pub struct EXIT_TOP = (Find::ExitTop, RoomPosition, Into::into);
    pub struct EXIT_RIGHT = (Find::ExitRight, RoomPosition, Into::into);
    pub struct EXIT_BOTTOM = (Find::ExitBottom, RoomPosition, Into::into);
    pub struct EXIT_LEFT = (Find::ExitLeft, RoomPosition, Into::into);
    pub struct EXIT = (Find::Exit, RoomPosition, Into::into);
}

#[cfg(feature = "score")]
typesafe_find_constants! {
    pub struct SCORE_CONTAINERS = (Find::ScoreContainers, ScoreContainer, Into::into);
    pub struct SCORE_COLLECTORS = (Find::ScoreCollectors, ScoreCollector, Into::into);
}

#[cfg(feature = "symbols")]
typesafe_find_constants! {
    pub struct SYMBOL_CONTAINERS = (Find::SymbolContainers, SymbolContainer, Into::into);
    pub struct SYMBOL_DECODERS = (Find::SymbolDecoders, SymbolDecoder, Into::into);
}

#[cfg(feature = "thorium")]
typesafe_find_constants! {
    pub struct REACTORS = (Find::Reactors, Reactor, Into::into);
}
