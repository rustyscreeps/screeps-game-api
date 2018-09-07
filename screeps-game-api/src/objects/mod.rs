use stdweb::unstable::{TryFrom, TryInto};
use stdweb::{Reference, Value};

use {ResourceType, ReturnCode, StructureType};

#[macro_use]
mod macros;
mod impls;

pub use self::impls::SpawnOptions;
pub use self::impls::{Reservation, Sign};

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
    // this is implemented later
    //    Structure,
    Tombstone,
);

pub enum Structure {
    Container(StructureContainer),
    Controller(StructureController),
    Extension(StructureExtension),
    Extractor(StructureExtractor),
    KeeperLair(StructureKeeperLair),
    Lab(StructureLab),
    Link(StructureLink),
    Nuker(StructureNuker),
    Observer(StructureObserver),
    PowerBank(StructurePowerBank),
    PowerSpawn(StructurePowerSpawn),
    Portal(StructurePortal),
    Rampart(StructureRampart),
    Road(StructureRoad),
    Spawn(StructureSpawn),
    Storage(StructureStorage),
    Terminal(StructureTerminal),
    Tower(StructureTower),
    Wall(StructureWall),
}

impl AsRef<Reference> for Structure {
    fn as_ref(&self) -> &Reference {
        match *self {
            Structure::Container(ref v) => v.as_ref(),
            Structure::Controller(ref v) => v.as_ref(),
            Structure::Extension(ref v) => v.as_ref(),
            Structure::Extractor(ref v) => v.as_ref(),
            Structure::KeeperLair(ref v) => v.as_ref(),
            Structure::Lab(ref v) => v.as_ref(),
            Structure::Link(ref v) => v.as_ref(),
            Structure::Nuker(ref v) => v.as_ref(),
            Structure::Observer(ref v) => v.as_ref(),
            Structure::PowerBank(ref v) => v.as_ref(),
            Structure::PowerSpawn(ref v) => v.as_ref(),
            Structure::Portal(ref v) => v.as_ref(),
            Structure::Rampart(ref v) => v.as_ref(),
            Structure::Road(ref v) => v.as_ref(),
            Structure::Spawn(ref v) => v.as_ref(),
            Structure::Storage(ref v) => v.as_ref(),
            Structure::Terminal(ref v) => v.as_ref(),
            Structure::Tower(ref v) => v.as_ref(),
            Structure::Wall(ref v) => v.as_ref(),
        }
    }
}
impl From<Structure> for Reference {
    fn from(wrapper: Structure) -> Reference {
        match wrapper {
            Structure::Container(v) => v.0,
            Structure::Controller(v) => v.0,
            Structure::Extension(v) => v.0,
            Structure::Extractor(v) => v.0,
            Structure::KeeperLair(v) => v.0,
            Structure::Lab(v) => v.0,
            Structure::Link(v) => v.0,
            Structure::Nuker(v) => v.0,
            Structure::Observer(v) => v.0,
            Structure::PowerBank(v) => v.0,
            Structure::PowerSpawn(v) => v.0,
            Structure::Portal(v) => v.0,
            Structure::Rampart(v) => v.0,
            Structure::Road(v) => v.0,
            Structure::Spawn(v) => v.0,
            Structure::Storage(v) => v.0,
            Structure::Terminal(v) => v.0,
            Structure::Tower(v) => v.0,
            Structure::Wall(v) => v.0,
        }
    }
}

impl Structure {
    fn from_reference(reference: Reference) -> Self {
        let s = js_unwrap!(@{&reference}.structureType);
        match s {
            StructureType::Container => Structure::Container(StructureContainer(reference)),
            StructureType::Controller => Structure::Controller(StructureController(reference)),
            StructureType::Extension => Structure::Extension(StructureExtension(reference)),
            StructureType::Extractor => Structure::Extractor(StructureExtractor(reference)),
            StructureType::KeeperLair => Structure::KeeperLair(StructureKeeperLair(reference)),
            StructureType::Lab => Structure::Lab(StructureLab(reference)),
            StructureType::Link => Structure::Link(StructureLink(reference)),
            StructureType::Nuker => Structure::Nuker(StructureNuker(reference)),
            StructureType::Observer => Structure::Observer(StructureObserver(reference)),
            StructureType::PowerBank => Structure::PowerBank(StructurePowerBank(reference)),
            StructureType::PowerSpawn => Structure::PowerSpawn(StructurePowerSpawn(reference)),
            StructureType::Portal => Structure::Portal(StructurePortal(reference)),
            StructureType::Rampart => Structure::Rampart(StructureRampart(reference)),
            StructureType::Road => Structure::Road(StructureRoad(reference)),
            StructureType::Spawn => Structure::Spawn(StructureSpawn(reference)),
            StructureType::Storage => Structure::Storage(StructureStorage(reference)),
            StructureType::Terminal => Structure::Terminal(StructureTerminal(reference)),
            StructureType::Tower => Structure::Tower(StructureTower(reference)),
            StructureType::Wall => Structure::Wall(StructureWall(reference)),
        }
    }
}

impl TryFrom<Value> for Structure {
    type Error = <Value as TryInto<Reference>>::Error;

    fn try_from(v: Value) -> Result<Structure, Self::Error> {
        Ok(Self::from_reference(v.try_into()?))
    }
}

unsafe impl RoomObjectProperties for Structure {
    fn try_from(obj: RoomObject) -> Option<Self> {
        let is_me = js_unwrap!(@{obj.as_ref()} instanceof Structure);
        if is_me {
            Some(Self::from_reference(obj.0))
        } else {
            None
        }
    }
}

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

pub unsafe trait RoomObjectProperties:
    AsRef<Reference> + Into<Reference> + HasPosition
{
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
        Structure::from_reference(self.into())
    }
}

pub unsafe trait OwnedStructureProperties: StructureProperties {
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
unsafe impl<T> Attackable for T where T: StructureProperties {}
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

impl_structure_properties!{
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
    StructureWall
}

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
