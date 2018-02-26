use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

use api::ReturnCode;

#[macro_use]
mod macros;
mod impls;

reference_wrappers!(
    ConstructionSite,
    Creep,
    Flag,
    Mineral,
    Nuke,
    OwnedStructure,
    Resource,
    Room,
    RoomPosition,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructurePortal,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    Structure,
    RoomObject,
);

pub trait HasPosition: AsRef<Reference> {
    fn pos(&self) -> RoomPosition;
}

impl HasPosition for RoomPosition {
    fn pos(&self) -> RoomPosition {
        self.clone()
    }
}

impl<T> HasPosition for T
where
    T: RoomObjectProperties,
{
    fn pos(&self) -> RoomPosition {
        RoomObjectProperties::pos(self)
    }
}

pub trait RoomObjectProperties: AsRef<Reference> {
    fn pos(&self) -> RoomPosition;
    fn room(&self) -> Room;
}

pub trait StructureProperties: RoomObjectProperties {
    fn hits(&self) -> i32;
    fn hits_max(&self) -> i32;
    fn id(&self) -> String;
    // TODO: StructureType
    fn destroy(&self) -> ReturnCode;
    fn is_active(&self) -> bool;
}

pub trait OwnedStructureProperties: RoomObjectProperties {
    fn my(&self) -> bool;
    fn owner(&self) -> Option<String>;
}

impl<'a, T: RoomObjectProperties> RoomObjectProperties for &'a T {
    fn pos(&self) -> RoomPosition {
        <T as RoomObjectProperties>::pos(self)
    }
    fn room(&self) -> Room {
        <T as RoomObjectProperties>::room(self)
    }
}

impl_room_object!(
    ConstructionSite,
    Creep,
    Flag,
    Mineral,
    Nuke,
    OwnedStructure,
    Resource,
    RoomObject,
    Source,
    Structure,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructurePortal,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
);

impl_structure!(
    OwnedStructure,
    Structure,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructurePortal,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
);

impl_owned_structure!(
    OwnedStructure,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructurePortal,
    StructureRampart,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
);
