use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

use {ResourceType, ReturnCode, StructureType};

#[macro_use]
mod macros;
mod impls;

pub use self::impls::SpawnOptions;

reference_wrappers!(
    ConstructionSite,
    Creep,
    Flag,
    Mineral,
    Nuke,
    OwnedStructure,
    Resource,
    Room,
    RoomObject,
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
    Tombstone,
);

pub trait HasPosition {
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
        js_unwrap!(@{self.as_ref()}.pos)
    }
}

pub unsafe trait RoomObjectProperties
    : AsRef<Reference> + Into<Reference> + HasPosition {
    fn try_from(obj: RoomObject) -> Option<Self>;

    fn room(&self) -> Room {
        js_unwrap!(@{self.as_ref()}.room)
    }
}

pub unsafe trait StructureProperties: RoomObjectProperties {
    fn hits(&self) -> i32 {
        js_unwrap!(@{self.as_ref()}.hits)
    }
    fn hits_max(&self) -> i32 {
        js_unwrap!(@{self.as_ref()}.hitsMax)
    }
    fn id(&self) -> String {
        js_unwrap!(@{self.as_ref()}.id)
    }
    fn structure_type(&self) -> StructureType {
        js_unwrap!(@{self.as_ref()}.structureType)
    }
    fn destroy(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.destroy())
    }
    fn is_active(&self) -> bool {
        js_unwrap!(@{self.as_ref()}.isActive())
    }
    /// Usable on either owned structures or neutral structures in owned rooms,
    /// returns `ReturnCode::NotOwner` otherwise.
    fn notify_when_attacked(&self, notify_when_attacked: bool) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.notifyWhenAttacked(@{notify_when_attacked}))
    }
    fn as_structure(self) -> Structure {
        Structure(self.into())
    }
}

pub unsafe trait OwnedStructureProperties: RoomObjectProperties {
    fn my(&self) -> bool {
        js_unwrap!(@{self.as_ref()}.my)
    }
    fn owner(&self) -> Option<String> {
        (js! {
            var self = @{self.as_ref()};
            if (self.owner) {
                return self.owner.username;
            } else {
                return null;
            }
        }).try_into()
            .expect("expected OwnedStructure.owner.username to be a string")
    }
    fn as_owned_structure(self) -> OwnedStructure {
        OwnedStructure(self.into())
    }
}

pub unsafe trait HasStore: RoomObjectProperties {
    fn store_total(&self) -> i32 {
        js_unwrap!(_.sum(@{self.as_ref()}.store))
    }

    fn store_types(&self) -> Vec<ResourceType> {
        js_unwrap!(Object.keys(@{self.as_ref()}.store).map(__resource_type_str_to_num))
    }

    fn store_of(&self, ty: ResourceType) -> i32 {
        js_unwrap!(@{self.as_ref()}.store[__resource_type_num_to_str(@{ty as i32})] || 0)
    }

    fn energy(&self) -> i32 {
        js_unwrap!(@{self.as_ref()}.store[RESOURCE_ENERGY])
    }

    fn store_capacity(&self) -> i32 {
        js_unwrap!(@{self.as_ref()}.storeCapacity)
    }
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
pub unsafe trait Withdrawable: RoomObjectProperties {}
unsafe impl Withdrawable for StructureExtension {}
unsafe impl Withdrawable for StructureContainer {}
unsafe impl Withdrawable for StructureLab {}
unsafe impl Withdrawable for StructureLink {}
unsafe impl Withdrawable for StructureSpawn {}
unsafe impl Withdrawable for StructureStorage {}
unsafe impl Withdrawable for StructureTower {}
unsafe impl Withdrawable for StructurePowerSpawn {}
unsafe impl Withdrawable for StructureTerminal {}
unsafe impl Withdrawable for Tombstone {}

pub unsafe trait Attackable: RoomObjectProperties {}
unsafe impl<T> Attackable for T
where
    T: StructureProperties,
{
}
unsafe impl Attackable for Creep {}

impl_room_object_properties! {
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
    Tombstone,
}

unsafe impl StructureProperties for OwnedStructure {}
unsafe impl StructureProperties for Structure {}
unsafe impl StructureProperties for StructureContainer {}
unsafe impl StructureProperties for StructureController {}
unsafe impl StructureProperties for StructureExtension {}
unsafe impl StructureProperties for StructureExtractor {}
unsafe impl StructureProperties for StructureKeeperLair {}
unsafe impl StructureProperties for StructureLab {}
unsafe impl StructureProperties for StructureLink {}
unsafe impl StructureProperties for StructureNuker {}
unsafe impl StructureProperties for StructureObserver {}
unsafe impl StructureProperties for StructurePowerBank {}
unsafe impl StructureProperties for StructurePowerSpawn {}
unsafe impl StructureProperties for StructurePortal {}
unsafe impl StructureProperties for StructureRampart {}
unsafe impl StructureProperties for StructureRoad {}
unsafe impl StructureProperties for StructureSpawn {}
unsafe impl StructureProperties for StructureStorage {}
unsafe impl StructureProperties for StructureTerminal {}
unsafe impl StructureProperties for StructureTower {}
unsafe impl StructureProperties for StructureWall {}

unsafe impl OwnedStructureProperties for OwnedStructure {}
unsafe impl OwnedStructureProperties for StructureController {}
unsafe impl OwnedStructureProperties for StructureExtension {}
unsafe impl OwnedStructureProperties for StructureExtractor {}
unsafe impl OwnedStructureProperties for StructureKeeperLair {}
unsafe impl OwnedStructureProperties for StructureLab {}
unsafe impl OwnedStructureProperties for StructureLink {}
unsafe impl OwnedStructureProperties for StructureNuker {}
unsafe impl OwnedStructureProperties for StructureObserver {}
unsafe impl OwnedStructureProperties for StructurePowerBank {}
unsafe impl OwnedStructureProperties for StructurePowerSpawn {}
unsafe impl OwnedStructureProperties for StructurePortal {}
unsafe impl OwnedStructureProperties for StructureRampart {}
unsafe impl OwnedStructureProperties for StructureSpawn {}
unsafe impl OwnedStructureProperties for StructureStorage {}
unsafe impl OwnedStructureProperties for StructureTerminal {}
unsafe impl OwnedStructureProperties for StructureTower {}

unsafe impl HasStore for StructureContainer {}
unsafe impl HasStore for StructureStorage {}
unsafe impl HasStore for StructureTerminal {}
unsafe impl HasStore for Tombstone {
    fn store_capacity(&self) -> i32 {
        0 // no storeCapacity property
    }
}
