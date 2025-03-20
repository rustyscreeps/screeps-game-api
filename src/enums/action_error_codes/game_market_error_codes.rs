use std::fmt;
use std::error::Error;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::FromReturnCode;

/// Error codes used by [game::market::cancel_order](crate::game::market::cancel_order).
///
/// Screeps API Docs: [Game.market.cancelOrder](https://docs.screeps.com/api/#Game.market.cancelOrder).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/market.js#L100)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum MarketCancelOrderErrorCode {
    InvalidArgs = -10,
}

impl FromReturnCode for MarketCancelOrderErrorCode {
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
            -10 => Some(Err(MarketCancelOrderErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for MarketCancelOrderErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            MarketCancelOrderErrorCode::InvalidArgs => "the order id is not valid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for MarketCancelOrderErrorCode {}

/// Error codes used by [game::market::change_order_price](crate::game::market::change_order_price).
///
/// Screeps API Docs: [Game.market.changeOrderPrice](https://docs.screeps.com/api/#Game.market.changeOrderPrice).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/market.js#L155)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ChangeOrderPriceErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    InvalidArgs = -10,
}

impl FromReturnCode for ChangeOrderPriceErrorCode {
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
            -1 => Some(Err(ChangeOrderPriceErrorCode::NotOwner)),
            -6 => Some(Err(ChangeOrderPriceErrorCode::NotEnoughResources)),
            -10 => Some(Err(ChangeOrderPriceErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for ChangeOrderPriceErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ChangeOrderPriceErrorCode::NotOwner => "you are not the owner of the room's terminal or there is no terminal",
            ChangeOrderPriceErrorCode::NotEnoughResources => "you don't have enough credits to pay a fee",
            ChangeOrderPriceErrorCode::InvalidArgs => "the arguments provided are invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for ChangeOrderPriceErrorCode {}

/// Error codes used by [game::market::create_order](crate::game::market::create_order).
///
/// Screeps API Docs: [Game.market.createOrder](https://docs.screeps.com/api/#Game.market.createOrder).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/market.js#L68)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum CreateOrderErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    Full = -8,
    InvalidArgs = -10,
}

impl FromReturnCode for CreateOrderErrorCode {
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
            -1 => Some(Err(CreateOrderErrorCode::NotOwner)),
            -6 => Some(Err(CreateOrderErrorCode::NotEnoughResources)),
            -8 => Some(Err(CreateOrderErrorCode::Full)),
            -10 => Some(Err(CreateOrderErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            CreateOrderErrorCode::NotOwner => "you are not the owner of the room's terminal or there is no terminal",
            CreateOrderErrorCode::NotEnoughResources => "you don't have enough credits to pay a fee",
            CreateOrderErrorCode::Full => "you cannot create more than 50 orders",
            CreateOrderErrorCode::InvalidArgs => "the arguments provided are invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for CreateOrderErrorCode {}

/// Error codes used by [game::market::deal](crate::game::market::deal).
///
/// Screeps API Docs: [Game.market.deal](https://docs.screeps.com/api/#Game.market.deal).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/market.js#L108)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum DealErrorCode {
    NotOwner = -1,
    NotEnoughResources = -6,
    Full = -8,
    InvalidArgs = -10,
    Tired = -11,
}

impl FromReturnCode for DealErrorCode {
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
            -1 => Some(Err(DealErrorCode::NotOwner)),
            -6 => Some(Err(DealErrorCode::NotEnoughResources)),
            -8 => Some(Err(DealErrorCode::Full)),
            -10 => Some(Err(DealErrorCode::InvalidArgs)),
            -11 => Some(Err(DealErrorCode::Tired)),
            _ => None,
        }
    }
}

impl fmt::Display for DealErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            DealErrorCode::NotOwner => "you don't have a terminal in the target room",
            DealErrorCode::NotEnoughResources => "you don't have enough credits or resource units",
            DealErrorCode::Full => "you cannot execute more than 10 deals during one tick",
            DealErrorCode::InvalidArgs => "the arguments provided are invalid",
            DealErrorCode::Tired => "the target terminal is still cooling down",
        };

        write!(f, "{}", msg)
    }
}

impl Error for DealErrorCode {}

/// Error codes used by [game::market::extend_order](crate::game::market::extend_order).
///
/// Screeps API Docs: [Game.market.extendOrder](https://docs.screeps.com/api/#Game.market.extendOrder).
///
/// [Screeps Engine Source Code](https://github.com/screeps/engine/blob/97c9d12385fed686655c13b09f5f2457dd83a2bf/src/game/market.js#L174)
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ExtendOrderErrorCode {
    NotEnoughResources = -6,
    InvalidArgs = -10,
}

impl FromReturnCode for ExtendOrderErrorCode {
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
            -6 => Some(Err(ExtendOrderErrorCode::NotEnoughResources)),
            -10 => Some(Err(ExtendOrderErrorCode::InvalidArgs)),
            _ => None,
        }
    }
}

impl fmt::Display for ExtendOrderErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: &'static str = match self {
            ExtendOrderErrorCode::NotEnoughResources => "you don't have enough credits to pay a fee",
            ExtendOrderErrorCode::InvalidArgs => "the arguments provided are invalid",
        };

        write!(f, "{}", msg)
    }
}

impl Error for ExtendOrderErrorCode {}