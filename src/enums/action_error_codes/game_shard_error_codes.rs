use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [game::cpu::unlock](crate::game::shard::activate_access).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Game.shard.activateAccess).
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ActivateAccessErrorCode {
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
}

impl FromReturnCode for ActivateAccessErrorCode {
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
            -6 => Some(Err(ActivateAccessErrorCode::NotEnoughResources)),
            -7 => Some(Err(ActivateAccessErrorCode::InvalidTarget)),
            -8 => Some(Err(ActivateAccessErrorCode::Full)),
            _ => None,
        }
    }
}

impl fmt::Display for ActivateAccessErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ActivateAccessErrorCode::NotEnoughResources => {
                "your account does not have enough accessKey resource"
            }
            ActivateAccessErrorCode::InvalidTarget => "this shard is not restricted",
            ActivateAccessErrorCode::Full => "your access is unlimited",
        };

        write!(f, "{}", msg)
    }
}

impl Error for ActivateAccessErrorCode {}

impl From<ActivateAccessErrorCode> for ErrorCode {
    fn from(value: ActivateAccessErrorCode) -> Self {
        // Safety: ActivateAccessErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: ActivateAccessErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
