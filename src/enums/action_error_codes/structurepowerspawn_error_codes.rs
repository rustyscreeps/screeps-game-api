use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by
/// [StructurePowerSpawn::process_power](crate::StructurePowerSpawn::process_power).
///
///
/// Screeps API Docs: [StructurePowerSpawn.processPower](https://docs.screeps.com/api/#StructurePowerSpawn.processPower).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L613)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ProcessPowerErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    RclNotEnough = -14,
}

impl FromReturnCode for ProcessPowerErrorCode {
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
            -1 => Some(Err(ProcessPowerErrorCode::NotOwner)),
            -6 => Some(Err(ProcessPowerErrorCode::NotEnoughResources)),
            -14 => Some(Err(ProcessPowerErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for ProcessPowerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ProcessPowerErrorCode::NotOwner => "you are not the owner of this structure",
            ProcessPowerErrorCode::NotEnoughResources => {
                "the structure does not have enough energy or power resource units"
            }
            ProcessPowerErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for ProcessPowerErrorCode {}
