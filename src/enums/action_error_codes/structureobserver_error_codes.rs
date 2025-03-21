use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by
/// [StructureObserver::observe_room](crate::StructureObserver::observe_room).
///
/// Screeps API Docs: [StructureObserver.observeRoom](https://docs.screeps.com/api/#StructureObserver.observeRoom).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L548)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ObserveRoomErrorCode {
    NotOwner = -1,
    NotInRange = -9,
    InvalidArgs = -10,
    RclNotEnough = -14,
}

impl FromReturnCode for ObserveRoomErrorCode {
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
            -1 => Some(Err(ObserveRoomErrorCode::NotOwner)),
            -9 => Some(Err(ObserveRoomErrorCode::NotInRange)),
            -10 => Some(Err(ObserveRoomErrorCode::InvalidArgs)),
            -14 => Some(Err(ObserveRoomErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for ObserveRoomErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ObserveRoomErrorCode::NotOwner => "you are not the owner of this structure",
            ObserveRoomErrorCode::NotInRange => "room roomname is not in observing range",
            ObserveRoomErrorCode::InvalidArgs => "roomname argument is not a valid room name value",
            ObserveRoomErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for ObserveRoomErrorCode {}
