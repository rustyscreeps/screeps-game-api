use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by [Structure::destroy](crate::Structure::destroy).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Structure.destroy).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L72)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum DestroyErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
}

impl FromReturnCode for DestroyErrorCode {
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
            -1 => Some(Err(DestroyErrorCode::NotOwner)),
            -4 => Some(Err(DestroyErrorCode::Busy)),
            -7 => Some(Err(DestroyErrorCode::InvalidTarget)),
            _ => None,
        }
    }
}

impl fmt::Display for DestroyErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            DestroyErrorCode::NotOwner => {
                "you are not the owner of this structure, and it's not in your room"
            }
            DestroyErrorCode::Busy => "hostile creeps are in the room",
            DestroyErrorCode::InvalidTarget => "room property invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for DestroyErrorCode {}

impl From<DestroyErrorCode> for ErrorCode {
    fn from(value: DestroyErrorCode) -> Self {
        // Safety: DestroyErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: DestroyErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}

/// Error codes used by
/// [Structure::notify_when_attacked](crate::Structure::notify_when_attacked).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Structure.notifyWhenAttacked).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L89)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum StructureNotifyWhenAttackedErrorCode {
    NotOwner = -1,
    InvalidTarget = -7,
    InvalidArgs = -10,
}

impl FromReturnCode for StructureNotifyWhenAttackedErrorCode {
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
            -1 => Some(Err(StructureNotifyWhenAttackedErrorCode::NotOwner)),
            -7 => Some(Err(StructureNotifyWhenAttackedErrorCode::InvalidTarget)),
            -10 => Some(Err(StructureNotifyWhenAttackedErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for StructureNotifyWhenAttackedErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            StructureNotifyWhenAttackedErrorCode::NotOwner => {
                "you are not the owner of this structure"
            }
            StructureNotifyWhenAttackedErrorCode::InvalidTarget => "room property invalid",
            StructureNotifyWhenAttackedErrorCode::InvalidArgs => {
                "enable argument is not a boolean value"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for StructureNotifyWhenAttackedErrorCode {}

impl From<StructureNotifyWhenAttackedErrorCode> for ErrorCode {
    fn from(value: StructureNotifyWhenAttackedErrorCode) -> Self {
        // Safety: StructureNotifyWhenAttackedErrorCode is repr(i8), so we can cast it
        // to get the discriminant value, which will match the raw return code value
        // that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: StructureNotifyWhenAttackedErrorCode discriminants are always error
        // code values, and thus the Result returned here will always be an `Err`
        // variant, so we can always extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}
