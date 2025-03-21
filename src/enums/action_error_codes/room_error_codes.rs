use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by
/// [Room::create_construction_site](crate::Room::create_construction_site).
///
/// Screeps API Docs: [Room.createConstructionSite](https://docs.screeps.com/api/#Room.createConstructionSite).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/rooms.js#L1029)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RoomCreateConstructionSiteErrorCode {
    NotOwner = -1,
    InvalidTarget = -7,
    Full = -8,
    InvalidArgs = -10,
    RclNotEnough = -14,
}

impl FromReturnCode for RoomCreateConstructionSiteErrorCode {
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
            -1 => Some(Err(RoomCreateConstructionSiteErrorCode::NotOwner)),
            -7 => Some(Err(RoomCreateConstructionSiteErrorCode::InvalidTarget)),
            -8 => Some(Err(RoomCreateConstructionSiteErrorCode::Full)),
            -10 => Some(Err(RoomCreateConstructionSiteErrorCode::InvalidArgs)),
            -14 => Some(Err(RoomCreateConstructionSiteErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for RoomCreateConstructionSiteErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RoomCreateConstructionSiteErrorCode::NotOwner => "the room is claimed or reserved by a hostile player",
            RoomCreateConstructionSiteErrorCode::InvalidTarget => "the structure cannot be placed at the specified location",
            RoomCreateConstructionSiteErrorCode::Full => "you have too many construction sites. the maximum number of construction sites per player is 100",
            RoomCreateConstructionSiteErrorCode::InvalidArgs => "the location is incorrect",
            RoomCreateConstructionSiteErrorCode::RclNotEnough => "room controller level insufficient. learn more",
        };

        write!(f, "{}", msg)
    }
}

impl Error for RoomCreateConstructionSiteErrorCode {}

/// Error codes used by [Room::create_flag](crate::Room::create_flag).
///
/// Screeps API Docs: [Room.createFlag](https://docs.screeps.com/api/#Room.createFlag).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/rooms.js#L978)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RoomCreateFlagErrorCode {
    NameExists = -3,
    Full = -8,
    InvalidArgs = -10,
}

impl FromReturnCode for RoomCreateFlagErrorCode {
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
            -3 => Some(Err(RoomCreateFlagErrorCode::NameExists)),
            -8 => Some(Err(RoomCreateFlagErrorCode::Full)),
            -10 => Some(Err(RoomCreateFlagErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for RoomCreateFlagErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RoomCreateFlagErrorCode::NameExists => "there is a flag with the same name already",
            RoomCreateFlagErrorCode::Full => {
                "you have too many flags. the maximum number of flags per player is 10000"
            }
            RoomCreateFlagErrorCode::InvalidArgs => {
                "the location or the name or the color constant is incorrect"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RoomCreateFlagErrorCode {}

/// Error codes used by [Room::find_exit_to](crate::Room::find_exit_to).
///
/// Screeps API Docs: [Room.findExitTo](https://docs.screeps.com/api/#Room.findExitTo).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/rooms.js#L1130)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum FindExitToErrorCode {
    NoPath = -2,
    InvalidArgs = -10,
}

impl FromReturnCode for FindExitToErrorCode {
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
            -2 => Some(Err(FindExitToErrorCode::NoPath)),
            -10 => Some(Err(FindExitToErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for FindExitToErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            FindExitToErrorCode::NoPath => "path can not be found",
            FindExitToErrorCode::InvalidArgs => "the location is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for FindExitToErrorCode {}
