//! Room position type and related operations and traits.
//!
//! This is a reimplementation/translation of the `RoomPosition` code originally
//! written in JavaScript. All RoomPosition to RoomPosition operations in this
//! file stay within Rust.
use std::fmt;

use super::{RoomName, HALF_WORLD_SIZE};

mod extra_math;
mod game_math;
mod game_methods;
mod pair_utils;
mod world_utils;

/// Represents a position in a particular room in Screeps.
///
/// **Note:** This is analagous to the `RoomPosition` JavaScript type.
///
/// We've renamed this type to `Position` in `screeps-game-api` to reflect the
/// fact that it's implemented entirely as a local type, and does represent a
/// position located within an entire shard, not only within a single room.
///
/// This should be a very efficient type to use in most if not all situations.
/// It's represented by a single `u32`, all math operations are implemented in
/// pure-Rust code, and uploading to / downloading from JavaScript only requires
/// transferring a single `i32`.
///
/// # Using Position
///
/// You can retrieve a `Position` by getting the position of a game object using
/// [`HasPosition::pos`], or by creating one from coordinates with
/// [`Position::new`].
///
/// You can use any of the math methods available on this page to manipulate
/// [`Position`], and you can pass it to any game methods expecting a position
/// or something with a position.
///
/// # Serialization
///
/// `Position` implements both `serde::Serialize` and
/// `serde::Deserialize`.
///
/// When serializing, it will use the format `{roomName: String, x: u32, y:
/// u32}` in "human readable" formats like JSON, and will serialize as a single
/// `i32` in "non-human readable" formats like [`bincode`].
///
/// You can also pass `Position` into JavaScript using the `js!{}`
/// macro provided by `stdweb`, or helper methods using the same code like
/// [`MemoryReference::set`][crate::memory::MemoryReference::set]. It will be
/// serialized the same as in JSON, as an object with `roomName`, `x` and `y`
/// properties.
///
/// *Note:* serializing using `js!{}` or `MemoryReference::set` will _not_
/// create a JavaScript `RoomPosition`, only something with the same properties.
///
/// If you need a reference to a `RoomPosition` in JavaScript to use manually,
/// you have two options:
///
/// - Use `.remote()` to get a `stdweb::Reference`, and then use that reference
///   in JavaScript
///
/// - Convert the room position to an integer with [`Position::packed_repr`],
///   send that to JS, and use the `pos_from_packed` JavaScript function
///   provided by this library:
///
///   ```no_run
///   use stdweb::js;
///   use screeps::Position;
///
///   let pos = Position::new(20, 21, "E5N6".parse().unwrap());
///   let result = js! {
///       let pos = pos_from_packed(@{pos.packed_repr()});
///       pos.roomName
///   };
///   ```
///
/// [`bincode`]: https://github.com/servo/bincode
/// [`HasPosition::pos`]: crate::HasPosition::pos
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Position {
    /// A bit-packed integer, containing, from highest-order to lowest:
    ///
    /// - 1 byte: (room_x) + 128
    /// - 1 byte: (room_y) + 128
    /// - 1 byte: x
    /// - 1 byte: y
    ///
    /// For `Wxx` rooms, `room_x = -xx - 1`. For `Exx` rooms, `room_x = xx`.
    ///
    /// For `Nyy` rooms, `room_y = -yy - 1`. For `Syy` rooms, `room_y = yy`.
    ///
    /// This is the same representation used in the Screeps server, allowing for
    /// easy translation. Besides the method names and display representation,
    /// this is the one part of RoomPosition copied directly from the
    /// engine code.
    packed: u32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("packed", &self.packed)
            .field("x", &self.x())
            .field("y", &self.y())
            .field("room_name", &self.room_name())
            .finish()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[room {} pos {},{}]",
            self.room_name(),
            self.x(),
            self.y()
        )
    }
}

impl Position {
    /// Create a new Position
    ///
    /// # Panics
    ///
    /// Will panic if either `x` or `y` is larger than 49, or if `room_name` is
    /// outside of the range `E127N127 - W127S127`.
    #[inline]
    pub fn new(x: u32, y: u32, room_name: RoomName) -> Self {
        assert!(x < 50, "out of bounds x: {}", x);
        assert!(y < 50, "out of bounds y: {}", y);

        Self::from_coords_adjusted_and_room_packed(x, y, room_name.packed_repr())
    }

    /// Creates a `Position` from x,y coordinates and room coordinates
    /// already adjusted to be positive using `HALF_WORLD_SIZE`.
    ///
    /// Non-public as this doesn't check the bounds for any of these values.
    #[inline]
    fn from_coords_and_world_coords_adjusted(x: u32, y: u32, room_x: u32, room_y: u32) -> Self {
        Position {
            packed: (room_x << 24) | (room_y << 16) | (x << 8) | y,
        }
    }

    /// Creates a `Position` from x,y coordinates and an already-packed room
    /// representation.
    ///
    /// Non-public as this doesn't check the bounds for any of these values.
    #[inline]
    fn from_coords_adjusted_and_room_packed(x: u32, y: u32, room_repr_packed: u16) -> Self {
        Position {
            packed: ((room_repr_packed as u32) << 16) | (x << 8) | y,
        }
    }

