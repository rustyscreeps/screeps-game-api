//! Screeps object wrappers.
mod impls;

pub use self::impls::{
    AccountPowerCreep, AttackEvent, AttackType, BodyPart, BuildEvent, ConstructionSite, CostMatrix,
    CostMatrixSet, Creep, Deposit, Effect, Event, EventType, ExitEvent, FindOptions, Flag,
    HarvestEvent, HasLocalPosition, HealEvent, HealType, InterShardPortalDestination,
    JsFindOptions, Mineral, MoveToOptions, Nuke, ObjectDestroyedEvent, OwnedStructure, Owner, Path,
    PortalDestination, PowerCreep, PowerEvent, PowerInfo, RepairEvent, Reservation,
    ReserveControllerEvent, Resource, Room, RoomObject, RoomPosition, RoomTerrain, Ruin, Sign,
    Source, SpawnOptions, Spawning, Step, Store, Structure, StructureContainer,
    StructureController, StructureExtension, StructureExtractor, StructureFactory,
    StructureInvaderCore, StructureKeeperLair, StructureLab, StructureLink, StructureNuker,
    StructureObserver, StructurePortal, StructurePowerBank, StructurePowerSpawn, StructureRampart,
    StructureRoad, StructureSpawn, StructureStorage, StructureTerminal, StructureTower,
    StructureWall, Tombstone, TransferEvent, UpgradeControllerEvent,
};

#[cfg(feature = "score")]
pub use self::impls::{ScoreCollector, ScoreContainer};

#[cfg(feature = "symbols")]
pub use self::impls::{SymbolContainer, SymbolDecoder};

pub use self::impls::{
    CircleStyle, FontStyle, LineDrawStyle, LineStyle, PolyStyle, RectStyle, RoomVisual, TextAlign,
    TextStyle, Visual,
};

pub use self::impls::{MapVisual, MapVisualShape};
