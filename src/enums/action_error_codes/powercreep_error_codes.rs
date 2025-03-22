use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by [PowerCreep::create](crate::PowerCreep::create).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.create).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L395)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PowerCreepCreateErrorCode {
    NameExists = -3,
    NotEnoughResources = -6,
    InvalidArgs = -10,
}

impl FromReturnCode for PowerCreepCreateErrorCode {
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
            -3 => Some(Err(PowerCreepCreateErrorCode::NameExists)),
            -6 => Some(Err(PowerCreepCreateErrorCode::NotEnoughResources)),
            -10 => Some(Err(PowerCreepCreateErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for PowerCreepCreateErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PowerCreepCreateErrorCode::NameExists => "a power creep with the specified name already exists",
            PowerCreepCreateErrorCode::NotEnoughResources => "you don't have free power levels in your account",
            PowerCreepCreateErrorCode::InvalidArgs => "the provided power creep name is exceeds the limit, or the power creep class is invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for PowerCreepCreateErrorCode {}

impl From<PowerCreepCreateErrorCode> for ErrorCode {
    fn from(value: PowerCreepCreateErrorCode) -> Self {
        // Safety: PowerCreepCreateErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PowerCreepCreateErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [PowerCreep::cancel_order](crate::PowerCreep::cancel_order).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.cancelOrder).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L342)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PowerCreepCancelOrderErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotFound = -5,
}

impl FromReturnCode for PowerCreepCancelOrderErrorCode {
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
            -1 => Some(Err(PowerCreepCancelOrderErrorCode::NotOwner)),
            -4 => Some(Err(PowerCreepCancelOrderErrorCode::Busy)),
            -5 => Some(Err(PowerCreepCancelOrderErrorCode::NotFound)),
            _ => None,
        }
    }
}

impl fmt::Display for PowerCreepCancelOrderErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PowerCreepCancelOrderErrorCode::NotOwner => "you are not the owner of the creep",
            PowerCreepCancelOrderErrorCode::Busy => "the power creep is not spawned in the world",
            PowerCreepCancelOrderErrorCode::NotFound => {
                "the order with the specified name is not found"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for PowerCreepCancelOrderErrorCode {}

impl From<PowerCreepCancelOrderErrorCode> for ErrorCode {
    fn from(value: PowerCreepCancelOrderErrorCode) -> Self {
        // Safety: PowerCreepCancelOrderErrorCode is repr(i8), so we can cast it to get
        // the discriminant value, which will match the raw return code value that
        // ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PowerCreepCancelOrderErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [AccountPowerCreep::delete](crate::AccountPowerCreep::delete).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.delete).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L204)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum DeleteErrorCode {
    NotOwner = -1,
    Busy = -4,
}

impl FromReturnCode for DeleteErrorCode {
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
            -1 => Some(Err(DeleteErrorCode::NotOwner)),
            -4 => Some(Err(DeleteErrorCode::Busy)),
            _ => None,
        }
    }
}

impl fmt::Display for DeleteErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            DeleteErrorCode::NotOwner => "you are not the owner of the creep",
            DeleteErrorCode::Busy => "the power creep is spawned in the world",
        };

        write!(f, "{}", msg)
    }
}

impl Error for DeleteErrorCode {}

