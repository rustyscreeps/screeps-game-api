//! Screeps object wrappers.
mod impls;

pub use event::*;
pub use game_types::*;
pub use generic::*;
pub use room_objects::*;
pub use visual::*;

/// Object wrappers representing data retrieved from room event logs
pub mod event {
    pub use super::impls::{
        AttackEvent, AttackType, BuildEvent, Event, EventType, ExitEvent, HarvestEvent, HealEvent,
        HealType, ObjectDestroyedEvent, PowerEvent, RepairEvent, ReserveControllerEvent,
        TransferEvent, UpgradeControllerEvent,
    };
}

/// Object wrappers for game types that are not room objects and are safe to use
/// in future ticks.
pub mod game_types {
    pub use super::impls::{CostMatrix, RoomPosition, RoomTerrain};
}

/// Object wrappers for simple javascript objects with known properties sent to
/// or returned by game functions
pub mod generic {
    pub use super::impls::{
        AccountPowerCreep, BodyPart, CostMatrixSet, Effect, FindPathOptions, HasLocalPosition,
        InterShardPortalDestination, JsFindPathOptions, MoveToOptions, Owner, Path,
        PortalDestination, PowerInfo, Reservation, Sign, SpawnOptions, Step,
    };
}

/// Object wrappers representing room objects within the game world, which are
/// unsafe to use in future ticks
pub mod room_objects {
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
}

/// Object wrappers allowing drawing of shapes in rooms or on the map in the
/// game world
pub mod visual {
    pub use super::impls::{
        CircleStyle, FontStyle, LineDrawStyle, LineStyle, MapVisual, MapVisualShape, PolyStyle,
        RectStyle, RoomVisual, TextAlign, TextStyle, Visual,
    };
}
