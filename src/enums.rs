//! Enums using [`enum_dispatch`] for generic wrappers around object types with
//! shared traits.
//!
//! [`enum_dispatch`]: enum_dispatch::enum_dispatch
use enum_dispatch::enum_dispatch;
use wasm_bindgen::{JsCast, JsValue};

use crate::{objects::*, prelude::*, ResourceType, RESOURCES_ALL};

#[enum_dispatch(Attackable)]
pub enum AttackableObject {
    Creep,
    PowerCreep,
    StructureContainer,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
}

impl From<AttackableObject> for RoomObject {
    fn from(attackable: AttackableObject) -> Self {
        use AttackableObject::*;

        match attackable {
            Creep(o) => RoomObject::from(o),
            PowerCreep(o) => RoomObject::from(o),
            StructureContainer(o) => RoomObject::from(o),
            StructureExtension(o) => RoomObject::from(o),
            StructureExtractor(o) => RoomObject::from(o),
            StructureFactory(o) => RoomObject::from(o),
            StructureInvaderCore(o) => RoomObject::from(o),
            StructureLab(o) => RoomObject::from(o),
            StructureLink(o) => RoomObject::from(o),
            StructureNuker(o) => RoomObject::from(o),
            StructureObserver(o) => RoomObject::from(o),
            StructurePowerBank(o) => RoomObject::from(o),
            StructurePowerSpawn(o) => RoomObject::from(o),
            StructureRampart(o) => RoomObject::from(o),
            StructureRoad(o) => RoomObject::from(o),
            StructureSpawn(o) => RoomObject::from(o),
            StructureStorage(o) => RoomObject::from(o),
            StructureTerminal(o) => RoomObject::from(o),
            StructureTower(o) => RoomObject::from(o),
            StructureWall(o) => RoomObject::from(o),
        }
    }
}

impl AsRef<RoomObject> for AttackableObject {
    fn as_ref(&self) -> &RoomObject {
        use AttackableObject::*;

        match self {
            Creep(o) => o.as_ref(),
            PowerCreep(o) => o.as_ref(),
            StructureContainer(o) => o.as_ref(),
            StructureExtension(o) => o.as_ref(),
            StructureExtractor(o) => o.as_ref(),
            StructureFactory(o) => o.as_ref(),
            StructureInvaderCore(o) => o.as_ref(),
            StructureLab(o) => o.as_ref(),
            StructureLink(o) => o.as_ref(),
            StructureNuker(o) => o.as_ref(),
            StructureObserver(o) => o.as_ref(),
            StructurePowerBank(o) => o.as_ref(),
            StructurePowerSpawn(o) => o.as_ref(),
            StructureRampart(o) => o.as_ref(),
            StructureRoad(o) => o.as_ref(),
            StructureSpawn(o) => o.as_ref(),
            StructureStorage(o) => o.as_ref(),
            StructureTerminal(o) => o.as_ref(),
            StructureTower(o) => o.as_ref(),
            StructureWall(o) => o.as_ref(),
        }
    }
}

#[enum_dispatch(CanDecay)]
pub enum DecayingObject {
    Deposit,
    Ruin,
    #[cfg(feature = "seasonal-season-1")]
    ScoreContainer,
    StructureContainer,
    StructurePortal,
    StructurePowerBank,
    StructureRampart,
    StructureRoad,
    #[cfg(feature = "seasonal-season-2")]
    SymbolContainer,
    Tombstone,
}

#[enum_dispatch(HasCooldown)]
pub enum CooldownObject {
    Deposit,
    StructureExtractor,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureTerminal,
}

#[enum_dispatch(HasPosition)]
pub enum ObjectWithPosition {
    ConstructionSite,
    Creep,
    Deposit,
    Flag,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    RoomPosition,
    Ruin,
    #[cfg(feature = "seasonal-season-1")]
    ScoreCollector,
    #[cfg(feature = "seasonal-season-1")]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    #[cfg(feature = "seasonal-season-2")]
    SymbolContainer,
    #[cfg(feature = "seasonal-season-2")]
    SymbolDecoder,
    Tombstone,
}

