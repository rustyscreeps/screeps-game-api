use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [StructureController::activate_safe_mode](crate::StructureController::activate_safe_mode).
///
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureController.activateSafeMode).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L211)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ActivateSafeModeErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    Tired = -11,
}

impl FromReturnCode for ActivateSafeModeErrorCode {
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
            -1 => Some(Err(ActivateSafeModeErrorCode::NotOwner)),
            -4 => Some(Err(ActivateSafeModeErrorCode::Busy)),
            -6 => Some(Err(ActivateSafeModeErrorCode::NotEnoughResources)),
            -11 => Some(Err(ActivateSafeModeErrorCode::Tired)),
            _ => None,
        }
    }
}

impl fmt::Display for ActivateSafeModeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ActivateSafeModeErrorCode::NotOwner => "you are not the owner of this controller",
            ActivateSafeModeErrorCode::Busy => "there is another room in safe mode already",
            ActivateSafeModeErrorCode::NotEnoughResources => "there is no safe mode activations available",
            ActivateSafeModeErrorCode::Tired => "the previous safe mode is still cooling down, or the controller is upgradeblocked, or the controller is downgraded for 50% plus 5000 ticks or more",
        };

        write!(f, "{}", msg)
    }
}

impl Error for ActivateSafeModeErrorCode {}

impl From<ActivateSafeModeErrorCode> for ErrorCode {
    fn from(value: ActivateSafeModeErrorCode) -> Self {
        // Safety: ActivateSafeModeErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: ActivateSafeModeErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}

/// Error codes used by
/// [StructureController::unclaim](crate::StructureController::unclaim).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureController.unclaim).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L201)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum UnclaimErrorCode {
    NotOwner = -1,
}

impl FromReturnCode for UnclaimErrorCode {
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
            -1 => Some(Err(UnclaimErrorCode::NotOwner)),
            _ => None,
        }
    }
}

impl fmt::Display for UnclaimErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            UnclaimErrorCode::NotOwner => "you are not the owner of this controller",
        };

        write!(f, "{}", msg)
    }
}

impl Error for UnclaimErrorCode {}

impl From<UnclaimErrorCode> for ErrorCode {
    fn from(value: UnclaimErrorCode) -> Self {
        // Safety: UnclaimErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: UnclaimErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}
