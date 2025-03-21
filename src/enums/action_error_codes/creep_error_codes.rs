use std::{error::Error, fmt};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by [Creep::attack](crate::Creep::attack).
///
/// Screeps API Docs: [Creep.attack](https://docs.screeps.com/api/#Creep.attack).
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

/// Error codes used by
/// [Creep::attack_controller](crate::Creep::attack_controller).
///
/// Screeps API Docs: [Creep.attackController](https://docs.screeps.com/api/#Creep.attackController).
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

/// Error codes used by [Creep::build](crate::Creep::build).
///
/// Screeps API Docs: [Creep.build](https://docs.screeps.com/api/#Creep.build).
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

/// Error codes used by [Creep::cancel_order](crate::Creep::cancel_order).
///
/// Screeps API Docs: [Creep.cancelOrder](https://docs.screeps.com/api/#Creep.cancelOrder).
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

/// Error codes used by
/// [Creep::claim_controller](crate::Creep::claim_controller).
///
/// Screeps API Docs: [Creep.claimController](https://docs.screeps.com/api/#Creep.claimController).
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

/// Error codes used by [Creep::dismantle](crate::Creep::dismantle).
///
/// Screeps API Docs: [Creep.dismantle](https://docs.screeps.com/api/#Creep.dismantle).
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

/// Error codes used by [Creep::drop](crate::Creep::drop).
///
/// Screeps API Docs: [Creep.drop](https://docs.screeps.com/api/#Creep.drop).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L404)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepDropErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidArgs = -10,
}

