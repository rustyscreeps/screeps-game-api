use std::fmt;
use std::error::Error;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by [StructureTerminal::send](crate::StructureTerminal::send).
///
/// Screeps API Docs: [StructureTerminal.send](https://docs.screeps.com/api/#StructureTerminal.send).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L714)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SendErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    InvalidArgs = -10,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for SendErrorCode {
    type Error = Self;

    fn result_from_i8(val: i8) -> Result<(), Self::Error> {
        let maybe_result = Self::try_result_from_i8(val);
        #[cfg(feature="unsafe-return-conversion")]
        unsafe {
            maybe_result.unwrap_unchecked()
        }
        #[cfg(not(feature="unsafe-return-conversion"))]
        maybe_result.unwrap()
    }

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>> {
        match val {
            0 => Some(Ok(())),
            -1 => Some(Err(SendErrorCode::NotOwner)),
            -6 => Some(Err(SendErrorCode::NotEnoughResources)),
            -10 => Some(Err(SendErrorCode::InvalidArgs)),
            -11 => Some(Err(SendErrorCode::Tired)),
            -14 => Some(Err(SendErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for SendErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SendErrorCode::NotOwner => "you are not the owner of this structure",
            SendErrorCode::NotEnoughResources => "the structure does not have the required amount of resources",
            SendErrorCode::InvalidArgs => "the arguments provided are incorrect",
            SendErrorCode::Tired => "the terminal is still cooling down",
            SendErrorCode::RclNotEnough => "your room controller level is insufficient to use this terminal",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SendErrorCode {}