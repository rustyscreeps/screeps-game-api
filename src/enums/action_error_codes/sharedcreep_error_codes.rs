use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by
/// [SharedCreepProperties::drop](crate::SharedCreepProperties::drop).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.drop).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L404)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum DropErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidArgs = -10,
}

impl FromReturnCode for DropErrorCode {
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
            -1 => Some(Err(DropErrorCode::NotOwner)),
            -4 => Some(Err(DropErrorCode::Busy)),
            -6 => Some(Err(DropErrorCode::NotEnoughResources)),
            -10 => Some(Err(DropErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for DropErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            DropErrorCode::NotOwner => "you are not the owner of this creep",
            DropErrorCode::Busy => "the creep is still being spawned",
            DropErrorCode::NotEnoughResources => {
                "the creep does not have the given amount of resources"
            }
            DropErrorCode::InvalidArgs => "the resourcetype is not a valid resource_* constants",
        };

        write!(f, "{}", msg)
    }
}

impl Error for DropErrorCode {}

impl From<DropErrorCode> for ErrorCode {
    fn from(value: DropErrorCode) -> Self {
        // Safety: DropErrorCode is repr(i8), so we can cast it to get the discriminant
        // value, which will match the raw return code value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: DropErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [SharedCreepProperties::notify_when_attacked](crate::SharedCreepProperties::notify_when_attacked).
///
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.notifyWhenAttacked).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L988)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum NotifyWhenAttackedErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidArgs = -10,
}

impl FromReturnCode for NotifyWhenAttackedErrorCode {
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
            -1 => Some(Err(NotifyWhenAttackedErrorCode::NotOwner)),
            -4 => Some(Err(NotifyWhenAttackedErrorCode::Busy)),
            -10 => Some(Err(NotifyWhenAttackedErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for NotifyWhenAttackedErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            NotifyWhenAttackedErrorCode::NotOwner => "you are not the owner of this creep",
            NotifyWhenAttackedErrorCode::Busy => "the creep is still being spawned",
            NotifyWhenAttackedErrorCode::InvalidArgs => "enable argument is not a boolean value",
        };

        write!(f, "{}", msg)
    }
}

impl Error for NotifyWhenAttackedErrorCode {}

