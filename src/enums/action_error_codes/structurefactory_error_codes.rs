use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by
/// [StructureFactory::produce](crate::StructureFactory::produce).
///
/// Screeps API Docs: [StructureFactory.produce](https://docs.screeps.com/api/#StructureFactory.produce).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L1434)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ProduceErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    InvalidArgs = -10,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for ProduceErrorCode {
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
            -1 => Some(Err(ProduceErrorCode::NotOwner)),
            -4 => Some(Err(ProduceErrorCode::Busy)),
            -6 => Some(Err(ProduceErrorCode::NotEnoughResources)),
            -7 => Some(Err(ProduceErrorCode::InvalidTarget)),
            -8 => Some(Err(ProduceErrorCode::Full)),
            -10 => Some(Err(ProduceErrorCode::InvalidArgs)),
            -11 => Some(Err(ProduceErrorCode::Tired)),
            -14 => Some(Err(ProduceErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for ProduceErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ProduceErrorCode::NotOwner => "you are not the owner of this structure",
            ProduceErrorCode::Busy => {
                "the factory is not operated by the pwr_operate_factory power"
            }
            ProduceErrorCode::NotEnoughResources => {
                "the structure does not have the required amount of resources"
            }
            ProduceErrorCode::InvalidTarget => {
                "the factory cannot produce the commodity of this level"
            }
            ProduceErrorCode::Full => "the factory cannot contain the produce",
            ProduceErrorCode::InvalidArgs => "the arguments provided are incorrect",
            ProduceErrorCode::Tired => "the factory is still cooling down",
            ProduceErrorCode::RclNotEnough => {
                "your room controller level is insufficient to use the factory"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for ProduceErrorCode {}
