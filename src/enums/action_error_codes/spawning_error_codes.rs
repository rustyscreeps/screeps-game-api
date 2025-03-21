use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by [Spawning::cancel](crate::Spawning::cancel).
///
/// Screeps API Docs: [Spawning.cancel](https://docs.screeps.com/api/#StructureSpawn.Spawning.cancel).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1328)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CancelErrorCode {
    NotOwner = -1,
}

impl FromReturnCode for CancelErrorCode {
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
            -1 => Some(Err(CancelErrorCode::NotOwner)),
            _ => None,
        }
    }
}

impl fmt::Display for CancelErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CancelErrorCode::NotOwner => "you are not the owner of this spawn",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CancelErrorCode {}

/// Error codes used by
/// [Spawning::set_directions](crate::Spawning::set_directions).
///
/// Screeps API Docs: [Spawning.setDirections](https://docs.screeps.com/api/#StructureSpawn.Spawning.setDirections).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1312)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SetDirectionsErrorCode {
    NotOwner = -1,
    InvalidArgs = -10,
}

impl FromReturnCode for SetDirectionsErrorCode {
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
            -1 => Some(Err(SetDirectionsErrorCode::NotOwner)),
            -10 => Some(Err(SetDirectionsErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for SetDirectionsErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SetDirectionsErrorCode::NotOwner => "you are not the owner of this spawn",
            SetDirectionsErrorCode::InvalidArgs => "the directions is array is invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SetDirectionsErrorCode {}
