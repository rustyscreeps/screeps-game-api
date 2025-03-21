use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by [StructureTower::attack](crate::StructureTower::attack).
///
/// Screeps API Docs: [StructureTower.attack](https://docs.screeps.com/api/#StructureTower.attack).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L766)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum TowerAttackErrorCode {
    NotOwner = -1,
    NotEnoughEnergy = -6,
    InvalidTarget = -7,
    RclNotEnough = -14,
}

impl FromReturnCode for TowerAttackErrorCode {
    type Error = Self;

    fn result_from_i8(val: i8) -> Result<(), Self::Error> {
        let maybe_result = Self::try_result_from_i8(val);
        #[cfg(feature = "unsafe-return-conversion")]
        unsafe {
            maybe_result.unwrap_unchecked()
        }
        #[cfg(not(feature = "unsafe-return-conversion"))]
        maybe_result.unwrap()
    }

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>> {
        match val {
            0 => Some(Ok(())),
            -1 => Some(Err(TowerAttackErrorCode::NotOwner)),
            -6 => Some(Err(TowerAttackErrorCode::NotEnoughEnergy)),
            -7 => Some(Err(TowerAttackErrorCode::InvalidTarget)),
            -14 => Some(Err(TowerAttackErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for TowerAttackErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            TowerAttackErrorCode::NotOwner => "you are not the owner of this structure",
            TowerAttackErrorCode::NotEnoughEnergy => "the tower does not have enough energy",
            TowerAttackErrorCode::InvalidTarget => "the target is not a valid attackable object",
            TowerAttackErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for TowerAttackErrorCode {}

/// Error codes used by [StructureTower::heal](crate::StructureTower::heal).
///
/// Screeps API Docs: [StructureTower.heal](https://docs.screeps.com/api/#StructureTower.heal).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L786)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum TowerHealErrorCode {
    NotOwner = -1,
    NotEnoughEnergy = -6,
    InvalidTarget = -7,
    RclNotEnough = -14,
}

impl FromReturnCode for TowerHealErrorCode {
    type Error = Self;

    fn result_from_i8(val: i8) -> Result<(), Self::Error> {
        let maybe_result = Self::try_result_from_i8(val);
        #[cfg(feature = "unsafe-return-conversion")]
        unsafe {
            maybe_result.unwrap_unchecked()
        }
        #[cfg(not(feature = "unsafe-return-conversion"))]
        maybe_result.unwrap()
    }

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>> {
        match val {
            0 => Some(Ok(())),
            -1 => Some(Err(TowerHealErrorCode::NotOwner)),
            -6 => Some(Err(TowerHealErrorCode::NotEnoughEnergy)),
            -7 => Some(Err(TowerHealErrorCode::InvalidTarget)),
            -14 => Some(Err(TowerHealErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for TowerHealErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            TowerHealErrorCode::NotOwner => "you are not the owner of this structure",
            TowerHealErrorCode::NotEnoughEnergy => "the tower does not have enough energy",
            TowerHealErrorCode::InvalidTarget => "the target is not a valid creep object",
            TowerHealErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for TowerHealErrorCode {}

/// Error codes used by [StructureTower::repair](crate::StructureTower::repair).
///
/// Screeps API Docs: [StructureTower.repair](https://docs.screeps.com/api/#StructureTower.repair).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L806)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum TowerRepairErrorCode {
    NotOwner = -1,
    NotEnoughEnergy = -6,
    InvalidTarget = -7,
    RclNotEnough = -14,
}

impl FromReturnCode for TowerRepairErrorCode {
    type Error = Self;

    fn result_from_i8(val: i8) -> Result<(), Self::Error> {
        let maybe_result = Self::try_result_from_i8(val);
        #[cfg(feature = "unsafe-return-conversion")]
        unsafe {
            maybe_result.unwrap_unchecked()
        }
        #[cfg(not(feature = "unsafe-return-conversion"))]
        maybe_result.unwrap()
    }

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>> {
        match val {
            0 => Some(Ok(())),
            -1 => Some(Err(TowerRepairErrorCode::NotOwner)),
            -6 => Some(Err(TowerRepairErrorCode::NotEnoughEnergy)),
            -7 => Some(Err(TowerRepairErrorCode::InvalidTarget)),
            -14 => Some(Err(TowerRepairErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for TowerRepairErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            TowerRepairErrorCode::NotOwner => "you are not the owner of this structure",
            TowerRepairErrorCode::NotEnoughEnergy => "the tower does not have enough energy",
            TowerRepairErrorCode::InvalidTarget => "the target is not a valid repairable object",
            TowerRepairErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for TowerRepairErrorCode {}
