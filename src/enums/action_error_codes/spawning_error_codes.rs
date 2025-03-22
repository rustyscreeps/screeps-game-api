use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by [Spawning::cancel](crate::Spawning::cancel).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureSpawn.Spawning.cancel).
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

impl From<CancelErrorCode> for ErrorCode {
    fn from(value: CancelErrorCode) -> Self {
        // Safety: CancelErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CancelErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Spawning::set_directions](crate::Spawning::set_directions).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureSpawn.Spawning.setDirections).
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

impl From<SetDirectionsErrorCode> for ErrorCode {
    fn from(value: SetDirectionsErrorCode) -> Self {
        // Safety: SetDirectionsErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SetDirectionsErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