impl From<DeleteErrorCode> for ErrorCode {
    fn from(value: DeleteErrorCode) -> Self {
        // Safety: DeleteErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: DeleteErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [PowerCreep::enable_room](crate::PowerCreep::enable_room).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.enableRoom).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L295)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum EnableRoomErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
}

impl FromReturnCode for EnableRoomErrorCode {
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
            -1 => Some(Err(EnableRoomErrorCode::NotOwner)),
            -4 => Some(Err(EnableRoomErrorCode::Busy)),
            -7 => Some(Err(EnableRoomErrorCode::InvalidTarget)),
            -9 => Some(Err(EnableRoomErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for EnableRoomErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            EnableRoomErrorCode::NotOwner => "you are not the owner of this creep",
            EnableRoomErrorCode::Busy => "the power creep is not spawned in the world",
            EnableRoomErrorCode::InvalidTarget => "the target is not a controller structure",
            EnableRoomErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for EnableRoomErrorCode {}

impl From<EnableRoomErrorCode> for ErrorCode {
    fn from(value: EnableRoomErrorCode) -> Self {
        // Safety: EnableRoomErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: EnableRoomErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [PowerCreep::move_direction](crate::PowerCreep::move_direction).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.move).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L106)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PowerCreepMoveDirectionErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
}

impl FromReturnCode for PowerCreepMoveDirectionErrorCode {
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
            -1 => Some(Err(PowerCreepMoveDirectionErrorCode::NotOwner)),
            -4 => Some(Err(PowerCreepMoveDirectionErrorCode::Busy)),
            -9 => Some(Err(PowerCreepMoveDirectionErrorCode::NotInRange)),
            -10 => Some(Err(PowerCreepMoveDirectionErrorCode::InvalidArgs)),
            -11 => Some(Err(PowerCreepMoveDirectionErrorCode::Tired)),
            _ => None,
        }
    }
}

impl fmt::Display for PowerCreepMoveDirectionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PowerCreepMoveDirectionErrorCode::NotOwner => "you are not the owner of this creep",
            PowerCreepMoveDirectionErrorCode::Busy => "the power creep is not spawned in the world",
            PowerCreepMoveDirectionErrorCode::NotInRange => "the target creep is too far away",
            PowerCreepMoveDirectionErrorCode::InvalidArgs => "the provided direction is incorrect",
            PowerCreepMoveDirectionErrorCode::Tired => {
                "the fatigue indicator of the creep is non-zero"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for PowerCreepMoveDirectionErrorCode {}

impl From<PowerCreepMoveDirectionErrorCode> for ErrorCode {
    fn from(value: PowerCreepMoveDirectionErrorCode) -> Self {
        // Safety: PowerCreepMoveDirectionErrorCode is repr(i8), so we can cast it to
        // get the discriminant value, which will match the raw return code value that
        // ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PowerCreepMoveDirectionErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [PowerCreep::move_by_path](crate::PowerCreep::move_by_path).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.moveByPath).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L120)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PowerCreepMoveByPathErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotFound = -5,
    InvalidArgs = -10,
    Tired = -11,
}

impl FromReturnCode for PowerCreepMoveByPathErrorCode {
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
            -1 => Some(Err(PowerCreepMoveByPathErrorCode::NotOwner)),
            -4 => Some(Err(PowerCreepMoveByPathErrorCode::Busy)),
            -5 => Some(Err(PowerCreepMoveByPathErrorCode::NotFound)),
            -10 => Some(Err(PowerCreepMoveByPathErrorCode::InvalidArgs)),
            -11 => Some(Err(PowerCreepMoveByPathErrorCode::Tired)),
            _ => None,
        }
    }
}

impl fmt::Display for PowerCreepMoveByPathErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PowerCreepMoveByPathErrorCode::NotOwner => "you are not the owner of this creep",
            PowerCreepMoveByPathErrorCode::Busy => "the power creep is not spawned in the world",
            PowerCreepMoveByPathErrorCode::NotFound => {
                "the specified path doesn't match the creep's location"
            }
            PowerCreepMoveByPathErrorCode::InvalidArgs => "path is not a valid path array",
            PowerCreepMoveByPathErrorCode::Tired => {
                "the fatigue indicator of the creep is non-zero"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for PowerCreepMoveByPathErrorCode {}

impl From<PowerCreepMoveByPathErrorCode> for ErrorCode {
    fn from(value: PowerCreepMoveByPathErrorCode) -> Self {
        // Safety: PowerCreepMoveByPathErrorCode is repr(i8), so we can cast it to get
        // the discriminant value, which will match the raw return code value that
        // ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PowerCreepMoveByPathErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [PowerCreep::move_to](crate::PowerCreep#method.move_to).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.moveTo).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L113)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PowerCreepMoveToErrorCode {
    NotOwner = -1,
    NoPath = -2,
    Busy = -4,
    NotFound = -5,
    InvalidTarget = -7,
    Tired = -11,
}

impl FromReturnCode for PowerCreepMoveToErrorCode {
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
            -1 => Some(Err(PowerCreepMoveToErrorCode::NotOwner)),
            -2 => Some(Err(PowerCreepMoveToErrorCode::NoPath)),
            -4 => Some(Err(PowerCreepMoveToErrorCode::Busy)),
            -5 => Some(Err(PowerCreepMoveToErrorCode::NotFound)),
            -7 => Some(Err(PowerCreepMoveToErrorCode::InvalidTarget)),
            -11 => Some(Err(PowerCreepMoveToErrorCode::Tired)),
            _ => None,
        }
    }
}

impl fmt::Display for PowerCreepMoveToErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PowerCreepMoveToErrorCode::NotOwner => "you are not the owner of this creep",
            PowerCreepMoveToErrorCode::NoPath => "no path to the target could be found",
            PowerCreepMoveToErrorCode::Busy => "the power creep is not spawned in the world",
            PowerCreepMoveToErrorCode::NotFound => "the creep has no memorized path to reuse",
            PowerCreepMoveToErrorCode::InvalidTarget => "the target provided is invalid",
            PowerCreepMoveToErrorCode::Tired => "the fatigue indicator of the creep is non-zero",
        };

