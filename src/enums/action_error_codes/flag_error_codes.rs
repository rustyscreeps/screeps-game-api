use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by [Flag::remove](crate::Flag::remove).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Flag.remove).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/flags.js#L57)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
pub enum FlagRemoveErrorCode {}

impl FromReturnCode for FlagRemoveErrorCode {
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
            _ => None,
        }
    }
}

impl fmt::Display for FlagRemoveErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = "Zero-variant enum";

        write!(f, "{}", msg)
    }
}

impl Error for FlagRemoveErrorCode {}

/// Error codes used by [Flag::set_color](crate::Flag::set_color).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Flag.setColor).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/flags.js#L76)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SetColorErrorCode {
    InvalidArgs = -10,
}

impl FromReturnCode for SetColorErrorCode {
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
            -10 => Some(Err(SetColorErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for SetColorErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SetColorErrorCode::InvalidArgs => {
                "color or secondarycolor is not a valid color constant"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for SetColorErrorCode {}

impl From<SetColorErrorCode> for ErrorCode {
    fn from(value: SetColorErrorCode) -> Self {
        // Safety: SetColorErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SetColorErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Flag::set_position](crate::Flag::set_position).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Flag.setPosition).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/flags.js#L63)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SetPositionErrorCode {
    InvalidTarget = -7,
}

impl FromReturnCode for SetPositionErrorCode {
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
            -7 => Some(Err(SetPositionErrorCode::InvalidTarget)),
            _ => None,
        }
    }
}

impl fmt::Display for SetPositionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SetPositionErrorCode::InvalidTarget => "the target provided is invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SetPositionErrorCode {}

impl From<SetPositionErrorCode> for ErrorCode {
    fn from(value: SetPositionErrorCode) -> Self {
        // Safety: SetPositionErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SetPositionErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
