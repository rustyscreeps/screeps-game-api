//! Screeps object wrappers.
//!
//! Objects types that inherit [`RoomObject`] represent game objects with are
//! valid to be used only during the current tick; reading or writing a 'stale'
//! game object from a past tick will result in undefined behavior.
mod impls;

pub use event::*;
pub use game_types::*;
pub use input::*;
pub use output::*;
pub use room_objects::*;
pub use visual::*;

/// Object wrappers representing data retrieved from room event logs.
pub mod event {
    pub use super::impls::{
        AttackEvent, AttackType, BuildEvent, Event, EventType, ExitEvent, HarvestEvent, HealEvent,
        HealType, ObjectDestroyedEvent, PowerEvent, RepairEvent, ReserveControllerEvent,
        TransferEvent, UpgradeControllerEvent,
    };
}

/// Object wrappers for game types that are not room objects (are safe to use
/// in future ticks).
mod game_types {
    pub use super::impls::{CostMatrix, RoomPosition, RoomTerrain};
}

/// Object wrappers for simple javascript objects with known properties sent to
/// game functions.
pub mod input {
    pub use super::impls::{FindPathOptions, JsFindPathOptions, MoveToOptions};
}

/// Object wrappers for simple javascript objects with known properties returned
/// by game functions.
pub mod output {
    pub use super::impls::{
        AccountPowerCreep, BodyPart, Effect, InterShardPortalDestination, Owner, Path,
        PortalDestination, PowerInfo, Reservation, Sign, SpawnOptions, Step,
    };
}

/// Object wrappers for room objects.
mod room_objects {
    pub use super::impls::{
        ConstructionSite, Creep, Deposit, Flag, Mineral, Nuke, OwnedStructure, PowerCreep,
        Resource, Room, RoomObject, Ruin, Source, Spawning, Store, Structure, StructureContainer,
        StructureController, StructureExtension, StructureExtractor, StructureFactory,
        StructureInvaderCore, StructureKeeperLair, StructureLab, StructureLink, StructureNuker,
        StructureObserver, StructurePortal, StructurePowerBank, StructurePowerSpawn,
        StructureRampart, StructureRoad, StructureSpawn, StructureStorage, StructureTerminal,
        StructureTower, StructureWall, Tombstone,
    };

    #[cfg(feature = "score")]
    pub use super::impls::{ScoreCollector, ScoreContainer};

    #[cfg(feature = "symbols")]
    pub use super::impls::{SymbolContainer, SymbolDecoder};

    #[cfg(feature = "thorium")]
    pub use super::impls::Reactor;
}

/// Object wrappers allowing drawing of shapes in rooms or on the map in the
/// game world.
pub mod visual {
    pub use super::impls::{
        CircleStyle, FontStyle, LineDrawStyle, LineStyle, MapFontStyle, MapFontVariant,
        MapTextStyle, MapVisual, MapVisualShape, PolyStyle, RectStyle, RoomVisual, TextAlign,
        TextStyle, Visual,
    };
}