        write!(f, "{}", msg)
    }
}

impl Error for PowerCreepMoveToErrorCode {}

impl From<PowerCreepMoveToErrorCode> for ErrorCode {
    fn from(value: PowerCreepMoveToErrorCode) -> Self {
        // Safety: PowerCreepMoveToErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PowerCreepMoveToErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [AccountPowerCreep::rename](crate::AccountPowerCreep::rename).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.rename).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L356)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RenameErrorCode {
    NotOwner = -1,
    NameExists = -3,
    Busy = -4,
    InvalidArgs = -10,
}

impl FromReturnCode for RenameErrorCode {
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
            -1 => Some(Err(RenameErrorCode::NotOwner)),
            -3 => Some(Err(RenameErrorCode::NameExists)),
            -4 => Some(Err(RenameErrorCode::Busy)),
            -10 => Some(Err(RenameErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for RenameErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RenameErrorCode::NotOwner => "you are not the owner of the creep",
            RenameErrorCode::NameExists => "a power creep with the specified name already exists",
            RenameErrorCode::Busy => "the power creep is spawned in the world",
            RenameErrorCode::InvalidArgs => "the provided power creep name is exceeds the limit",
        };

        write!(f, "{}", msg)
    }
}

impl Error for RenameErrorCode {}

impl From<RenameErrorCode> for ErrorCode {
    fn from(value: RenameErrorCode) -> Self {
        // Safety: RenameErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RenameErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [PowerCreep::renew](crate::PowerCreep::renew).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.renew).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L319)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RenewErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
}

impl FromReturnCode for RenewErrorCode {
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
            -1 => Some(Err(RenewErrorCode::NotOwner)),
            -4 => Some(Err(RenewErrorCode::Busy)),
            -7 => Some(Err(RenewErrorCode::InvalidTarget)),
            -9 => Some(Err(RenewErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for RenewErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RenewErrorCode::NotOwner => "you are not the owner of this creep",
            RenewErrorCode::Busy => "the power creep is not spawned in the world",
            RenewErrorCode::InvalidTarget => "the target is not a valid power bank object",
            RenewErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for RenewErrorCode {}

impl From<RenewErrorCode> for ErrorCode {
    fn from(value: RenewErrorCode) -> Self {
        // Safety: RenewErrorCode is repr(i8), so we can cast it to get the discriminant
        // value, which will match the raw return code value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RenewErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [AccountPowerCreep::spawn](crate::AccountPowerCreep::spawn).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.spawn).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L162)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SpawnErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    Tired = -11,
    RclNotEnough = -14,
}

impl FromReturnCode for SpawnErrorCode {
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
            -1 => Some(Err(SpawnErrorCode::NotOwner)),
            -4 => Some(Err(SpawnErrorCode::Busy)),
            -7 => Some(Err(SpawnErrorCode::InvalidTarget)),
            -11 => Some(Err(SpawnErrorCode::Tired)),
            -14 => Some(Err(SpawnErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for SpawnErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SpawnErrorCode::NotOwner => "you are not the owner of the creep or the spawn",
            SpawnErrorCode::Busy => "the power creep is already spawned in the world",
            SpawnErrorCode::InvalidTarget => "the specified object is not a power spawn",
            SpawnErrorCode::Tired => "the power creep cannot be spawned because of the cooldown",
            SpawnErrorCode::RclNotEnough => "room controller level insufficient to use the spawn",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SpawnErrorCode {}

impl From<SpawnErrorCode> for ErrorCode {
    fn from(value: SpawnErrorCode) -> Self {
        // Safety: SpawnErrorCode is repr(i8), so we can cast it to get the discriminant
        // value, which will match the raw return code value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SpawnErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [AccountPowerCreep::upgrade](crate::AccountPowerCreep::upgrade).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.upgrade).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L217)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum UpgradeErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    Full = -8,
    InvalidArgs = -10,
}

impl FromReturnCode for UpgradeErrorCode {
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
            -1 => Some(Err(UpgradeErrorCode::NotOwner)),
            -6 => Some(Err(UpgradeErrorCode::NotEnoughResources)),
            -8 => Some(Err(UpgradeErrorCode::Full)),
            -10 => Some(Err(UpgradeErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for UpgradeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            UpgradeErrorCode::NotOwner => "you are not the owner of the creep",
            UpgradeErrorCode::NotEnoughResources => "you account power level is not enough",
            UpgradeErrorCode::Full => "the specified power cannot be upgraded on this creep's level, or the creep reached the maximum level",
            UpgradeErrorCode::InvalidArgs => "the specified power id is not valid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for UpgradeErrorCode {}

impl From<UpgradeErrorCode> for ErrorCode {
    fn from(value: UpgradeErrorCode) -> Self {
        // Safety: UpgradeErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: UpgradeErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [PowerCreep::use_power](crate::PowerCreep::use_power).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#PowerCreep.usePower).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/power-creeps.js#L246)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum UsePowerErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
}

impl FromReturnCode for UsePowerErrorCode {
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
            -1 => Some(Err(UsePowerErrorCode::NotOwner)),
            -4 => Some(Err(UsePowerErrorCode::Busy)),
            -6 => Some(Err(UsePowerErrorCode::NotEnoughResources)),
            -7 => Some(Err(UsePowerErrorCode::InvalidTarget)),
            -8 => Some(Err(UsePowerErrorCode::Full)),
            -9 => Some(Err(UsePowerErrorCode::NotInRange)),
            -10 => Some(Err(UsePowerErrorCode::InvalidArgs)),
            -11 => Some(Err(UsePowerErrorCode::Tired)),
            -12 => Some(Err(UsePowerErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for UsePowerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            UsePowerErrorCode::NotOwner => "you are not the owner of the creep",
            UsePowerErrorCode::Busy => "the creep is not spawned in the world",
            UsePowerErrorCode::NotEnoughResources => {
                "the creep doesn't have enough resources to use the power"
            }
            UsePowerErrorCode::InvalidTarget => "the specified target is not valid",
            UsePowerErrorCode::Full => "the target has the same active effect of a higher level",
            UsePowerErrorCode::NotInRange => "the specified target is too far away",
            UsePowerErrorCode::InvalidArgs => "using powers is not enabled on the room controller",
            UsePowerErrorCode::Tired => "the power ability is still on cooldown",
            UsePowerErrorCode::NoBodypart => "the creep doesn't have the specified power ability",
        };

        write!(f, "{}", msg)
    }
}

impl Error for UsePowerErrorCode {}

impl From<UsePowerErrorCode> for ErrorCode {
    fn from(value: UsePowerErrorCode) -> Self {
        // Safety: UsePowerErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: UsePowerErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