impl From<NotifyWhenAttackedErrorCode> for ErrorCode {
    fn from(value: NotifyWhenAttackedErrorCode) -> Self {
        // Safety: NotifyWhenAttackedErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: NotifyWhenAttackedErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [SharedCreepProperties::pickup](crate::SharedCreepProperties::pickup).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.pickup).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L566)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PickupErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
}

impl FromReturnCode for PickupErrorCode {
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
            -1 => Some(Err(PickupErrorCode::NotOwner)),
            -4 => Some(Err(PickupErrorCode::Busy)),
            -7 => Some(Err(PickupErrorCode::InvalidTarget)),
            -8 => Some(Err(PickupErrorCode::Full)),
            -9 => Some(Err(PickupErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for PickupErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PickupErrorCode::NotOwner => "you are not the owner of this creep",
            PickupErrorCode::Busy => "the creep is still being spawned",
            PickupErrorCode::InvalidTarget => "the target is not a valid object to pick up",
            PickupErrorCode::Full => "the creep cannot receive any more resource",
            PickupErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for PickupErrorCode {}

impl From<PickupErrorCode> for ErrorCode {
    fn from(value: PickupErrorCode) -> Self {
        // Safety: PickupErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PickupErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [SharedCreepProperties::say](crate::SharedCreepProperties::say).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.say).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L826)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SayErrorCode {
    NotOwner = -1,
    Busy = -4,
}

impl FromReturnCode for SayErrorCode {
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
            -1 => Some(Err(SayErrorCode::NotOwner)),
            -4 => Some(Err(SayErrorCode::Busy)),
            _ => None,
        }
    }
}

impl fmt::Display for SayErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SayErrorCode::NotOwner => "you are not the owner of this creep",
            SayErrorCode::Busy => "the creep is still being spawned",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SayErrorCode {}

impl From<SayErrorCode> for ErrorCode {
    fn from(value: SayErrorCode) -> Self {
        // Safety: SayErrorCode is repr(i8), so we can cast it to get the discriminant
        // value, which will match the raw return code value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SayErrorCode discriminants are always error code values, and thus the
        // Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [SharedCreepProperties::suicide](crate::SharedCreepProperties::suicide).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.suicide).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L813)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SuicideErrorCode {
    NotOwner = -1,
    Busy = -4,
}

impl FromReturnCode for SuicideErrorCode {
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
            -1 => Some(Err(SuicideErrorCode::NotOwner)),
            -4 => Some(Err(SuicideErrorCode::Busy)),
            _ => None,
        }
    }
}

impl fmt::Display for SuicideErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SuicideErrorCode::NotOwner => "you are not the owner of this creep",
            SuicideErrorCode::Busy => "the creep is still being spawned",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SuicideErrorCode {}

impl From<SuicideErrorCode> for ErrorCode {
    fn from(value: SuicideErrorCode) -> Self {
        // Safety: SuicideErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SuicideErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [SharedCreepProperties::transfer](crate::SharedCreepProperties::transfer).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.transfer).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L428)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum TransferErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
}

impl FromReturnCode for TransferErrorCode {
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
            -1 => Some(Err(TransferErrorCode::NotOwner)),
            -4 => Some(Err(TransferErrorCode::Busy)),
            -6 => Some(Err(TransferErrorCode::NotEnoughResources)),
            -7 => Some(Err(TransferErrorCode::InvalidTarget)),
            -8 => Some(Err(TransferErrorCode::Full)),
            -9 => Some(Err(TransferErrorCode::NotInRange)),
            -10 => Some(Err(TransferErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for TransferErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            TransferErrorCode::NotOwner => "you are not the owner of this creep",
            TransferErrorCode::Busy => "the creep is still being spawned",
            TransferErrorCode::NotEnoughResources => "the creep does not have the given amount of resources",
            TransferErrorCode::InvalidTarget => "the target is not a valid object which can contain the specified resource",
            TransferErrorCode::Full => "the target cannot receive any more resources",
            TransferErrorCode::NotInRange => "the target is too far away",
            TransferErrorCode::InvalidArgs => "the resourcetype is not one of the resource_* constants, or the amount is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for TransferErrorCode {}

impl From<TransferErrorCode> for ErrorCode {
    fn from(value: TransferErrorCode) -> Self {
        // Safety: TransferErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: TransferErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [SharedCreepProperties::withdraw](crate::SharedCreepProperties::withdraw).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.withdraw).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L493)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum WithdrawErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
}

impl FromReturnCode for WithdrawErrorCode {
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
            -1 => Some(Err(WithdrawErrorCode::NotOwner)),
            -4 => Some(Err(WithdrawErrorCode::Busy)),
            -6 => Some(Err(WithdrawErrorCode::NotEnoughResources)),
            -7 => Some(Err(WithdrawErrorCode::InvalidTarget)),
            -8 => Some(Err(WithdrawErrorCode::Full)),
            -9 => Some(Err(WithdrawErrorCode::NotInRange)),
            -10 => Some(Err(WithdrawErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for WithdrawErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            WithdrawErrorCode::NotOwner => "you are not the owner of this creep, or there is a hostile rampart on top of the target",
            WithdrawErrorCode::Busy => "the creep is still being spawned",
            WithdrawErrorCode::NotEnoughResources => "the target does not have the given amount of resources",
            WithdrawErrorCode::InvalidTarget => "the target is not a valid object which can contain the specified resource",
            WithdrawErrorCode::Full => "the creep's carry is full",
            WithdrawErrorCode::NotInRange => "the target is too far away",
            WithdrawErrorCode::InvalidArgs => "the resourcetype is not one of the resource_* constants, or the amount is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for WithdrawErrorCode {}

impl From<WithdrawErrorCode> for ErrorCode {
    fn from(value: WithdrawErrorCode) -> Self {
        // Safety: WithdrawErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: WithdrawErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
