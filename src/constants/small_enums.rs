//! Various constants translated as small enums.
use std::{borrow::Cow, fmt, str::FromStr};

use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use parse_display::FromStr;
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    find,
    numbers::{TERRAIN_MASK_SWAMP, TERRAIN_MASK_WALL},
};
use crate::macros::*;

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(i16)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
}

impl ReturnCode {
    /// Turns this return code into a result.
    ///
    /// `ReturnCode::Ok` is turned into `Result::Ok`, all other codes are turned
    /// into `Result::Err(code)`
    #[inline]
    pub fn as_result(self) -> Result<(), Self> {
        match self {
            ReturnCode::Ok => Ok(()),
            other => Err(other),
        }
    }
}

js_deserializable!(ReturnCode);

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum Direction {
    Top = 1,
    TopRight = 2,
    Right = 3,
    BottomRight = 4,
    Bottom = 5,
    BottomLeft = 6,
    Left = 7,
    TopLeft = 8,
}

js_deserializable!(Direction);

impl ::std::ops::Neg for Direction {
    type Output = Direction;

    /// Negates this direction. Top goes to Bottom, TopRight goes to BottomLeft,
    /// etc.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::Direction::*;
    ///
    /// assert_eq!(-Top, Bottom);
    /// assert_eq!(-BottomRight, TopLeft);
    /// assert_eq!(-Left, Right);
    /// ```
    #[inline]
    fn neg(self) -> Direction {
        use Direction::*;

        match self {
            Top => Bottom,
            TopRight => BottomLeft,
            Right => Left,
            BottomRight => TopLeft,
            Bottom => Top,
            BottomLeft => TopRight,
            Left => Right,
            TopLeft => BottomRight,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Direction::Top => "↑",
            Direction::TopRight => "↗",
            Direction::Right => "→",
            Direction::BottomRight => "↘",
            Direction::Bottom => "↓",
            Direction::BottomLeft => "↙",
            Direction::Left => "←",
            Direction::TopLeft => "↖",
        };
        f.write_str(ch)
    }
}

/// Type used for when the game returns a direction to an exit.
///
/// Restricted more than `Direction` in that it can't be diagonal. Used as the
/// result of [`Room::find_exit_to`].
///
/// Can be converted to both [`find::Exit`] for immediate use of [`Room::find`]
/// and [`Direction`].
///
/// [`Room::find`]: crate::objects::Room::find
/// [`Room::find_exit_to`]: crate::objects::Room::find_exit_to
#[derive(
    Copy, Clone, Debug, FromPrimitive, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash,
)]
#[repr(u8)]
pub enum ExitDirection {
    Top = Direction::Top as u8,
    Right = Direction::Right as u8,
    Bottom = Direction::Bottom as u8,
    Left = Direction::Left as u8,
}

impl From<ExitDirection> for find::Exit {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => find::Exit::Top,
            ExitDirection::Right => find::Exit::Right,
            ExitDirection::Bottom => find::Exit::Bottom,
            ExitDirection::Left => find::Exit::Left,
        }
    }
}

impl From<ExitDirection> for Direction {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => Direction::Top,
            ExitDirection::Right => Direction::Right,
            ExitDirection::Bottom => Direction::Bottom,
            ExitDirection::Left => Direction::Left,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(u8)]
pub enum Color {
    Red = 1,
    Purple = 2,
    Blue = 3,
    Cyan = 4,
    Green = 5,
    Yellow = 6,
    Orange = 7,
    Brown = 8,
    Grey = 9,
    White = 10,
}

js_deserializable!(Color);

/// Terrain constant.
///
/// This constant is in a unique position of being represented both by strings
/// and by integers in various parts of the API.
///
/// *Note:* This constant's `TryFrom<Value>` and `Deserialize` implementations
/// _only work with the integer constants_. If you're ever consuming strings
/// such as `"plain"`, `"swamp"`, `"wall"`, you can use the
/// `__terrain_str_to_num` JavaScript function, [`FromStr`][std::str::FromStr]
/// or [`Look::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Deserialize_repr,
    Serialize_repr,
    FromPrimitive,
    FromStr,
)]
#[repr(u8)]
#[display(style = "snake_case")]
pub enum Terrain {
    Plain = 0,
    Wall = TERRAIN_MASK_WALL,
    Swamp = TERRAIN_MASK_SWAMP,
}

impl Terrain {
    /// Helper function for deserializing from a string rather than from an
    /// integer.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &r#""plain", "wall" or "swamp""#)
        })
    }
}

js_deserializable!(Terrain);

/// Creep part types.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__part_num_to_str` and
/// `__part_str_to_num` JavaScript functions, [`FromStr`][std::str::FromStr] or
/// [`Part::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
#[display(style = "snake_case")]
pub enum Part {
    Move = 0,
    Work = 1,
    Carry = 2,
    Attack = 3,
    RangedAttack = 4,
    Tough = 5,
    Heal = 6,
    Claim = 7,
}

impl Part {
    /// Translates the `BODYPART_COST` constant.
    #[inline]
    pub fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            Part::Claim => 600,
        }
    }

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in BODYPARTS_ALL",
            )
        })
    }
}

js_deserializable!(Part);

/// Translates the `DENSITY_*` constants.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Density {
    Low = 1,
    Moderate = 2,
    High = 3,
    Ultra = 4,
}

js_deserializable!(Density);

impl Density {
    /// Translates the `MINERAL_DENSITY` constant.
    #[inline]
    pub fn amount(self) -> u32 {
        match self {
            Density::Low => 15_000,
            Density::Moderate => 35_000,
            Density::High => 70_000,
            Density::Ultra => 100_000,
        }
    }

    /// Translates the `MINERAL_DENSITY_PROBABILITY` constant.
    ///
    /// These are values intended for subsequent percentage checks
    /// in the order `Low` -> `Medium` -> `High` -> `Ultra`. Use the
    /// [`Density::iter_values`] iterator to iterate in this order.
    #[inline]
    pub fn probability(self) -> f32 {
        match self {
            Density::Low => 0.1,
            Density::Moderate => 0.5,
            Density::High => 0.9,
            Density::Ultra => 1.0,
        }
    }

    pub fn iter_values() -> impl Iterator<Item = Density> {
        <Density as enum_iterator::IntoEnumIterator>::into_enum_iter()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PowerClass {
    Operator,
}