#[enum_dispatch(HasStore)]
pub enum StoreObject {
    Creep,
    PowerCreep,
    #[cfg(feature = "seasonal-season-5")]
    Reactor,
    Ruin,
    #[cfg(feature = "seasonal-season-1")]
    ScoreCollector,
    #[cfg(feature = "seasonal-season-1")]
    ScoreContainer,
    StructureContainer,
    StructureExtension,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructurePowerSpawn,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    #[cfg(feature = "seasonal-season-2")]
    SymbolContainer,
    Tombstone,
}

impl StoreObject {
    /// All possible resources that this store may hold.
    ///
    /// Note: [`StructureLab`] is slightly odd in that it can hold any possible
    /// resource (including non-lab resources like power), but only one
    /// non-energy resource at a time.
    ///
    /// # Example
    /// Assuming that `store_object` is a [`StoreObject::StructureExtension`]:
    /// ```no_run
    /// # use screeps::{ResourceType, StoreObject};
    /// # let store_object: StoreObject = todo!();
    /// assert_eq!(store_object.resource_types(), &[ResourceType::Energy]);
    /// ```
    /// To access this property from a [`StructureObject`], use the following
    /// conversion:
    /// ```no_run
    /// # use screeps::{ResourceType, StoreObject, StoreObjectConversionError, StructureObject};
    /// # let structure_object: StructureObject = todo!();
    /// let ty: Result<&[ResourceType], StoreObjectConversionError> =
    ///     StoreObject::try_from(structure_object).map(|store_obj| store_obj.resource_types());
    /// ```
    ///
    /// Data collected on 2024-02-29 from <https://github.com/screeps/engine/tree/97c9d12385fed686655c13b09f5f2457dd83a2bf>
    pub const fn resource_types(&self) -> &'static [ResourceType] {
        match self {
            StoreObject::Creep(_) => RESOURCES_ALL,
            StoreObject::PowerCreep(_) => RESOURCES_ALL,
            #[cfg(feature = "seasonal-season-5")]
            StoreObject::Reactor(_) => &[ResourceType::Thorium],
            StoreObject::Ruin(_) => RESOURCES_ALL,
            #[cfg(feature = "seasonal-season-1")]
            StoreObject::ScoreCollector(_) => &[ResourceType::Score],
            #[cfg(feature = "seasonal-season-1")]
            StoreObject::ScoreContainer(_) => &[ResourceType::Score],
            StoreObject::StructureContainer(_) => RESOURCES_ALL,
            StoreObject::StructureExtension(_) => &[ResourceType::Energy],
            StoreObject::StructureFactory(_) => RESOURCES_ALL,
            StoreObject::StructureLab(_) => RESOURCES_ALL,
            StoreObject::StructureLink(_) => &[ResourceType::Energy],
            StoreObject::StructureNuker(_) => &[ResourceType::Energy, ResourceType::Ghodium],
            StoreObject::StructurePowerSpawn(_) => &[ResourceType::Energy, ResourceType::Power],
            StoreObject::StructureSpawn(_) => &[ResourceType::Energy],
            StoreObject::StructureStorage(_) => RESOURCES_ALL,
            StoreObject::StructureTerminal(_) => RESOURCES_ALL,
            StoreObject::StructureTower(_) => &[ResourceType::Energy],
            #[cfg(feature = "seasonal-season-2")]
            StoreObject::SymbolContainer(_) => &crate::constants::seasonal::season_2::SYMBOLS,
            StoreObject::Tombstone(_) => RESOURCES_ALL,
        }
    }
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(OwnedStructureProperties)]
pub enum OwnedStructureObject {
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerSpawn,
    StructureRampart,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
}

// todo TryFrom<Structure> for OwnedStructureObject

/// Any enum representing any game object that inherits the [`RoomObject`] type.
#[enum_dispatch(RoomObjectProperties)]
pub enum TypedRoomObject {
    ConstructionSite,
    Creep,
    Deposit,
    Flag,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    Ruin,
    #[cfg(feature = "seasonal-season-1")]
    ScoreCollector,
    #[cfg(feature = "seasonal-season-1")]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    #[cfg(feature = "seasonal-season-2")]
    SymbolContainer,
    #[cfg(feature = "seasonal-season-2")]
    SymbolDecoder,
    Tombstone,
}

