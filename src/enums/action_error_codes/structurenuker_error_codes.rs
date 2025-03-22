use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [StructureNuker::launch_nuke](crate::StructureNuker::launch_nuke).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureNuker.launchNuke).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1356)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum LaunchNukeErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for LaunchNukeErrorCode {
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
            -1 => Some(Err(LaunchNukeErrorCode::NotOwner)),
            -6 => Some(Err(LaunchNukeErrorCode::NotEnoughResources)),
            -7 => Some(Err(LaunchNukeErrorCode::InvalidTarget)),
            -9 => Some(Err(LaunchNukeErrorCode::NotInRange)),
            -10 => Some(Err(LaunchNukeErrorCode::InvalidArgs)),
            -11 => Some(Err(LaunchNukeErrorCode::Tired)),
            -14 => Some(Err(LaunchNukeErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for LaunchNukeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            LaunchNukeErrorCode::NotOwner => "you are not the owner of this structure",
            LaunchNukeErrorCode::NotEnoughResources => {
                "the structure does not have enough energy and/or ghodium"
            }
            LaunchNukeErrorCode::InvalidTarget => {
                "the nuke can't be launched to the specified roomposition (see start areas)"
            }
            LaunchNukeErrorCode::NotInRange => "the target room is out of range",
            LaunchNukeErrorCode::InvalidArgs => "the target is not a valid roomposition",
            LaunchNukeErrorCode::Tired => "this structure is still cooling down",
            LaunchNukeErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for LaunchNukeErrorCode {}

impl From<LaunchNukeErrorCode> for ErrorCode {
    fn from(value: LaunchNukeErrorCode) -> Self {
        // Safety: LaunchNukeErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: LaunchNukeErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
