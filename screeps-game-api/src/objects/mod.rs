//! Screeps object wrappers.
//!
//! # Unsafe traits
//!
//! This module contains a number of unsafe traits. Each is unsafe purely to
//! prevent accidental implementations on things which don't uphold the trait
//! contracts that have not been put into code. There is no unsafe code
//! in this crate which relies on these traits being implemented correctly,
//! only code which will panic if they are not.
//!
//! Even though this crate does not contain any, other crate unsafe could
//! rely on these contracts being upheld as long as JavaScript code does not
//! do anything mischievous, like removing properties from objects or sticking
//! unexpected things into dictionaries which we trust.
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::{Reference, Value};

use {ResourceType, ReturnCode, StructureType, ConversionError};

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
    type Error = ConversionError;

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

/// Trait for things which have positions in the Screeps world.
///
/// This can be freely implemented for anything with a way to get a position.
pub trait HasPosition {
    fn pos(&self) -> RoomPosition;
}

impl HasPosition for RoomPosition {
    fn pos(&self) -> RoomPosition {
        self.clone()
    }
}

/// All RoomObjects have positions.
impl<T> HasPosition for T
where
    T: RoomObjectProperties,
{
    fn pos(&self) -> RoomPosition {
        js_unwrap!(@{self.as_ref()}.pos)
    }
}

/// Trait for all wrappers over Screeps JavaScript objects extending
/// the `RoomObject` class.
///
/// # Contracts
///
/// The reference returned by `AsRef<Reference>::as_ref` must reference a
/// JavaScript object extending the `RoomObject` class.
pub unsafe trait RoomObjectProperties:
    AsRef<Reference> + Into<Reference> + HasPosition
{
    fn try_from(obj: RoomObject) -> Option<Self>;

    fn room(&self) -> Room {
        js_unwrap!(@{self.as_ref()}.room)
    }
}

/// Trait for all wrappers over Screeps JavaScript objects extending
/// the `Structure` class.
///
/// # Contracts
///
/// The reference returned by `AsRef<Reference>::as_ref` must reference a
/// JavaScript object extending the `Structure` class.
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

/// Trait for all wrappers over Screeps JavaScript objects extending
/// the `OwnedStructure` class.
///
/// # Contracts
///
/// The reference returned by `AsRef<Reference>::as_ref` must reference a
/// JavaScript object extending the `OwnedStructure` class.
pub unsafe trait OwnedStructureProperties: StructureProperties {
    fn my(&self) -> bool {
        js_unwrap!(@{self.as_ref()}.my)
    }
    fn owner_name(&self) -> Option<String> {
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

/// Trait for all wrappers over Screeps JavaScript objects with a
/// `store` property.
///
/// # Contracts
///
/// The JavaScript object referenced by the return of `AsRef<Reference>::as_ref`
/// must have a `store` property. Additionally, if it does not have a
/// `storeCapacity` property, `HasStore::store_capacity` must be overridden.
///
/// The `store` property must be a dict from string resource types to integers.
///
/// If present, the `storeCapacity` property must be an integer.
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

/// Trait for objects which can only store energy.
/// 
/// # Contract
/// 
/// The reference returned from `AsRef<Reference>::as_ref` must be have an 
/// `energy` and an `energyCapacity` properties.
pub unsafe trait CanStoreEnergy: StructureProperties {
    fn energy(&self) -> u32 {
        js_unwrap! { @{self.as_ref()}.energy }
    }

    fn energy_capacity(&self) -> u32 {
        js_unwrap! { @{self.as_ref()}.energyCapacity }
    }
}

/// Trait for objects which have to cooldown.
/// 
/// # Contract
/// 
/// The reference returned from `AsRef<Reference>::as_ref` must be have a
/// `cooldown` properties.
pub unsafe trait HasCooldown: StructureProperties {
    fn cooldown(&self) -> u32 {
        js_unwrap! { @{self.as_ref()}.cooldown }
    }
}

/// Trait for objects which can decay.
/// 
/// # Contract
/// 
/// The reference returned from `AsRef<Reference>::as_ref` must be have a
/// `ticksToDecay` properties.
pub unsafe trait CanDecay: RoomObjectProperties {
    fn ticks_to_decay(&self) -> u32 {
        js_unwrap! { @{self.as_ref()}.ticksToDecay }
    }
}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.transfer`.
///
/// # Contracts
///
/// The reference returned from `AsRef<Reference>::as_ref` must be a valid
/// target for `Creep.transfer`.
pub unsafe trait Transferable: RoomObjectProperties {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.withdraw`.
///
/// # Contracts
///
/// The reference returned from `AsRef<Reference>::as_ref` must be a valid
/// target for `Creep.withdraw`.
pub unsafe trait Withdrawable: RoomObjectProperties {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.attack`.
///
/// # Contracts
///
/// The reference returned from `AsRef<Reference>::as_ref` must be a valid
/// target for `Creep.attack`.
pub unsafe trait Attackable: RoomObjectProperties {}

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

unsafe impl CanStoreEnergy for StructureExtension {}
unsafe impl CanStoreEnergy for StructureLab {}
unsafe impl CanStoreEnergy for StructureLink {}
unsafe impl CanStoreEnergy for StructureNuker {}
unsafe impl CanStoreEnergy for StructurePowerSpawn {}
unsafe impl CanStoreEnergy for StructureSpawn {}
unsafe impl CanStoreEnergy for StructureTower {}

unsafe impl HasCooldown for StructureExtractor {}
unsafe impl HasCooldown for StructureLab {}
unsafe impl HasCooldown for StructureLink {}
unsafe impl HasCooldown for StructureNuker {}
unsafe impl HasCooldown for StructureTerminal {}

unsafe impl CanDecay for StructureContainer {}
unsafe impl CanDecay for StructurePowerBank {}
unsafe impl CanDecay for StructurePortal {}
unsafe impl CanDecay for StructureRampart {}
unsafe impl CanDecay for StructureRoad {}
unsafe impl CanDecay for Tombstone {}

simple_accessors! {
    StructureKeeperLair;
    (ticks_to_spawn -> ticksToSpawn -> u32),
}

simple_accessors! {
    StructurePowerBank;
    (power -> power -> u32),
}