impl FromReturnCode for CreepDropErrorCode {
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
            -1 => Some(Err(CreepDropErrorCode::NotOwner)),
            -4 => Some(Err(CreepDropErrorCode::Busy)),
            -6 => Some(Err(CreepDropErrorCode::NotEnoughResources)),
            -10 => Some(Err(CreepDropErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepDropErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepDropErrorCode::NotOwner => "you are not the owner of this creep",
            CreepDropErrorCode::Busy => "the creep is still being spawned",
            CreepDropErrorCode::NotEnoughResources => {
                "the creep does not have the given amount of resources"
            }
            CreepDropErrorCode::InvalidArgs => {
                "the resourcetype is not a valid resource_* constants"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepDropErrorCode {}

/// Error codes used by
/// [Creep::generate_safe_mode](crate::Creep::generate_safe_mode).
///
/// Screeps API Docs: [Creep.generateSafeMode](https://docs.screeps.com/api/#Creep.generateSafeMode).
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

/// Error codes used by [Creep::harvest](crate::Creep::harvest).
///
/// Screeps API Docs: [Creep.harvest](https://docs.screeps.com/api/#Creep.harvest).
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

/// Error codes used by [Creep::heal](crate::Creep::heal).
///
/// Screeps API Docs: [Creep.heal](https://docs.screeps.com/api/#Creep.heal).
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

/// Error codes used by [Creep::move](crate::Creep::move).
///
/// Screeps API Docs: [Creep.move](https://docs.screeps.com/api/#Creep.move).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L126)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepMoveErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
}

impl FromReturnCode for CreepMoveErrorCode {
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
            -1 => Some(Err(CreepMoveErrorCode::NotOwner)),
            -4 => Some(Err(CreepMoveErrorCode::Busy)),
            -9 => Some(Err(CreepMoveErrorCode::NotInRange)),
            -10 => Some(Err(CreepMoveErrorCode::InvalidArgs)),
            -11 => Some(Err(CreepMoveErrorCode::Tired)),
            -12 => Some(Err(CreepMoveErrorCode::NoBodypart)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepMoveErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepMoveErrorCode::NotOwner => "you are not the owner of this creep",
            CreepMoveErrorCode::Busy => "the creep is still being spawned",
            CreepMoveErrorCode::NotInRange => "the target creep is too far away",
            CreepMoveErrorCode::InvalidArgs => "the provided direction is incorrect",
            CreepMoveErrorCode::Tired => "the fatigue indicator of the creep is non-zero",
            CreepMoveErrorCode::NoBodypart => "there are no move body parts in this creep’s body",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepMoveErrorCode {}

/// Error codes used by [Creep::move_by_path](crate::Creep::move_by_path).
///
/// Screeps API Docs: [Creep.moveByPath](https://docs.screeps.com/api/#Creep.moveByPath).
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

/// Error codes used by [Creep::move_to](crate::Creep::move_to).
///
/// Screeps API Docs: [Creep.moveTo](https://docs.screeps.com/api/#Creep.moveTo).
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

/// Error codes used by
/// [Creep::notify_when_attacked](crate::Creep::notify_when_attacked).
///
/// Screeps API Docs: [Creep.notifyWhenAttacked](https://docs.screeps.com/api/#Creep.notifyWhenAttacked).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L988)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepNotifyWhenAttackedErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidArgs = -10,
}

impl FromReturnCode for CreepNotifyWhenAttackedErrorCode {
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
            -1 => Some(Err(CreepNotifyWhenAttackedErrorCode::NotOwner)),
            -4 => Some(Err(CreepNotifyWhenAttackedErrorCode::Busy)),
            -10 => Some(Err(CreepNotifyWhenAttackedErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepNotifyWhenAttackedErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepNotifyWhenAttackedErrorCode::NotOwner => "you are not the owner of this creep",
            CreepNotifyWhenAttackedErrorCode::Busy => "the creep is still being spawned",
            CreepNotifyWhenAttackedErrorCode::InvalidArgs => {
                "enable argument is not a boolean value"
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepNotifyWhenAttackedErrorCode {}

/// Error codes used by [Creep::pickup](crate::Creep::pickup).
///
/// Screeps API Docs: [Creep.pickup](https://docs.screeps.com/api/#Creep.pickup).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L566)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepPickupErrorCode {
    NotOwner = -1,
    Busy = -4,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
}

impl FromReturnCode for CreepPickupErrorCode {
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
            -1 => Some(Err(CreepPickupErrorCode::NotOwner)),
            -4 => Some(Err(CreepPickupErrorCode::Busy)),
            -7 => Some(Err(CreepPickupErrorCode::InvalidTarget)),
            -8 => Some(Err(CreepPickupErrorCode::Full)),
            -9 => Some(Err(CreepPickupErrorCode::NotInRange)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepPickupErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepPickupErrorCode::NotOwner => "you are not the owner of this creep",
            CreepPickupErrorCode::Busy => "the creep is still being spawned",
            CreepPickupErrorCode::InvalidTarget => "the target is not a valid object to pick up",
            CreepPickupErrorCode::Full => "the creep cannot receive any more resource",
            CreepPickupErrorCode::NotInRange => "the target is too far away",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepPickupErrorCode {}

/// Error codes used by [Creep::pull](crate::Creep::pull).
///
/// Screeps API Docs: [Creep.pull](https://docs.screeps.com/api/#Creep.pull).
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

/// Error codes used by [Creep::ranged_attack](crate::Creep::ranged_attack).
///
/// Screeps API Docs: [Creep.rangedAttack](https://docs.screeps.com/api/#Creep.rangedAttack).
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

/// Error codes used by [Creep::ranged_heal](crate::Creep::ranged_heal).
///
/// Screeps API Docs: [Creep.rangedHeal](https://docs.screeps.com/api/#Creep.rangedHeal).
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

/// Error codes used by
/// [Creep::ranged_mass_attack](crate::Creep::ranged_mass_attack).
///
/// Screeps API Docs: [Creep.rangedMassAttack](https://docs.screeps.com/api/#Creep.rangedMassAttack).
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

/// Error codes used by [Creep::repair](crate::Creep::repair).
///
/// Screeps API Docs: [Creep.repair](https://docs.screeps.com/api/#Creep.repair).
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

/// Error codes used by
/// [Creep::reserve_controller](crate::Creep::reserve_controller).
///
/// Screeps API Docs: [Creep.reserveController](https://docs.screeps.com/api/#Creep.reserveController).
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

/// Error codes used by [Creep::say](crate::Creep::say).
///
/// Screeps API Docs: [Creep.say](https://docs.screeps.com/api/#Creep.say).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L826)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepSayErrorCode {
    NotOwner = -1,
    Busy = -4,
}

impl FromReturnCode for CreepSayErrorCode {
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
            -1 => Some(Err(CreepSayErrorCode::NotOwner)),
            -4 => Some(Err(CreepSayErrorCode::Busy)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepSayErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepSayErrorCode::NotOwner => "you are not the owner of this creep",
            CreepSayErrorCode::Busy => "the creep is still being spawned",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepSayErrorCode {}

/// Error codes used by [Creep::sign_controller](crate::Creep::sign_controller).
///
/// Screeps API Docs: [Creep.signController](https://docs.screeps.com/api/#Creep.signController).
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

/// Error codes used by [Creep::suicide](crate::Creep::suicide).
///
/// Screeps API Docs: [Creep.suicide](https://docs.screeps.com/api/#Creep.suicide).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L813)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepSuicideErrorCode {
    NotOwner = -1,
    Busy = -4,
}

impl FromReturnCode for CreepSuicideErrorCode {
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
            -1 => Some(Err(CreepSuicideErrorCode::NotOwner)),
            -4 => Some(Err(CreepSuicideErrorCode::Busy)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepSuicideErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepSuicideErrorCode::NotOwner => "you are not the owner of this creep",
            CreepSuicideErrorCode::Busy => "the creep is still being spawned",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepSuicideErrorCode {}

/// Error codes used by [Creep::transfer](crate::Creep::transfer).
///
/// Screeps API Docs: [Creep.transfer](https://docs.screeps.com/api/#Creep.transfer).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L428)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepTransferErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
}

impl FromReturnCode for CreepTransferErrorCode {
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
            -1 => Some(Err(CreepTransferErrorCode::NotOwner)),
            -4 => Some(Err(CreepTransferErrorCode::Busy)),
            -6 => Some(Err(CreepTransferErrorCode::NotEnoughResources)),
            -7 => Some(Err(CreepTransferErrorCode::InvalidTarget)),
            -8 => Some(Err(CreepTransferErrorCode::Full)),
            -9 => Some(Err(CreepTransferErrorCode::NotInRange)),
            -10 => Some(Err(CreepTransferErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepTransferErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepTransferErrorCode::NotOwner => "you are not the owner of this creep",
            CreepTransferErrorCode::Busy => "the creep is still being spawned",
            CreepTransferErrorCode::NotEnoughResources => "the creep does not have the given amount of resources",
            CreepTransferErrorCode::InvalidTarget => "the target is not a valid object which can contain the specified resource",
            CreepTransferErrorCode::Full => "the target cannot receive any more resources",
            CreepTransferErrorCode::NotInRange => "the target is too far away",
            CreepTransferErrorCode::InvalidArgs => "the resourcetype is not one of the resource_* constants, or the amount is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepTransferErrorCode {}

/// Error codes used by
/// [Creep::upgrade_controller](crate::Creep::upgrade_controller).
///
/// Screeps API Docs: [Creep.upgradeController](https://docs.screeps.com/api/#Creep.upgradeController).
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

/// Error codes used by [Creep::withdraw](crate::Creep::withdraw).
///
/// Screeps API Docs: [Creep.withdraw](https://docs.screeps.com/api/#Creep.withdraw).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/creeps.js#L493)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreepWithdrawErrorCode {
    NotOwner = -1,
    Busy = -4,
    NotEnoughResources = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
}

impl FromReturnCode for CreepWithdrawErrorCode {
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
            -1 => Some(Err(CreepWithdrawErrorCode::NotOwner)),
            -4 => Some(Err(CreepWithdrawErrorCode::Busy)),
            -6 => Some(Err(CreepWithdrawErrorCode::NotEnoughResources)),
            -7 => Some(Err(CreepWithdrawErrorCode::InvalidTarget)),
            -8 => Some(Err(CreepWithdrawErrorCode::Full)),
            -9 => Some(Err(CreepWithdrawErrorCode::NotInRange)),
            -10 => Some(Err(CreepWithdrawErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for CreepWithdrawErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreepWithdrawErrorCode::NotOwner => "you are not the owner of this creep, or there is a hostile rampart on top of the target",
            CreepWithdrawErrorCode::Busy => "the creep is still being spawned",
            CreepWithdrawErrorCode::NotEnoughResources => "the target does not have the given amount of resources",
            CreepWithdrawErrorCode::InvalidTarget => "the target is not a valid object which can contain the specified resource",
            CreepWithdrawErrorCode::Full => "the creep's carry is full",
            CreepWithdrawErrorCode::NotInRange => "the target is too far away",
            CreepWithdrawErrorCode::InvalidArgs => "the resourcetype is not one of the resource_* constants, or the amount is incorrect",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreepWithdrawErrorCode {}
