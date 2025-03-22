use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [StructureSpawn::spawn_creep](crate::StructureSpawn::spawn_creep).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureSpawn.spawnCreep).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1063)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SpawnCreepErrorCode {
    NotOwner = -1,
    NameExists = -3,
    Busy = -4,
    NotEnoughEnergy = -6,
    InvalidArgs = -10,
    RclNotEnough = -14,
}

impl FromReturnCode for SpawnCreepErrorCode {
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
            -1 => Some(Err(SpawnCreepErrorCode::NotOwner)),
            -3 => Some(Err(SpawnCreepErrorCode::NameExists)),
            -4 => Some(Err(SpawnCreepErrorCode::Busy)),
            -6 => Some(Err(SpawnCreepErrorCode::NotEnoughEnergy)),
            -10 => Some(Err(SpawnCreepErrorCode::InvalidArgs)),
            -14 => Some(Err(SpawnCreepErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for SpawnCreepErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SpawnCreepErrorCode::NotOwner => "you are not the owner of this spawn",
            SpawnCreepErrorCode::NameExists => "there is a creep with the same name already",
            SpawnCreepErrorCode::Busy => "the spawn is already in process of spawning another creep",
            SpawnCreepErrorCode::NotEnoughEnergy => "the spawn and its extensions contain not enough energy to create a creep with the given body",
            SpawnCreepErrorCode::InvalidArgs => "body is not properly described or name was not provided",
            SpawnCreepErrorCode::RclNotEnough => "your room controller level is insufficient to use this spawn",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SpawnCreepErrorCode {}

impl From<SpawnCreepErrorCode> for ErrorCode {
    fn from(value: SpawnCreepErrorCode) -> Self {
        // Safety: SpawnCreepErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SpawnCreepErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [StructureSpawn::recycle_creep](crate::StructureSpawn::recycle_creep).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureSpawn.recycleCreep).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1269)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RecycleCreepErrorCode {
    NotOwner = -1,
    InvalidTarget = -7,
    NotInRange = -9,
    RclNotEnough = -14,
}

impl FromReturnCode for RecycleCreepErrorCode {
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
            -1 => Some(Err(RecycleCreepErrorCode::NotOwner)),
            -7 => Some(Err(RecycleCreepErrorCode::InvalidTarget)),
            -9 => Some(Err(RecycleCreepErrorCode::NotInRange)),
            -14 => Some(Err(RecycleCreepErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for RecycleCreepErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RecycleCreepErrorCode::NotOwner => {
                "you are not the owner of this spawn or the target creep"
            }
            RecycleCreepErrorCode::InvalidTarget => "the specified target object is not a creep",
            RecycleCreepErrorCode::NotInRange => "the target creep is too far away",
            RecycleCreepErrorCode::RclNotEnough => {
                "your room controller level is insufficient to use this spawn"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RecycleCreepErrorCode {}

impl From<RecycleCreepErrorCode> for ErrorCode {
    fn from(value: RecycleCreepErrorCode) -> Self {
        // Safety: RecycleCreepErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RecycleCreepErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [StructureSpawn::renew_creep](crate::StructureSpawn::renew_creep).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureSpawn.renewCreep).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1237)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RenewCreepErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughEnergy = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    RclNotEnough = -14,
}

impl FromReturnCode for RenewCreepErrorCode {
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
            -1 => Some(Err(RenewCreepErrorCode::NotOwner)),
            -4 => Some(Err(RenewCreepErrorCode::Busy)),
            -6 => Some(Err(RenewCreepErrorCode::NotEnoughEnergy)),
            -7 => Some(Err(RenewCreepErrorCode::InvalidTarget)),
            -8 => Some(Err(RenewCreepErrorCode::Full)),
            -9 => Some(Err(RenewCreepErrorCode::NotInRange)),
            -14 => Some(Err(RenewCreepErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for RenewCreepErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RenewCreepErrorCode::NotOwner => "you are not the owner of the spawn, or the creep",
            RenewCreepErrorCode::Busy => "the spawn is spawning another creep",
            RenewCreepErrorCode::NotEnoughEnergy => "the spawn does not have enough energy",
            RenewCreepErrorCode::InvalidTarget => {
                "the specified target object is not a creep, or the creep has claim body part"
            }
            RenewCreepErrorCode::Full => "the target creep's time to live timer is full",
            RenewCreepErrorCode::NotInRange => "the target creep is too far away",
            RenewCreepErrorCode::RclNotEnough => {
                "your room controller level is insufficient to use this spawn"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RenewCreepErrorCode {}

impl From<RenewCreepErrorCode> for ErrorCode {
    fn from(value: RenewCreepErrorCode) -> Self {
        // Safety: RenewCreepErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RenewCreepErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
