use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by [game::map::find_exit](crate::game::map::find_exit).
///
/// Screeps API Docs: [Game.map.findExit](https://docs.screeps.com/api/#Game.map.findExit).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/map.js#L188)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum FindExitErrorCode {
    NoPath = -2,
    InvalidArgs = -10,
}

impl FromReturnCode for FindExitErrorCode {
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
            -2 => Some(Err(FindExitErrorCode::NoPath)),
            -10 => Some(Err(FindExitErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for FindExitErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            FindExitErrorCode::NoPath => "path can not be found",
            FindExitErrorCode::InvalidArgs => "the location is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for FindExitErrorCode {}

/// Error codes used by [game::map::find_route](crate::game::map::find_route).
///
/// Screeps API Docs: [Game.map.findRoute](https://docs.screeps.com/api/#Game.map.findRoute).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/map.js#L69)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum FindRouteErrorCode {
    NoPath = -2,
}

impl FromReturnCode for FindRouteErrorCode {
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
            -2 => Some(Err(FindRouteErrorCode::NoPath)),
            _ => None,
        }
    }
}

impl fmt::Display for FindRouteErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            FindRouteErrorCode::NoPath => "path can not be found",
        };

        write!(f, "{}", msg)
    }
}

impl Error for FindRouteErrorCode {}
