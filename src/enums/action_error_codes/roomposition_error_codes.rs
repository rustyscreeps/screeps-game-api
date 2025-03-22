use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [RoomPosition::create_construction_site](crate::RoomPosition::create_construction_site).
///
///
/// [Screeps API Docs](https://docs.screeps.com/api/#RoomPosition.createConstructionSite).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/rooms.js#L1630)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RoomPositionCreateConstructionSiteErrorCode {
    NotOwner = -1,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    RclNotEnough = -14,
}

impl FromReturnCode for RoomPositionCreateConstructionSiteErrorCode {
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
            -1 => Some(Err(RoomPositionCreateConstructionSiteErrorCode::NotOwner)),
            -7 => Some(Err(
                RoomPositionCreateConstructionSiteErrorCode::InvalidTarget,
            )),
            -8 => Some(Err(RoomPositionCreateConstructionSiteErrorCode::Full)),
            -9 => Some(Err(RoomPositionCreateConstructionSiteErrorCode::NotInRange)),
            -10 => Some(Err(
                RoomPositionCreateConstructionSiteErrorCode::InvalidArgs,
            )),
            -14 => Some(Err(
                RoomPositionCreateConstructionSiteErrorCode::RclNotEnough,
            )),
            _ => None,
        }
    }
}

impl fmt::Display for RoomPositionCreateConstructionSiteErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RoomPositionCreateConstructionSiteErrorCode::NotOwner => "the room is claimed or reserved by a hostile player",
            RoomPositionCreateConstructionSiteErrorCode::InvalidTarget => "the structure cannot be placed at the specified location",
            RoomPositionCreateConstructionSiteErrorCode::Full => "you have too many construction sites. the maximum number of construction sites per player is 100",
            RoomPositionCreateConstructionSiteErrorCode::NotInRange => "room not visible",
            RoomPositionCreateConstructionSiteErrorCode::InvalidArgs => "the location is incorrect",
            RoomPositionCreateConstructionSiteErrorCode::RclNotEnough => "room controller level insufficient. learn more",
        };

        write!(f, "{}", msg)
    }
}

impl Error for RoomPositionCreateConstructionSiteErrorCode {}

impl From<RoomPositionCreateConstructionSiteErrorCode> for ErrorCode {
    fn from(value: RoomPositionCreateConstructionSiteErrorCode) -> Self {
        // Safety: RoomPositionCreateConstructionSiteErrorCode is repr(i8), so we can
        // cast it to get the discriminant value, which will match the raw return code
        // value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RoomPositionCreateConstructionSiteErrorCode discriminants are always
        // error code values, and thus the Result returned here will always be an `Err`
        // variant, so we can always extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}

/// Error codes used by
/// [RoomPosition::create_flag](crate::RoomPosition::create_flag).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#RoomPosition.createFlag).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/rooms.js#L1622)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RoomPositionCreateFlagErrorCode {
    NameExists = -3,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
}

impl FromReturnCode for RoomPositionCreateFlagErrorCode {
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
            -3 => Some(Err(RoomPositionCreateFlagErrorCode::NameExists)),
            -8 => Some(Err(RoomPositionCreateFlagErrorCode::Full)),
            -9 => Some(Err(RoomPositionCreateFlagErrorCode::NotInRange)),
            -10 => Some(Err(RoomPositionCreateFlagErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for RoomPositionCreateFlagErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RoomPositionCreateFlagErrorCode::NameExists => {
                "there is a flag with the same name already"
            }
            RoomPositionCreateFlagErrorCode::Full => {
                "you have too many flags. the maximum number of flags per player is 10000"
            }
            RoomPositionCreateFlagErrorCode::NotInRange => "room not visible",
            RoomPositionCreateFlagErrorCode::InvalidArgs => {
                "the location or the color constant is incorrect"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RoomPositionCreateFlagErrorCode {}

impl From<RoomPositionCreateFlagErrorCode> for ErrorCode {
    fn from(value: RoomPositionCreateFlagErrorCode) -> Self {
        // Safety: RoomPositionCreateFlagErrorCode is repr(i8), so we can cast it to get
        // the discriminant value, which will match the raw return code value that
        // ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RoomPositionCreateFlagErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8)
            .unwrap_err()
            .expect("expect enum discriminant to be an error code")
    }
}
