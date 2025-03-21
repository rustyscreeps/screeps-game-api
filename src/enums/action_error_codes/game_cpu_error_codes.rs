use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by
/// [game::cpu::set_shard_limits](crate::game::cpu::set_shard_limits).
///
/// Screeps API Docs: [Game.cpu.setShardLimits](https://docs.screeps.com/api/#Game.cpu.setShardLimits).
#[cfg(feature = "mmo")]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SetShardLimitsErrorCode {
    Busy = -4,
    InvalidArgs = -10,
}

impl FromReturnCode for SetShardLimitsErrorCode {
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
            -4 => Some(Err(SetShardLimitsErrorCode::Busy)),
            -10 => Some(Err(SetShardLimitsErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for SetShardLimitsErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SetShardLimitsErrorCode::Busy => "12-hours cooldown period is not over yet",
            SetShardLimitsErrorCode::InvalidArgs => {
                "the argument is not a valid shard limits object"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for SetShardLimitsErrorCode {}

/// Error codes used by [game::cpu::unlock](crate::game::cpu::unlock).
///
/// Screeps API Docs: [Game.cpu.unlock](https://docs.screeps.com/api/#Game.cpu.unlock).
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum UnlockErrorCode {
    NotEnoughResources = -6,
    Full = -8,
}

impl FromReturnCode for UnlockErrorCode {
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
            -6 => Some(Err(UnlockErrorCode::NotEnoughResources)),
            -8 => Some(Err(UnlockErrorCode::Full)),
            _ => None,
        }
    }
}

impl fmt::Display for UnlockErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            UnlockErrorCode::NotEnoughResources => {
                "your account does not have enough cpuunlock resource"
            }
            UnlockErrorCode::Full => "your cpu is unlocked with a subscription",
        };

        write!(f, "{}", msg)
    }
}

impl Error for UnlockErrorCode {}

/// Error codes used by
/// [game::cpu::generate_pixel](crate::game::cpu::generate_pixel).
///
/// Screeps API Docs: [Game.cpu.generatePixel](https://docs.screeps.com/api/#Game.cpu.generatePixel).
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum GeneratePixelErrorCode {
    NotEnoughResources = -6,
}

impl FromReturnCode for GeneratePixelErrorCode {
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
            -6 => Some(Err(GeneratePixelErrorCode::NotEnoughResources)),
            _ => None,
        }
    }
}

impl fmt::Display for GeneratePixelErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            GeneratePixelErrorCode::NotEnoughResources => "your bucket does not have enough cpu",
        };

        write!(f, "{}", msg)
    }
}

impl Error for GeneratePixelErrorCode {}
