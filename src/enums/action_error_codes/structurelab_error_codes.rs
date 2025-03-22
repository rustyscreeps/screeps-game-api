use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [StructureLab::boost_creep](crate::StructureLab::boost_creep).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureLab.boostCreep).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L412)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum BoostCreepErrorCode {
    NotOwner = -1,
    NotFound = -5,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
    RclNotEnough = -14,
}

impl FromReturnCode for BoostCreepErrorCode {
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
            -1 => Some(Err(BoostCreepErrorCode::NotOwner)),
            -5 => Some(Err(BoostCreepErrorCode::NotFound)),
            -6 => Some(Err(BoostCreepErrorCode::NotEnoughResources)),
            -7 => Some(Err(BoostCreepErrorCode::InvalidTarget)),
            -9 => Some(Err(BoostCreepErrorCode::NotInRange)),
            -14 => Some(Err(BoostCreepErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for BoostCreepErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            BoostCreepErrorCode::NotOwner => "you are not the owner of this lab",
            BoostCreepErrorCode::NotFound => {
                "the mineral containing in the lab cannot boost any of the creep's body parts"
            }
            BoostCreepErrorCode::NotEnoughResources => {
                "the lab does not have enough energy or minerals"
            }
            BoostCreepErrorCode::InvalidTarget => "the targets is not valid creep object",
            BoostCreepErrorCode::NotInRange => "the targets are too far away",
            BoostCreepErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for BoostCreepErrorCode {}

impl From<BoostCreepErrorCode> for ErrorCode {
    fn from(value: BoostCreepErrorCode) -> Self {
        // Safety: BoostCreepErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: BoostCreepErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [StructureLab::reverse_reaction](crate::StructureLab::reverse_reaction).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureLab.reverseReaction).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L360)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ReverseReactionErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for ReverseReactionErrorCode {
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
            -1 => Some(Err(ReverseReactionErrorCode::NotOwner)),
            -6 => Some(Err(ReverseReactionErrorCode::NotEnoughResources)),
            -7 => Some(Err(ReverseReactionErrorCode::InvalidTarget)),
            -8 => Some(Err(ReverseReactionErrorCode::Full)),
            -9 => Some(Err(ReverseReactionErrorCode::NotInRange)),
            -10 => Some(Err(ReverseReactionErrorCode::InvalidArgs)),
            -11 => Some(Err(ReverseReactionErrorCode::Tired)),
            -14 => Some(Err(ReverseReactionErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for ReverseReactionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ReverseReactionErrorCode::NotOwner => "you are not the owner of this lab",
            ReverseReactionErrorCode::NotEnoughResources => {
                "the source lab do not have enough resources"
            }
            ReverseReactionErrorCode::InvalidTarget => "the targets are not valid lab objects",
            ReverseReactionErrorCode::Full => "one of targets cannot receive any more resource",
            ReverseReactionErrorCode::NotInRange => "the targets are too far away",
            ReverseReactionErrorCode::InvalidArgs => {
                "the reaction cannot be reversed into this resources"
            }
            ReverseReactionErrorCode::Tired => "the lab is still cooling down",
            ReverseReactionErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for ReverseReactionErrorCode {}

impl From<ReverseReactionErrorCode> for ErrorCode {
    fn from(value: ReverseReactionErrorCode) -> Self {
        // Safety: ReverseReactionErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: ReverseReactionErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [StructureLab::run_reaction](crate::StructureLab::run_reaction).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureLab.runReaction).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L317)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RunReactionErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for RunReactionErrorCode {
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
            -1 => Some(Err(RunReactionErrorCode::NotOwner)),
            -6 => Some(Err(RunReactionErrorCode::NotEnoughResources)),
            -7 => Some(Err(RunReactionErrorCode::InvalidTarget)),
            -8 => Some(Err(RunReactionErrorCode::Full)),
            -9 => Some(Err(RunReactionErrorCode::NotInRange)),
            -10 => Some(Err(RunReactionErrorCode::InvalidArgs)),
            -11 => Some(Err(RunReactionErrorCode::Tired)),
            -14 => Some(Err(RunReactionErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for RunReactionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RunReactionErrorCode::NotOwner => "you are not the owner of this lab",
            RunReactionErrorCode::NotEnoughResources => {
                "the source lab do not have enough resources"
            }
            RunReactionErrorCode::InvalidTarget => "the targets are not valid lab objects",
            RunReactionErrorCode::Full => "the target cannot receive any more resource",
            RunReactionErrorCode::NotInRange => "the targets are too far away",
            RunReactionErrorCode::InvalidArgs => "the reaction cannot be run using this resources",
            RunReactionErrorCode::Tired => "the lab is still cooling down",
            RunReactionErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RunReactionErrorCode {}

impl From<RunReactionErrorCode> for ErrorCode {
    fn from(value: RunReactionErrorCode) -> Self {
        // Safety: RunReactionErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RunReactionErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [StructureLab::unboost_creep](crate::StructureLab::unboost_creep).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#StructureLab.unboostCreep).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/structures.js#L443)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum UnboostCreepErrorCode {
    NotOwner = -1,
    NotFound = -5,
    InvalidTarget = -7,
    NotInRange = -9,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for UnboostCreepErrorCode {
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
            -1 => Some(Err(UnboostCreepErrorCode::NotOwner)),
            -5 => Some(Err(UnboostCreepErrorCode::NotFound)),
            -7 => Some(Err(UnboostCreepErrorCode::InvalidTarget)),
            -9 => Some(Err(UnboostCreepErrorCode::NotInRange)),
            -11 => Some(Err(UnboostCreepErrorCode::Tired)),
            -14 => Some(Err(UnboostCreepErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for UnboostCreepErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            UnboostCreepErrorCode::NotOwner => {
                "you are not the owner of this lab, or the target creep"
            }
            UnboostCreepErrorCode::NotFound => "the target has no boosted parts",
            UnboostCreepErrorCode::InvalidTarget => "the target is not a valid creep object",
            UnboostCreepErrorCode::NotInRange => "the target is too far away",
            UnboostCreepErrorCode::Tired => "the lab is still cooling down",
            UnboostCreepErrorCode::RclNotEnough => {
                "room controller level insufficient to use this structure"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for UnboostCreepErrorCode {}

impl From<UnboostCreepErrorCode> for ErrorCode {
    fn from(value: UnboostCreepErrorCode) -> Self {
        // Safety: UnboostCreepErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: UnboostCreepErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