    #[inline]
    pub fn packed_repr(self) -> i32 {
        self.packed as i32
    }

    #[inline]
    pub fn from_packed(packed: i32) -> Self {
        Position {
            packed: packed as u32,
        }
    }

    /// Gets the horizontal coordinate of this position's room name.
    #[inline]
    fn room_x(self) -> i32 {
        self.room_name().x_coord()
    }

    /// Gets the vertical coordinate of this position's room name.
    #[inline]
    fn room_y(self) -> i32 {
        self.room_name().y_coord()
    }

    /// Gets this position's in-room x coordinate.
    #[inline]
    pub fn x(self) -> u32 {
        self.packed >> 8 & 0xFF
    }

    /// Gets this position's in-room y coordinate.
    #[inline]
    pub fn y(self) -> u32 {
        self.packed & 0xFF
    }

    #[inline]
    pub fn room_name(self) -> RoomName {
        RoomName::from_packed(((self.packed >> 16) & 0xFFFF) as u16)
    }

    #[inline]
    pub fn set_x(&mut self, x: u32) {
        assert!(x < 50, "out of bounds x: {}", x);
        self.packed = (self.packed & !(0xFF << 8)) | (x << 8);
    }

    #[inline]
    pub fn set_y(&mut self, y: u32) {
        assert!(y < 50, "out of bounds y: {}", y);
        self.packed = (self.packed & !0xFF) | y;
    }

    #[inline]
    pub fn set_room_name(&mut self, room_name: RoomName) {
        let room_repr_packed = room_name.packed_repr() as u32;
        self.packed = (self.packed & 0xFFFF) | (room_repr_packed << 16);
    }

    #[inline]
    pub fn with_x(mut self, x: u32) -> Self {
        self.set_x(x);
        self
    }

    #[inline]
    pub fn with_y(mut self, y: u32) -> Self {
        self.set_y(y);
        self
    }

    #[inline]
    pub fn with_room_name(mut self, room_name: RoomName) -> Self {
        self.set_room_name(room_name);
        self
    }
}

mod stdweb {
    use stdweb::{Reference, Value};

    use crate::{
        macros::*,
        traits::{TryFrom, TryInto},
    };

    use super::Position;

    impl Position {
        pub fn remote(self) -> Reference {
            js_unwrap!(pos_from_packed(@{self.packed_repr()}))
        }
    }

    impl TryFrom<Value> for Position {
        type Error = <Value as TryInto<String>>::Error;

        fn try_from(v: Value) -> Result<Position, Self::Error> {
            if let Value::Number(v) = v {
                let packed: i32 = v.try_into()?;
                return Ok(Position::from_packed(packed));
            }

            let value = js! {
                return @{v}.__packedPos;
            };

            match value {
                Value::Undefined => {
                    let x = js! {v.x}.try_into()?;
                    let y = js! {v.y}.try_into()?;
                    let room_name = js! {v.roomName}.try_into()?;
                    Ok(Self::new(x, y, room_name))
                }
                other => Ok(Self::from_packed(other.try_into()?)),
            }
        }
    }

    impl crate::traits::FromExpectedType<Reference> for Position {
        fn from_expected_type(reference: Reference) -> Result<Self, crate::ConversionError> {
            Self::try_from(Value::Reference(reference))
        }
    }

    js_serializable!(Position);
}

mod serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{Position, RoomName};

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ReadableFormat {
        room_name: RoomName,
        x: u32,
        y: u32,
    }

    impl From<ReadableFormat> for Position {
        fn from(ReadableFormat { room_name, x, y }: ReadableFormat) -> Self {
            Position::new(x, y, room_name)
        }
    }

    impl From<Position> for ReadableFormat {
        fn from(pos: Position) -> Self {
            ReadableFormat {
                room_name: pos.room_name(),
                x: pos.x(),
                y: pos.y(),
            }
        }
    }

    impl Serialize for Position {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                ReadableFormat::from(*self).serialize(serializer)
            } else {
                self.packed_repr().serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for Position {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                ReadableFormat::deserialize(deserializer).map(Into::into)
            } else {
                i32::deserialize(deserializer).map(Position::from_packed)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Position;

    const TEST_POSITIONS: &[(i32, (u32, u32, &str))] = &[
        (-2122440404i32, (33, 44, "E1N1")),
        (-1803615720i32, (2, 24, "E20N0")),
        (2139029504i32, (0, 0, "W0N0")),
        (-2139160576i32, (0, 0, "E0N0")),
        (2139095040i32, (0, 0, "W0S0")),
        (-2139095040i32, (0, 0, "E0S0")),
    ];

    #[test]
    fn from_i32_accurate() {
        for (packed, (x, y, name)) in TEST_POSITIONS.iter().copied() {
            let pos = Position::from_packed(packed);
            assert_eq!(pos.x(), x);
            assert_eq!(pos.y(), y);
            assert_eq!(&*pos.room_name().to_array_string(), name);
        }
    }

    #[test]
    fn from_args_accurate() {
        for (packed, (x, y, name)) in TEST_POSITIONS.iter().copied() {
            let pos = Position::new(x, y, name.parse().unwrap());
            assert_eq!(pos.packed_repr(), packed);
        }
    }
}