#[enum_dispatch(SharedCreepProperties)]
pub enum MovableObject {
    Creep,
    PowerCreep,
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(StructureProperties, HasPosition)]
#[derive(Clone, Debug)]
pub enum StructureObject {
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
}

#[enum_dispatch(Transferable)]
pub enum TransferableObject {
    StructureExtension,
    Creep,
    StructureContainer,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureSpawn,
    StructureStorage,
    StructureTower,
    StructurePowerSpawn,
    StructureTerminal,
    PowerCreep,
}

impl AsRef<RoomObject> for TransferableObject {
    fn as_ref(&self) -> &RoomObject {
        use TransferableObject::*;

        match self {
            StructureExtension(o) => o.as_ref(),
            Creep(o) => o.as_ref(),
            StructureContainer(o) => o.as_ref(),
            StructureFactory(o) => o.as_ref(),
            StructureLab(o) => o.as_ref(),
            StructureLink(o) => o.as_ref(),
            StructureNuker(o) => o.as_ref(),
            StructureSpawn(o) => o.as_ref(),
            StructureStorage(o) => o.as_ref(),
            StructureTower(o) => o.as_ref(),
            StructurePowerSpawn(o) => o.as_ref(),
            StructureTerminal(o) => o.as_ref(),
            PowerCreep(o) => o.as_ref(),
        }
    }
}

impl From<JsValue> for StructureObject {
    fn from(reference: JsValue) -> Self {
        let structure: Structure = reference.unchecked_into();

        structure.into()
    }
}

