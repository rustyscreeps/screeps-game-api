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
use stdweb::{Reference, ReferenceType, Value};

use {
    constants::{ResourceType, ReturnCode, StructureType},
    traits::{IntoExpectedType, TryFrom, TryInto},
    ConversionError,
};

mod impls;
mod structure;

pub use self::{
    impls::{
        AttackEvent, AttackType, BuildEvent, Event, EventType, ExitEvent, FindOptions,
        HarvestEvent, HealEvent, HealType, LookResult, ObjectDestroyedEvent, Path,
        PositionedLookResult, RepairEvent, Reservation, ReserveControllerEvent, Sign, SpawnOptions,
        Step, UpgradeControllerEvent,
    },
    structure::Structure,
};

reference_wrappers!(
    #[reference(instance_of = "ConstructionSite")]
    ConstructionSite,
    #[reference(instance_of = "Creep")]
    Creep,
    #[reference(instance_of = "Flag")]
    Flag,
    #[reference(instance_of = "Mineral")]
    Mineral,
    #[reference(instance_of = "Nuke")]
    Nuke,
    #[reference(instance_of = "OwnedStructure")]
    OwnedStructure,
    #[reference(instance_of = "Resource")]
    Resource,
    #[reference(instance_of = "Room")]
    Room,
    #[reference(instance_of = "RoomObject")]
    RoomObject,
    #[reference(instance_of = "RoomPosition")]
    RoomPosition,
    #[reference(instance_of = "Room.Terrain")]
    RoomTerrain,
    #[reference(instance_of = "Source")]
    Source,
    #[reference(instance_of = "StructureContainer")]
    StructureContainer,
    #[reference(instance_of = "StructureController")]
    StructureController,
    #[reference(instance_of = "StructureExtension")]
    StructureExtension,
    #[reference(instance_of = "StructureExtractor")]
    StructureExtractor,
    #[reference(instance_of = "StructureKeeperLair")]
    StructureKeeperLair,
    #[reference(instance_of = "StructureLab")]
    StructureLab,
    #[reference(instance_of = "StructureLink")]
    StructureLink,
    #[reference(instance_of = "StructureNuker")]
    StructureNuker,
    #[reference(instance_of = "StructureObserver")]
    StructureObserver,
    #[reference(instance_of = "StructurePowerBank")]
    StructurePowerBank,
    #[reference(instance_of = "StructurePowerSpawn")]
    StructurePowerSpawn,
    #[reference(instance_of = "StructurePortal")]
    StructurePortal,
    #[reference(instance_of = "StructureRampart")]
    StructureRampart,
    #[reference(instance_of = "StructureRoad")]
    StructureRoad,
    #[reference(instance_of = "StructureSpawn")]
    StructureSpawn,
    #[reference(instance_of = "Spawning")]
    Spawning,
    #[reference(instance_of = "StructureStorage")]
    StructureStorage,
    #[reference(instance_of = "StructureTerminal")]
    StructureTerminal,
    #[reference(instance_of = "StructureTower")]
    StructureTower,
    #[reference(instance_of = "StructureWall")]
    StructureWall,
    // this is implemented later
    // #[reference(instance_of = "Structure")]
    // Structure,
    #[reference(instance_of = "Tombstone")]
    Tombstone,
);

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

/// All `RoomObject`s have positions.
impl<T> HasPosition for T
where
    T: RoomObjectProperties,
{
    fn pos(&self) -> RoomPosition {
        js_unwrap_ref!(@{self.as_ref()}.pos)
    }
}

/// Trait covering all objects with an id.
pub unsafe trait HasId: RoomObjectProperties {
    fn id(&self) -> String {
        js_unwrap!(@{self.as_ref()}.id)
    }
}

impl_has_id! {
    ConstructionSite;
    Creep;
    Mineral;
    Nuke;
    Resource;
    Source;
    OwnedStructure;
    Structure;
    StructureContainer;
    StructureController;
    StructureExtension;
    StructureExtractor;
    StructureKeeperLair;
    StructureLab;
    StructureLink;
    StructureNuker;
    StructureObserver;
    StructurePowerBank;
    StructurePowerSpawn;
    StructurePortal;
    StructureRampart;
    StructureRoad;
    StructureSpawn;
    StructureStorage;
    StructureTerminal;
    StructureTower;
    StructureWall;
    Tombstone;
}

/// Trait for all wrappers over Screeps JavaScript objects extending
/// the `RoomObject` class.
///
/// # Contracts
///
/// The reference returned by `AsRef<Reference>::as_ref` must reference a
/// JavaScript object extending the `RoomObject` class.
pub unsafe trait RoomObjectProperties:
    AsRef<Reference>
    + Into<Reference>
    + HasPosition
    + ReferenceType
    + TryFrom<Value, Error = ConversionError>
    + TryFrom<Reference, Error = ConversionError>
{
    fn room(&self) -> Room {
        js_unwrap_ref!(@{self.as_ref()}.room)
    }
}

