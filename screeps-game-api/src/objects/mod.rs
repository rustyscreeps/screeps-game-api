use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

use {ReturnCode, StructureType};

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
    fn structure_type(&self) -> StructureType;
}

pub trait OwnedStructureProperties: RoomObjectProperties {
    fn my(&self) -> bool;
    fn owner(&self) -> Option<String>;
}

pub unsafe trait Transferable: RoomObjectProperties {}
unsafe impl Transferable for StructureExtension {}
unsafe impl Transferable for Creep {}
unsafe impl Transferable for StructureContainer {}
unsafe impl Transferable for StructureLab {}
unsafe impl Transferable for StructureLink {}
unsafe impl Transferable for StructureNuker {}
unsafe impl Transferable for StructureSpawn {}
unsafe impl Transferable for StructureStorage {}
unsafe impl Transferable for StructureTower {}
unsafe impl Transferable for StructurePowerSpawn {}
unsafe impl Transferable for StructureTerminal {}
pub unsafe trait Withdrawable: Transferable {}
unsafe impl Withdrawable for StructureExtension {}
unsafe impl Withdrawable for StructureContainer {}
unsafe impl Withdrawable for StructureLab {}
unsafe impl Withdrawable for StructureLink {}
unsafe impl Withdrawable for StructureSpawn {}
unsafe impl Withdrawable for StructureStorage {}
unsafe impl Withdrawable for StructureTower {}
unsafe impl Withdrawable for StructurePowerSpawn {}
unsafe impl Withdrawable for StructureTerminal {}

pub unsafe trait Attackable: RoomObjectProperties {}
unsafe impl<T> Attackable for T
where
    T: StructureProperties,
{
}
unsafe impl Attackable for Creep {}

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
