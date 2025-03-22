use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [StructureLink::transfer_energy](crate::StructureLink::transfer_energy).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureLink.transferEnergy).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L488)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum TransferEnergyErrorCode {
    NotOwner = -1,
    NotEnoughEnergy = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for TransferEnergyErrorCode {
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
            -1 => Some(Err(TransferEnergyErrorCode::NotOwner)),
            -6 => Some(Err(TransferEnergyErrorCode::NotEnoughEnergy)),
            -7 => Some(Err(TransferEnergyErrorCode::InvalidTarget)),
            -8 => Some(Err(TransferEnergyErrorCode::Full)),
            -9 => Some(Err(TransferEnergyErrorCode::NotInRange)),
            -10 => Some(Err(TransferEnergyErrorCode::InvalidArgs)),
            -11 => Some(Err(TransferEnergyErrorCode::Tired)),
            -14 => Some(Err(TransferEnergyErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for TransferEnergyErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            TransferEnergyErrorCode::NotOwner => "you are not the owner of this link",
            TransferEnergyErrorCode::NotEnoughEnergy => {
                "the structure does not have the given amount of energy"
            }
            TransferEnergyErrorCode::InvalidTarget => {
                "the target is not a valid structurelink object"
            }
            TransferEnergyErrorCode::Full => "the target cannot receive any more energy",
            TransferEnergyErrorCode::NotInRange => "the target is too far away",
            TransferEnergyErrorCode::InvalidArgs => "the energy amount is incorrect",
            TransferEnergyErrorCode::Tired => "the link is still cooling down",
            TransferEnergyErrorCode::RclNotEnough => {
                "room controller level insufficient to use this link"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for TransferEnergyErrorCode {}

impl From<TransferEnergyErrorCode> for ErrorCode {
    fn from(value: TransferEnergyErrorCode) -> Self {
        // Safety: TransferEnergyErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: TransferEnergyErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}
