use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [StructureRampart::set_public](crate::StructureRampart::set_public).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureRampart.setPublic).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L651)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SetPublicErrorCode {
    NotOwner = -1,
}

impl FromReturnCode for SetPublicErrorCode {
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
            -1 => Some(Err(SetPublicErrorCode::NotOwner)),
            _ => None,
        }
    }
}

impl fmt::Display for SetPublicErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SetPublicErrorCode::NotOwner => "you are not the owner of this structure",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SetPublicErrorCode {}

impl From<SetPublicErrorCode> for ErrorCode {
    fn from(value: SetPublicErrorCode) -> Self {
        // Safety: SetPublicErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SetPublicErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}