/// Trait for all wrappers over Screeps JavaScript objects extending
/// the `Structure` class.
///
/// # Contracts
///
/// The reference returned by `AsRef<Reference>::as_ref` must reference a
/// JavaScript object extending the `Structure` class.
pub unsafe trait StructureProperties: RoomObjectProperties + HasId {
    fn structure_type(&self) -> StructureType {
        js_unwrap!(__structure_type_str_to_num(@{self.as_ref()}.structureType))
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
        Into::<Reference>::into(self)
            .into_expected_type()
            .expect("expected converting a StructureProperties to a Structure would suceed.")
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
        })
        .try_into()
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
    fn store_total(&self) -> u32 {
        js_unwrap!(_.sum(@{self.as_ref()}.store))
    }

    fn store_types(&self) -> Vec<ResourceType> {
        js_unwrap!(Object.keys(@{self.as_ref()}.store).map(__resource_type_str_to_num))
    }

    fn store_of(&self, ty: ResourceType) -> u32 {
        js_unwrap!(@{self.as_ref()}.store[__resource_type_num_to_str(@{ty as u32})] || 0)
    }

    fn energy(&self) -> u32 {
        js_unwrap!(@{self.as_ref()}.store[RESOURCE_ENERGY])
    }

    fn store_capacity(&self) -> u32 {
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

/// Used to specify which structures can use their stored energy for spawning creeps.
///
/// # Contract
///
/// The reference returned from `AsRef<Reference>::as_ref` must be able to be used
/// by a spawner to create a new creep.
pub unsafe trait HasEnergyForSpawn: CanStoreEnergy {}

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
pub unsafe trait Attackable: RoomObjectProperties {
    fn hits(&self) -> u32 {
        js_unwrap!{ @{self.as_ref()}.hits }
    }

    fn hits_max(&self) -> u32 {
        js_unwrap!{ @{self.as_ref()}.hitsMax }
    }
}

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

unsafe impl Attackable for Creep {}
unsafe impl Attackable for OwnedStructure {}
unsafe impl Attackable for Structure {}
unsafe impl Attackable for StructureContainer {}
unsafe impl Attackable for StructureExtension {}
unsafe impl Attackable for StructureExtractor {}
unsafe impl Attackable for StructureKeeperLair {}
unsafe impl Attackable for StructureLab {}
unsafe impl Attackable for StructureLink {}
unsafe impl Attackable for StructureNuker {}
unsafe impl Attackable for StructureObserver {}
unsafe impl Attackable for StructurePowerBank {}
unsafe impl Attackable for StructurePowerSpawn {}
unsafe impl Attackable for StructurePortal {}
unsafe impl Attackable for StructureRampart {}
unsafe impl Attackable for StructureRoad {}
unsafe impl Attackable for StructureSpawn {}
unsafe impl Attackable for StructureStorage {}
unsafe impl Attackable for StructureTerminal {}
unsafe impl Attackable for StructureTower {}
unsafe impl Attackable for StructureWall {}

unsafe impl RoomObjectProperties for ConstructionSite {}
unsafe impl RoomObjectProperties for Creep {}
unsafe impl RoomObjectProperties for Flag {}
unsafe impl RoomObjectProperties for Mineral {}
unsafe impl RoomObjectProperties for Nuke {}
unsafe impl RoomObjectProperties for OwnedStructure {}
unsafe impl RoomObjectProperties for Resource {}
unsafe impl RoomObjectProperties for RoomObject {}
unsafe impl RoomObjectProperties for Source {}
unsafe impl RoomObjectProperties for StructureContainer {}
unsafe impl RoomObjectProperties for StructureController {}
unsafe impl RoomObjectProperties for StructureExtension {}
unsafe impl RoomObjectProperties for StructureExtractor {}
unsafe impl RoomObjectProperties for StructureKeeperLair {}
unsafe impl RoomObjectProperties for StructureLab {}
unsafe impl RoomObjectProperties for StructureLink {}
unsafe impl RoomObjectProperties for StructureNuker {}
unsafe impl RoomObjectProperties for StructureObserver {}
unsafe impl RoomObjectProperties for StructurePowerBank {}
unsafe impl RoomObjectProperties for StructurePowerSpawn {}
unsafe impl RoomObjectProperties for StructurePortal {}
unsafe impl RoomObjectProperties for StructureRampart {}
unsafe impl RoomObjectProperties for StructureRoad {}
unsafe impl RoomObjectProperties for StructureSpawn {}
unsafe impl RoomObjectProperties for StructureStorage {}
unsafe impl RoomObjectProperties for StructureTerminal {}
unsafe impl RoomObjectProperties for StructureTower {}
unsafe impl RoomObjectProperties for StructureWall {}
unsafe impl RoomObjectProperties for Structure {}
unsafe impl RoomObjectProperties for Tombstone {}

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
    fn store_capacity(&self) -> u32 {
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

unsafe impl HasEnergyForSpawn for StructureExtension {}
unsafe impl HasEnergyForSpawn for StructureSpawn {}

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
