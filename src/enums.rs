use enum_dispatch::enum_dispatch;

use crate::{
    objects::*,
};

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(IsStructure)]
pub enum StructureObject {
    StructureSpawn,
    // StructureExtension,
    // StructureRoad,
    // StructureWall,
    // StructureRampart,
    // StructureKeeperLair,
    // StructurePortal,
    // StructureController,
    // StructureLink,
    StructureStorage,
    // StructureTower,
    // StructureObserver,
    // StructurePowerBank,
    // StructurePowerSpawn,
    // StructureExtractor,
    // StructureLab,
    StructureTerminal,
    // StructureContainer,
    // StructureNuker,
    // StructureFactory,
    // StructureInvaderCore,
}

#[enum_dispatch(HasStore)]
pub enum StoreObject {
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
}


#[enum_dispatch(Attackable)]
pub enum AttackableObject {
    StructureStorage,
    StructureSpawn,
    StructureTerminal,
}

impl From<AttackableObject> for RoomObject {
    fn from(attackable: AttackableObject) -> Self {
        use AttackableObject::*;

        match attackable {
            StructureStorage(o) => RoomObject::from(o),
            StructureSpawn(o) => RoomObject::from(o),
            StructureTerminal(o) => RoomObject::from(o),
        }
    }
}


impl AsRef<RoomObject> for AttackableObject {
    fn as_ref(&self) -> &RoomObject {
        use AttackableObject::*;

        match self {
            StructureStorage(o) => o.as_ref(),
            StructureSpawn(o) => o.as_ref(),
            StructureTerminal(o) => o.as_ref(),
        }
    }
}