impl JsCollectionFromValue for StructureObject {
    fn from_value(val: JsValue) -> Self {
        Self::from(val)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OwnedStructureConversionError {
    NotOwnable,
}

impl TryFrom<StructureObject> for OwnedStructureObject {
    type Error = OwnedStructureConversionError;

    fn try_from(structure: StructureObject) -> Result<Self, Self::Error> {
        match structure {
            StructureObject::StructureController(val) => {
                Ok(OwnedStructureObject::StructureController(val))
            }
            StructureObject::StructureExtension(val) => {
                Ok(OwnedStructureObject::StructureExtension(val))
            }
            StructureObject::StructureExtractor(val) => {
                Ok(OwnedStructureObject::StructureExtractor(val))
            }
            StructureObject::StructureFactory(val) => {
                Ok(OwnedStructureObject::StructureFactory(val))
            }
            StructureObject::StructureInvaderCore(val) => {
                Ok(OwnedStructureObject::StructureInvaderCore(val))
            }
            StructureObject::StructureKeeperLair(val) => {
                Ok(OwnedStructureObject::StructureKeeperLair(val))
            }
            StructureObject::StructureLab(val) => Ok(OwnedStructureObject::StructureLab(val)),
            StructureObject::StructureLink(val) => Ok(OwnedStructureObject::StructureLink(val)),
            StructureObject::StructureNuker(val) => Ok(OwnedStructureObject::StructureNuker(val)),
            StructureObject::StructureObserver(val) => {
                Ok(OwnedStructureObject::StructureObserver(val))
            }
            StructureObject::StructurePowerSpawn(val) => {
                Ok(OwnedStructureObject::StructurePowerSpawn(val))
            }
            StructureObject::StructureRampart(val) => {
                Ok(OwnedStructureObject::StructureRampart(val))
            }
            StructureObject::StructureSpawn(val) => Ok(OwnedStructureObject::StructureSpawn(val)),
            StructureObject::StructureStorage(val) => {
                Ok(OwnedStructureObject::StructureStorage(val))
            }
            StructureObject::StructureTerminal(val) => {
                Ok(OwnedStructureObject::StructureTerminal(val))
            }
            StructureObject::StructureTower(val) => Ok(OwnedStructureObject::StructureTower(val)),
            StructureObject::StructureContainer(_)
            | StructureObject::StructureRoad(_)
            | StructureObject::StructurePortal(_)
            | StructureObject::StructurePowerBank(_)
            | StructureObject::StructureWall(_) => Err(OwnedStructureConversionError::NotOwnable),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TransferableObjectConversionError {
    NotTransferable,
}

impl TryFrom<StructureObject> for TransferableObject {
    type Error = TransferableObjectConversionError;

    fn try_from(structure: StructureObject) -> Result<Self, Self::Error> {
        match structure {
            StructureObject::StructureExtension(val) => Ok(Self::StructureExtension(val)),
            StructureObject::StructureContainer(val) => Ok(Self::StructureContainer(val)),
            StructureObject::StructureFactory(val) => Ok(Self::StructureFactory(val)),
            StructureObject::StructureLab(val) => Ok(Self::StructureLab(val)),
            StructureObject::StructureLink(val) => Ok(Self::StructureLink(val)),
            StructureObject::StructureNuker(val) => Ok(Self::StructureNuker(val)),
            StructureObject::StructureSpawn(val) => Ok(Self::StructureSpawn(val)),
            StructureObject::StructureStorage(val) => Ok(Self::StructureStorage(val)),
            StructureObject::StructureTower(val) => Ok(Self::StructureTower(val)),
            StructureObject::StructurePowerSpawn(val) => Ok(Self::StructurePowerSpawn(val)),
            StructureObject::StructureTerminal(val) => Ok(Self::StructureTerminal(val)),

            StructureObject::StructureController(_)
            | StructureObject::StructureExtractor(_)
            | StructureObject::StructureInvaderCore(_)
            | StructureObject::StructureKeeperLair(_)
            | StructureObject::StructureObserver(_)
            | StructureObject::StructurePortal(_)
            | StructureObject::StructurePowerBank(_)
            | StructureObject::StructureRampart(_)
            | StructureObject::StructureRoad(_)
            | StructureObject::StructureWall(_) => {
                Err(TransferableObjectConversionError::NotTransferable)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StoreObjectConversionError {
    DoesntHaveStore,
}

impl TryFrom<StructureObject> for StoreObject {
    type Error = StoreObjectConversionError;

    fn try_from(structure: StructureObject) -> Result<Self, Self::Error> {
        match structure {
            StructureObject::StructureContainer(val) => Ok(Self::StructureContainer(val)),
            StructureObject::StructureExtension(val) => Ok(Self::StructureExtension(val)),
            StructureObject::StructureFactory(val) => Ok(Self::StructureFactory(val)),
            StructureObject::StructureLab(val) => Ok(Self::StructureLab(val)),
            StructureObject::StructureLink(val) => Ok(Self::StructureLink(val)),
            StructureObject::StructureNuker(val) => Ok(Self::StructureNuker(val)),
            StructureObject::StructurePowerSpawn(val) => Ok(Self::StructurePowerSpawn(val)),
            StructureObject::StructureSpawn(val) => Ok(Self::StructureSpawn(val)),
            StructureObject::StructureStorage(val) => Ok(Self::StructureStorage(val)),
            StructureObject::StructureTerminal(val) => Ok(Self::StructureTerminal(val)),
            StructureObject::StructureTower(val) => Ok(Self::StructureTower(val)),

            StructureObject::StructureController(_)
            | StructureObject::StructureExtractor(_)
            | StructureObject::StructureInvaderCore(_)
            | StructureObject::StructureKeeperLair(_)
            | StructureObject::StructureObserver(_)
            | StructureObject::StructurePortal(_)
            | StructureObject::StructurePowerBank(_)
            | StructureObject::StructureRampart(_)
            | StructureObject::StructureRoad(_)
            | StructureObject::StructureWall(_) => Err(StoreObjectConversionError::DoesntHaveStore),
        }
    }
}

impl From<Structure> for StructureObject {
    fn from(structure: Structure) -> Self {
        use crate::constants::StructureType::*;

        match structure.structure_type() {
            Container => Self::StructureContainer(structure.unchecked_into()),
            Controller => Self::StructureController(structure.unchecked_into()),
            Extension => Self::StructureExtension(structure.unchecked_into()),
            Extractor => Self::StructureExtractor(structure.unchecked_into()),
            Factory => Self::StructureFactory(structure.unchecked_into()),
            InvaderCore => Self::StructureInvaderCore(structure.unchecked_into()),
            KeeperLair => Self::StructureKeeperLair(structure.unchecked_into()),
            Lab => Self::StructureLab(structure.unchecked_into()),
            Link => Self::StructureLink(structure.unchecked_into()),
            Nuker => Self::StructureNuker(structure.unchecked_into()),
            Observer => Self::StructureObserver(structure.unchecked_into()),
            Portal => Self::StructurePortal(structure.unchecked_into()),
            PowerBank => Self::StructurePowerBank(structure.unchecked_into()),
            PowerSpawn => Self::StructurePowerSpawn(structure.unchecked_into()),
            Rampart => Self::StructureRampart(structure.unchecked_into()),
            Road => Self::StructureRoad(structure.unchecked_into()),
            Spawn => Self::StructureSpawn(structure.unchecked_into()),
            Storage => Self::StructureStorage(structure.unchecked_into()),
            Terminal => Self::StructureTerminal(structure.unchecked_into()),
            Tower => Self::StructureTower(structure.unchecked_into()),
            Wall => Self::StructureWall(structure.unchecked_into()),
            _ => panic!("unknown structure type for conversion into enum"),
        }
    }
}

impl StructureObject {
    pub fn as_structure(&self) -> &Structure {
        match self {
            Self::StructureSpawn(s) => s.as_ref(),
            Self::StructureExtension(s) => s.as_ref(),
            Self::StructureRoad(s) => s.as_ref(),
            Self::StructureWall(s) => s.as_ref(),
            Self::StructureRampart(s) => s.as_ref(),
            Self::StructureKeeperLair(s) => s.as_ref(),
            Self::StructurePortal(s) => s.as_ref(),
            Self::StructureController(s) => s.as_ref(),
            Self::StructureLink(s) => s.as_ref(),
            Self::StructureStorage(s) => s.as_ref(),
            Self::StructureTower(s) => s.as_ref(),
            Self::StructureObserver(s) => s.as_ref(),
            Self::StructurePowerBank(s) => s.as_ref(),
            Self::StructurePowerSpawn(s) => s.as_ref(),
            Self::StructureExtractor(s) => s.as_ref(),
            Self::StructureLab(s) => s.as_ref(),
            Self::StructureTerminal(s) => s.as_ref(),
            Self::StructureContainer(s) => s.as_ref(),
            Self::StructureNuker(s) => s.as_ref(),
            Self::StructureFactory(s) => s.as_ref(),
            Self::StructureInvaderCore(s) => s.as_ref(),
        }
    }

    pub fn as_owned(&self) -> Option<&dyn OwnedStructureProperties> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(s) => Some(s),
            Self::StructureKeeperLair(s) => Some(s),
            Self::StructurePortal(_) => None,
            Self::StructureController(s) => Some(s),
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(s) => Some(s),
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(s) => Some(s),
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(_) => None,
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(s) => Some(s),
        }
    }

    pub fn as_has_store(&self) -> Option<&dyn HasStore> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(_) => None,
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(_) => None,
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(_) => None,
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }

    pub fn as_transferable(&self) -> Option<&dyn Transferable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(_) => None,
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(_) => None,
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(_) => None,
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }

    pub fn as_withdrawable(&self) -> Option<&dyn Withdrawable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(_) => None,
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(_) => None,
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(_) => None,
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(_) => None,
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }

    pub fn as_attackable(&self) -> Option<&dyn Attackable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(s) => Some(s),
            Self::StructureWall(s) => Some(s),
            Self::StructureRampart(s) => Some(s),
            Self::StructureKeeperLair(s) => Some(s),
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(s) => Some(s),
            Self::StructurePowerBank(s) => Some(s),
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(s) => Some(s),
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(s) => Some(s),
        }
    }

    pub fn as_repairable(&self) -> Option<&dyn Repairable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(s) => Some(s),
            Self::StructureWall(s) => Some(s),
            Self::StructureRampart(s) => Some(s),
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(s) => Some(s),
            Self::StructurePowerBank(s) => Some(s),
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(s) => Some(s),
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }

    pub fn as_dismantleable(&self) -> Option<&dyn Dismantleable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(s) => Some(s),
            Self::StructureWall(s) => Some(s),
            Self::StructureRampart(s) => Some(s),
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(s) => Some(s),
            Self::StructurePowerBank(s) => Some(s),
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(s) => Some(s),
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }
}
