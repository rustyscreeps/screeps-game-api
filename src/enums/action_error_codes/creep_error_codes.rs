use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{constants::ErrorCode, FromReturnCode};

/// Error codes used by [Creep::claim_reactor](crate::Creep::claim_reactor).
///
/// [Screeps API Docs](https://docs-season.screeps.com/api/#Creep.claimReactor).
///
/// [Screeps Engine Source Code](https://github.com/screeps/mod-season5/blob/master/src/creep.claimReactor.js#L14)
#[cfg(feature = "seasonal-season-5")]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepClaimReactorErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

#[cfg(feature = "seasonal-season-5")]
impl FromReturnCode for CreepClaimReactorErrorCode {
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
            -1 => Some(Err(CreepClaimReactorErrorCode::NotOwner)),
            -4 => Some(Err(CreepClaimReactorErrorCode::Busy)),
            -7 => Some(Err(CreepClaimReactorErrorCode::InvalidTarget)),
            -9 => Some(Err(CreepClaimReactorErrorCode::NotInRange)),
            -12 => Some(Err(CreepClaimReactorErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

#[cfg(feature = "seasonal-season-5")]
impl fmt::Display for CreepClaimReactorErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepClaimReactorErrorCode::NotOwner => "you are not the owner of this creep",
            CreepClaimReactorErrorCode::Busy => "the creep is still being spawned",
            CreepClaimReactorErrorCode::InvalidTarget => "the target is not a reactor",
            CreepClaimReactorErrorCode::NotInRange => "the target is too far away",
            CreepClaimReactorErrorCode::NoBodypart => {
                "there are no claim body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

#[cfg(feature = "seasonal-season-5")]
impl Error for CreepClaimReactorErrorCode {}

#[cfg(feature = "seasonal-season-5")]
impl From<CreepClaimReactorErrorCode> for ErrorCode {
    fn from(value: CreepClaimReactorErrorCode) -> Self {
        // Safety: CreepClaimReactorErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepClaimReactorErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::attack](crate::Creep::attack).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.attack).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L593)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepAttackErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for CreepAttackErrorCode {
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
            -1 => Some(Err(CreepAttackErrorCode::NotOwner)),
            -4 => Some(Err(CreepAttackErrorCode::Busy)),
            -7 => Some(Err(CreepAttackErrorCode::InvalidTarget)),
            -9 => Some(Err(CreepAttackErrorCode::NotInRange)),
            -12 => Some(Err(CreepAttackErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepAttackErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepAttackErrorCode::NotOwner => "you are not the owner of this creep",
            CreepAttackErrorCode::Busy => "the creep is still being spawned",
            CreepAttackErrorCode::InvalidTarget => "the target is not a valid attackable object",
            CreepAttackErrorCode::NotInRange => "the target is too far away",
            CreepAttackErrorCode::NoBodypart => {
                "there are no attack body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepAttackErrorCode {}

impl From<CreepAttackErrorCode> for ErrorCode {
    fn from(value: CreepAttackErrorCode) -> Self {
        // Safety: CreepAttackErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepAttackErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Creep::attack_controller](crate::Creep::attack_controller).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.attackController).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L885)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum AttackControllerErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    Tired = -11,
    NoBodypart = -12,
}

impl FromReturnCode for AttackControllerErrorCode {
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
            -1 => Some(Err(AttackControllerErrorCode::NotOwner)),
            -4 => Some(Err(AttackControllerErrorCode::Busy)),
            -7 => Some(Err(AttackControllerErrorCode::InvalidTarget)),
            -9 => Some(Err(AttackControllerErrorCode::NotInRange)),
            -11 => Some(Err(AttackControllerErrorCode::Tired)),
            -12 => Some(Err(AttackControllerErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for AttackControllerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            AttackControllerErrorCode::NotOwner => "you are not the owner of this creep",
            AttackControllerErrorCode::Busy => "the creep is still being spawned",
            AttackControllerErrorCode::InvalidTarget => {
                "the target is not a valid owned or reserved controller object"
            }
            AttackControllerErrorCode::NotInRange => "the target is too far away",
            AttackControllerErrorCode::Tired => {
                "you have to wait until the next attack is possible"
            }
            AttackControllerErrorCode::NoBodypart => {
                "there are not enough claim body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for AttackControllerErrorCode {}

impl From<AttackControllerErrorCode> for ErrorCode {
    fn from(value: AttackControllerErrorCode) -> Self {
        // Safety: AttackControllerErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: AttackControllerErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::build](crate::Creep::build).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.build).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L762)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum BuildErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for BuildErrorCode {
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
            -1 => Some(Err(BuildErrorCode::NotOwner)),
            -4 => Some(Err(BuildErrorCode::Busy)),
            -6 => Some(Err(BuildErrorCode::NotEnoughResources)),
            -7 => Some(Err(BuildErrorCode::InvalidTarget)),
            -9 => Some(Err(BuildErrorCode::NotInRange)),
            -12 => Some(Err(BuildErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for BuildErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            BuildErrorCode::NotOwner => "you are not the owner of this creep",
            BuildErrorCode::Busy => "the creep is still being spawned",
            BuildErrorCode::NotEnoughResources => "the creep does not have any carried energy",
            BuildErrorCode::InvalidTarget => "the target is not a valid construction site object or the structure cannot be built here (probably because of a creep at the same square)",
            BuildErrorCode::NotInRange => "the target is too far away",
            BuildErrorCode::NoBodypart => "there are no work body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for BuildErrorCode {}

impl From<BuildErrorCode> for ErrorCode {
    fn from(value: BuildErrorCode) -> Self {
        // Safety: BuildErrorCode is repr(i8), so we can cast it to get the discriminant
        // value, which will match the raw return code value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: BuildErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::cancel_order](crate::Creep::cancel_order).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.cancelOrder).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L1008)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepCancelOrderErrorCode {
    NotFound = -5,
}

impl FromReturnCode for CreepCancelOrderErrorCode {
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
            -5 => Some(Err(CreepCancelOrderErrorCode::NotFound)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepCancelOrderErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepCancelOrderErrorCode::NotFound => "the order with the specified name is not found",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepCancelOrderErrorCode {}

impl From<CreepCancelOrderErrorCode> for ErrorCode {
    fn from(value: CreepCancelOrderErrorCode) -> Self {
        // Safety: CreepCancelOrderErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepCancelOrderErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Creep::claim_controller](crate::Creep::claim_controller).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.claimController).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L839)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ClaimControllerErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    NoBodypart = -12,
    GclNotEnough = -15,
}

impl FromReturnCode for ClaimControllerErrorCode {
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
            -1 => Some(Err(ClaimControllerErrorCode::NotOwner)),
            -4 => Some(Err(ClaimControllerErrorCode::Busy)),
            -7 => Some(Err(ClaimControllerErrorCode::InvalidTarget)),
            -8 => Some(Err(ClaimControllerErrorCode::Full)),
            -9 => Some(Err(ClaimControllerErrorCode::NotInRange)),
            -12 => Some(Err(ClaimControllerErrorCode::NoBodypart)),
            -15 => Some(Err(ClaimControllerErrorCode::GclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for ClaimControllerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ClaimControllerErrorCode::NotOwner => "you are not the owner of this creep",
            ClaimControllerErrorCode::Busy => "the creep is still being spawned",
            ClaimControllerErrorCode::InvalidTarget => {
                "the target is not a valid neutral controller object"
            }
            ClaimControllerErrorCode::Full => {
                "you cannot claim more than 3 rooms in the novice area"
            }
            ClaimControllerErrorCode::NotInRange => "the target is too far away",
            ClaimControllerErrorCode::NoBodypart => {
                "there are no claim body parts in this creep’s body"
            }
            ClaimControllerErrorCode::GclNotEnough => "your global control level is not enough",
        };

        write!(f, "{}", msg)
    }
}

impl Error for ClaimControllerErrorCode {}

impl From<ClaimControllerErrorCode> for ErrorCode {
    fn from(value: ClaimControllerErrorCode) -> Self {
        // Safety: ClaimControllerErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: ClaimControllerErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::dismantle](crate::Creep::dismantle).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.dismantle).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L1016)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum DismantleErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for DismantleErrorCode {
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
            -1 => Some(Err(DismantleErrorCode::NotOwner)),
            -4 => Some(Err(DismantleErrorCode::Busy)),
            -7 => Some(Err(DismantleErrorCode::InvalidTarget)),
            -9 => Some(Err(DismantleErrorCode::NotInRange)),
            -12 => Some(Err(DismantleErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for DismantleErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            DismantleErrorCode::NotOwner => "you are not the owner of this creep",
            DismantleErrorCode::Busy => "the creep is still being spawned",
            DismantleErrorCode::InvalidTarget => "the target is not a valid structure object",
            DismantleErrorCode::NotInRange => "the target is too far away",
            DismantleErrorCode::NoBodypart => "there are no work body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for DismantleErrorCode {}

impl From<DismantleErrorCode> for ErrorCode {
    fn from(value: DismantleErrorCode) -> Self {
        // Safety: DismantleErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: DismantleErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Creep::generate_safe_mode](crate::Creep::generate_safe_mode).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.generateSafeMode).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L1049)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum GenerateSafeModeErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
}

impl FromReturnCode for GenerateSafeModeErrorCode {
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
            -1 => Some(Err(GenerateSafeModeErrorCode::NotOwner)),
            -4 => Some(Err(GenerateSafeModeErrorCode::Busy)),
            -6 => Some(Err(GenerateSafeModeErrorCode::NotEnoughResources)),
            -7 => Some(Err(GenerateSafeModeErrorCode::InvalidTarget)),
            -9 => Some(Err(GenerateSafeModeErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for GenerateSafeModeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            GenerateSafeModeErrorCode::NotOwner => "you are not the owner of this creep",
            GenerateSafeModeErrorCode::Busy => "the creep is still being spawned",
            GenerateSafeModeErrorCode::NotEnoughResources => {
                "the creep does not have enough ghodium"
            }
            GenerateSafeModeErrorCode::InvalidTarget => {
                "the target is not a valid controller object"
            }
            GenerateSafeModeErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for GenerateSafeModeErrorCode {}

impl From<GenerateSafeModeErrorCode> for ErrorCode {
    fn from(value: GenerateSafeModeErrorCode) -> Self {
        // Safety: GenerateSafeModeErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: GenerateSafeModeErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::harvest](crate::Creep::harvest).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.harvest).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L335)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum HarvestErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotFound = -5,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
}

impl FromReturnCode for HarvestErrorCode {
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
            -1 => Some(Err(HarvestErrorCode::NotOwner)),
            -4 => Some(Err(HarvestErrorCode::Busy)),
            -5 => Some(Err(HarvestErrorCode::NotFound)),
            -6 => Some(Err(HarvestErrorCode::NotEnoughResources)),
            -7 => Some(Err(HarvestErrorCode::InvalidTarget)),
            -9 => Some(Err(HarvestErrorCode::NotInRange)),
            -11 => Some(Err(HarvestErrorCode::Tired)),
            -12 => Some(Err(HarvestErrorCode::NoBodypart)),
            -14 => Some(Err(HarvestErrorCode::RclNotEnough)),
            _ => None,
        }
    }
}

impl fmt::Display for HarvestErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            HarvestErrorCode::NotOwner => "you are not the owner of this creep, or the room controller is owned or reserved by another player",
            HarvestErrorCode::Busy => "the creep is still being spawned",
            HarvestErrorCode::NotFound => "extractor not found. you must build an extractor structure to harvest minerals. learn more",
            HarvestErrorCode::NotEnoughResources => "the target does not contain any harvestable energy or mineral",
            HarvestErrorCode::InvalidTarget => "the target is not a valid source or mineral object",
            HarvestErrorCode::NotInRange => "the target is too far away",
            HarvestErrorCode::Tired => "the extractor or the deposit is still cooling down",
            HarvestErrorCode::NoBodypart => "there are no work body parts in this creep’s body",
            HarvestErrorCode::RclNotEnough => "room controller level insufficient to use the extractor",
        };

        write!(f, "{}", msg)
    }
}

impl Error for HarvestErrorCode {}

impl From<HarvestErrorCode> for ErrorCode {
    fn from(value: HarvestErrorCode) -> Self {
        // Safety: HarvestErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: HarvestErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::heal](crate::Creep::heal).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.heal).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L678)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepHealErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for CreepHealErrorCode {
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
            -1 => Some(Err(CreepHealErrorCode::NotOwner)),
            -4 => Some(Err(CreepHealErrorCode::Busy)),
            -7 => Some(Err(CreepHealErrorCode::InvalidTarget)),
            -9 => Some(Err(CreepHealErrorCode::NotInRange)),
            -12 => Some(Err(CreepHealErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepHealErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepHealErrorCode::NotOwner => "you are not the owner of this creep",
            CreepHealErrorCode::Busy => "the creep is still being spawned",
            CreepHealErrorCode::InvalidTarget => "the target is not a valid creep object",
            CreepHealErrorCode::NotInRange => "the target is too far away",
            CreepHealErrorCode::NoBodypart => "there are no heal body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepHealErrorCode {}

impl From<CreepHealErrorCode> for ErrorCode {
    fn from(value: CreepHealErrorCode) -> Self {
        // Safety: CreepHealErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepHealErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::move_direction](crate::Creep::move_direction).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.move).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L126)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepMoveDirectionErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
}

impl FromReturnCode for CreepMoveDirectionErrorCode {
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
            -1 => Some(Err(CreepMoveDirectionErrorCode::NotOwner)),
            -4 => Some(Err(CreepMoveDirectionErrorCode::Busy)),
            -10 => Some(Err(CreepMoveDirectionErrorCode::InvalidArgs)),
            -11 => Some(Err(CreepMoveDirectionErrorCode::Tired)),
            -12 => Some(Err(CreepMoveDirectionErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepMoveDirectionErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepMoveDirectionErrorCode::NotOwner => "you are not the owner of this creep",
            CreepMoveDirectionErrorCode::Busy => "the creep is still being spawned",
            CreepMoveDirectionErrorCode::InvalidArgs => "the provided direction is incorrect",
            CreepMoveDirectionErrorCode::Tired => "the fatigue indicator of the creep is non-zero",
            CreepMoveDirectionErrorCode::NoBodypart => {
                "there are no move body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepMoveDirectionErrorCode {}

impl From<CreepMoveDirectionErrorCode> for ErrorCode {
    fn from(value: CreepMoveDirectionErrorCode) -> Self {
        // Safety: CreepMoveDirectionErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepMoveDirectionErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::move_pulled_by](crate::Creep::move_pulled_by).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.move).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L126)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepMovePulledByErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotInRange = -9,
    InvalidArgs = -10,
}

impl FromReturnCode for CreepMovePulledByErrorCode {
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
            -1 => Some(Err(CreepMovePulledByErrorCode::NotOwner)),
            -4 => Some(Err(CreepMovePulledByErrorCode::Busy)),
            -9 => Some(Err(CreepMovePulledByErrorCode::NotInRange)),
            -10 => Some(Err(CreepMovePulledByErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepMovePulledByErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepMovePulledByErrorCode::NotOwner => "you are not the owner of this creep",
            CreepMovePulledByErrorCode::Busy => "the creep is still being spawned",
            CreepMovePulledByErrorCode::NotInRange => "the target creep is too far away",
            CreepMovePulledByErrorCode::InvalidArgs => "the provided direction is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepMovePulledByErrorCode {}

impl From<CreepMovePulledByErrorCode> for ErrorCode {
    fn from(value: CreepMovePulledByErrorCode) -> Self {
        // Safety: CreepMovePulledByErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepMovePulledByErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::move_by_path](crate::Creep::move_by_path).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.moveByPath).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L305)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepMoveByPathErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotFound = -5,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
}

impl FromReturnCode for CreepMoveByPathErrorCode {
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
            -1 => Some(Err(CreepMoveByPathErrorCode::NotOwner)),
            -4 => Some(Err(CreepMoveByPathErrorCode::Busy)),
            -5 => Some(Err(CreepMoveByPathErrorCode::NotFound)),
            -10 => Some(Err(CreepMoveByPathErrorCode::InvalidArgs)),
            -11 => Some(Err(CreepMoveByPathErrorCode::Tired)),
            -12 => Some(Err(CreepMoveByPathErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepMoveByPathErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepMoveByPathErrorCode::NotOwner => "you are not the owner of this creep",
            CreepMoveByPathErrorCode::Busy => "the creep is still being spawned",
            CreepMoveByPathErrorCode::NotFound => {
                "the specified path doesn't match the creep's location"
            }
            CreepMoveByPathErrorCode::InvalidArgs => "path is not a valid path array",
            CreepMoveByPathErrorCode::Tired => "the fatigue indicator of the creep is non-zero",
            CreepMoveByPathErrorCode::NoBodypart => {
                "there are no move body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepMoveByPathErrorCode {}

impl From<CreepMoveByPathErrorCode> for ErrorCode {
    fn from(value: CreepMoveByPathErrorCode) -> Self {
        // Safety: CreepMoveByPathErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepMoveByPathErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::move_to](crate::Creep#method.move_to).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.moveTo).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L158)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepMoveToErrorCode {
    NotOwner = -1,
    NoPath = -2,
    Busy = -4,
    NotFound = -5,
    InvalidTarget = -7,
    Tired = -11,
    NoBodypart = -12,
}

impl FromReturnCode for CreepMoveToErrorCode {
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
            -1 => Some(Err(CreepMoveToErrorCode::NotOwner)),
            -2 => Some(Err(CreepMoveToErrorCode::NoPath)),
            -4 => Some(Err(CreepMoveToErrorCode::Busy)),
            -5 => Some(Err(CreepMoveToErrorCode::NotFound)),
            -7 => Some(Err(CreepMoveToErrorCode::InvalidTarget)),
            -11 => Some(Err(CreepMoveToErrorCode::Tired)),
            -12 => Some(Err(CreepMoveToErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepMoveToErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepMoveToErrorCode::NotOwner => "you are not the owner of this creep",
            CreepMoveToErrorCode::NoPath => "no path to the target could be found",
            CreepMoveToErrorCode::Busy => "the creep is still being spawned",
            CreepMoveToErrorCode::NotFound => "the creep has no memorized path to reuse",
            CreepMoveToErrorCode::InvalidTarget => "the target provided is invalid",
            CreepMoveToErrorCode::Tired => "the fatigue indicator of the creep is non-zero",
            CreepMoveToErrorCode::NoBodypart => "there are no move body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepMoveToErrorCode {}

impl From<CreepMoveToErrorCode> for ErrorCode {
    fn from(value: CreepMoveToErrorCode) -> Self {
        // Safety: CreepMoveToErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepMoveToErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::pull](crate::Creep::pull).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.pull).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L1093)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum PullErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
}

impl FromReturnCode for PullErrorCode {
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
            -1 => Some(Err(PullErrorCode::NotOwner)),
            -4 => Some(Err(PullErrorCode::Busy)),
            -7 => Some(Err(PullErrorCode::InvalidTarget)),
            -9 => Some(Err(PullErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for PullErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            PullErrorCode::NotOwner => "you are not the owner of this creep",
            PullErrorCode::Busy => "the creep is still being spawned",
            PullErrorCode::InvalidTarget => "the target provided is invalid",
            PullErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for PullErrorCode {}

impl From<PullErrorCode> for ErrorCode {
    fn from(value: PullErrorCode) -> Self {
        // Safety: PullErrorCode is repr(i8), so we can cast it to get the discriminant
        // value, which will match the raw return code value that ErrorCode expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: PullErrorCode discriminants are always error code values, and thus
        // the Result returned here will always be an `Err` variant, so we can always
        // extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::ranged_attack](crate::Creep::ranged_attack).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.rangedAttack).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L626)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RangedAttackErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for RangedAttackErrorCode {
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
            -1 => Some(Err(RangedAttackErrorCode::NotOwner)),
            -4 => Some(Err(RangedAttackErrorCode::Busy)),
            -7 => Some(Err(RangedAttackErrorCode::InvalidTarget)),
            -9 => Some(Err(RangedAttackErrorCode::NotInRange)),
            -12 => Some(Err(RangedAttackErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for RangedAttackErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RangedAttackErrorCode::NotOwner => "you are not the owner of this creep",
            RangedAttackErrorCode::Busy => "the creep is still being spawned",
            RangedAttackErrorCode::InvalidTarget => "the target is not a valid attackable object",
            RangedAttackErrorCode::NotInRange => "the target is too far away",
            RangedAttackErrorCode::NoBodypart => {
                "there are no ranged_attack body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RangedAttackErrorCode {}

impl From<RangedAttackErrorCode> for ErrorCode {
    fn from(value: RangedAttackErrorCode) -> Self {
        // Safety: RangedAttackErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RangedAttackErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::ranged_heal](crate::Creep::ranged_heal).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.rangedHeal).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L706)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RangedHealErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for RangedHealErrorCode {
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
            -1 => Some(Err(RangedHealErrorCode::NotOwner)),
            -4 => Some(Err(RangedHealErrorCode::Busy)),
            -7 => Some(Err(RangedHealErrorCode::InvalidTarget)),
            -9 => Some(Err(RangedHealErrorCode::NotInRange)),
            -12 => Some(Err(RangedHealErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for RangedHealErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RangedHealErrorCode::NotOwner => "you are not the owner of this creep",
            RangedHealErrorCode::Busy => "the creep is still being spawned",
            RangedHealErrorCode::InvalidTarget => "the target is not a valid creep object",
            RangedHealErrorCode::NotInRange => "the target is too far away",
            RangedHealErrorCode::NoBodypart => "there are no heal body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for RangedHealErrorCode {}

impl From<RangedHealErrorCode> for ErrorCode {
    fn from(value: RangedHealErrorCode) -> Self {
        // Safety: RangedHealErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RangedHealErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Creep::ranged_mass_attack](crate::Creep::ranged_mass_attack).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.rangedMassAttack).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L658)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum RangedMassAttackErrorCode {
    NotOwner = -1,
    Busy = -4,
    NoBodypart = -12,
}

impl FromReturnCode for RangedMassAttackErrorCode {
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
            -1 => Some(Err(RangedMassAttackErrorCode::NotOwner)),
            -4 => Some(Err(RangedMassAttackErrorCode::Busy)),
            -12 => Some(Err(RangedMassAttackErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for RangedMassAttackErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            RangedMassAttackErrorCode::NotOwner => "you are not the owner of this creep",
            RangedMassAttackErrorCode::Busy => "the creep is still being spawned",
            RangedMassAttackErrorCode::NoBodypart => {
                "there are no ranged_attack body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for RangedMassAttackErrorCode {}

impl From<RangedMassAttackErrorCode> for ErrorCode {
    fn from(value: RangedMassAttackErrorCode) -> Self {
        // Safety: RangedMassAttackErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: RangedMassAttackErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::repair](crate::Creep::repair).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.repair).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L734)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepRepairErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for CreepRepairErrorCode {
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
            -1 => Some(Err(CreepRepairErrorCode::NotOwner)),
            -4 => Some(Err(CreepRepairErrorCode::Busy)),
            -6 => Some(Err(CreepRepairErrorCode::NotEnoughResources)),
            -7 => Some(Err(CreepRepairErrorCode::InvalidTarget)),
            -9 => Some(Err(CreepRepairErrorCode::NotInRange)),
            -12 => Some(Err(CreepRepairErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepRepairErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepRepairErrorCode::NotOwner => "you are not the owner of this creep",
            CreepRepairErrorCode::Busy => "the creep is still being spawned",
            CreepRepairErrorCode::NotEnoughResources => "the creep does not carry any energy",
            CreepRepairErrorCode::InvalidTarget => "the target is not a valid structure object",
            CreepRepairErrorCode::NotInRange => "the target is too far away",
            CreepRepairErrorCode::NoBodypart => "there are no work body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepRepairErrorCode {}

impl From<CreepRepairErrorCode> for ErrorCode {
    fn from(value: CreepRepairErrorCode) -> Self {
        // Safety: CreepRepairErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: CreepRepairErrorCode discriminants are always error code values, and
        // thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Creep::reserve_controller](crate::Creep::reserve_controller).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.reserveController).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L955)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ReserveControllerErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for ReserveControllerErrorCode {
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
            -1 => Some(Err(ReserveControllerErrorCode::NotOwner)),
            -4 => Some(Err(ReserveControllerErrorCode::Busy)),
            -7 => Some(Err(ReserveControllerErrorCode::InvalidTarget)),
            -9 => Some(Err(ReserveControllerErrorCode::NotInRange)),
            -12 => Some(Err(ReserveControllerErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for ReserveControllerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ReserveControllerErrorCode::NotOwner => "you are not the owner of this creep",
            ReserveControllerErrorCode::Busy => "the creep is still being spawned",
            ReserveControllerErrorCode::InvalidTarget => {
                "the target is not a valid neutral controller object"
            }
            ReserveControllerErrorCode::NotInRange => "the target is too far away",
            ReserveControllerErrorCode::NoBodypart => {
                "there are no claim body parts in this creep’s body"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for ReserveControllerErrorCode {}

impl From<ReserveControllerErrorCode> for ErrorCode {
    fn from(value: ReserveControllerErrorCode) -> Self {
        // Safety: ReserveControllerErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: ReserveControllerErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by [Creep::sign_controller](crate::Creep::sign_controller).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.signController).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L1072)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum SignControllerErrorCode {
    Busy = -4,
    InvalidTarget = -7,
    NotInRange = -9,
}

impl FromReturnCode for SignControllerErrorCode {
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
            -4 => Some(Err(SignControllerErrorCode::Busy)),
            -7 => Some(Err(SignControllerErrorCode::InvalidTarget)),
            -9 => Some(Err(SignControllerErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for SignControllerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            SignControllerErrorCode::Busy => "the creep is still being spawned",
            SignControllerErrorCode::InvalidTarget => "the target is not a valid controller object",
            SignControllerErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for SignControllerErrorCode {}

impl From<SignControllerErrorCode> for ErrorCode {
    fn from(value: SignControllerErrorCode) -> Self {
        // Safety: SignControllerErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: SignControllerErrorCode discriminants are always error code values,
        // and thus the Result returned here will always be an `Err` variant, so we can
        // always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}

/// Error codes used by
/// [Creep::upgrade_controller](crate::Creep::upgrade_controller).
///
/// [Screeps API Docs](https://docs.screeps.com/api/#Creep.upgradeController).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L919)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum UpgradeControllerErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    NotInRange = -9,
    NoBodypart = -12,
}

impl FromReturnCode for UpgradeControllerErrorCode {
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
            -1 => Some(Err(UpgradeControllerErrorCode::NotOwner)),
            -4 => Some(Err(UpgradeControllerErrorCode::Busy)),
            -6 => Some(Err(UpgradeControllerErrorCode::NotEnoughResources)),
            -7 => Some(Err(UpgradeControllerErrorCode::InvalidTarget)),
            -9 => Some(Err(UpgradeControllerErrorCode::NotInRange)),
            -12 => Some(Err(UpgradeControllerErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for UpgradeControllerErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            UpgradeControllerErrorCode::NotOwner => "you are not the owner of this creep or the target controller",
            UpgradeControllerErrorCode::Busy => "the creep is still being spawned",
            UpgradeControllerErrorCode::NotEnoughResources => "the creep does not have any carried energy",
            UpgradeControllerErrorCode::InvalidTarget => "the target is not a valid controller object, or the controller upgrading is blocked",
            UpgradeControllerErrorCode::NotInRange => "the target is too far away",
            UpgradeControllerErrorCode::NoBodypart => "there are no work body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for UpgradeControllerErrorCode {}

impl From<UpgradeControllerErrorCode> for ErrorCode {
    fn from(value: UpgradeControllerErrorCode) -> Self {
        // Safety: UpgradeControllerErrorCode is repr(i8), so we can cast it to get the
        // discriminant value, which will match the raw return code value that ErrorCode
        // expects.   Ref: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.coercion.intro
        // Safety: UpgradeControllerErrorCode discriminants are always error code
        // values, and thus the Result returned here will always be an `Err` variant, so
        // we can always extract the error without panicking
        Self::result_from_i8(value as i8).unwrap_err()
    }
}
