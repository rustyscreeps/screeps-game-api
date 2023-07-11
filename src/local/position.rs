//! Room position type and related operations and traits.
//!
//! This is a reimplementation/translation of the `RoomPosition` code originally
//! written in JavaScript. All RoomPosition to RoomPosition operations in this
//! file stay within Rust.
use core::fmt::Debug;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt,
};

use crate::{constants::ROOM_SIZE, objects::RoomPosition, HasPosition};

use super::{RoomCoordinate, RoomName, RoomXY, HALF_WORLD_SIZE};

mod approximate_offsets;
mod extra_math;
mod game_math;
mod game_methods;
mod pair_utils;
mod world_utils;

/// Represents a position in a particular room in Screeps.
///
/// **Note:** This is analogous to the `RoomPosition` JavaScript type.
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
/// If you need a reference to a `RoomPosition` in JavaScript,
/// convert the native [`Position`] to a [`RoomPosition`]:
///
/// ```no_run
/// use screeps::{Position, RoomCoordinate, RoomPosition};
/// use std::convert::TryFrom;
///
/// let pos = Position::new(
///     RoomCoordinate::try_from(20).unwrap(),
///     RoomCoordinate::try_from(21).unwrap(),
///     "E5N6".parse().unwrap(),
/// );
/// let js_pos = RoomPosition::from(pos);
/// let result = js_pos.room_name();
/// ```
///
/// # Deserialization
///
/// `Position` implements `TryFrom<Value>`, allowing conversion from values
/// retrieved from JavaScript. The implementation is fairly lenient, and will
/// try to accept the value as any of the following things, in order:
///
/// - an integer representing the packedPos
///   - this can be produced by retrieving the `__packedPos` field of a
///     `RoomPosition`
/// - an object with a `__packedPos` property
///   - this allows converting from a JavaScript `RoomPosition` to a `Position`
///     without referencing `__packedPos` manually, but is less efficient since
///     it requires an extra callback into JavaScript to grab that field from
///     within the conversion code
/// - an object with `x`, `y` and `roomName` properties
///   - this is mainly intended to decode `Position`s which were previously sent
///     to JavaScript using `@{}` in `js!{}`, or serialized using
///     [`serde::Serialize`]
///   - this will also understand `RoomPosition`s in private servers versions
///     `3.2.1` and below, prior to when `__packedPos` was added
///
/// # World vs. in-room coordinates
///
/// When converting `Position` to integer x/y coordinates, there are two main
/// methods. The first is to use `x`/`y` as "in room" coordinates, which are
/// bounded within `0..=49`. These coordinates only identify the location within
/// a given room name. These are used by [`Position::x`], [`Position::y`],
/// [`Position::new`] as well as [`Position::coords`],
/// [`Position::coords_signed`] and the various implementations of `Into<([ui*],
/// [ui*])>` for `Position`.
///
/// The second is to use `x`/`y` as "world" coordinates, which are coordinates
/// spread across the world. To ensures they agree with in-room coordinates,
/// south is positive `y`, north is negative `y`, east is positive `x` and west
/// is negative `x`. One way to think of them is as extending the room
/// coordinates of the room `E0S0` throughout the entire map.
///
/// World coordinates are used by [`Position::world_x`], [`Position::world_y`],
/// [`Position::world_coords`], [`Position::from_world_coords`], and by all
/// implementations which allow adding or subtracting positions (see [Addition
/// and subtraction](#addition-and-subtraction)).
///
/// # Method Behavior
///
/// While this corresponds with the JavaScript `RoomPosition` type, it is not
/// identical. In particular, all "calculation" methods which take in another
/// position are re-implemented in pure Rust code, and some behave slightly
/// different.
///
/// For instance, [`Position::get_range_to`] operates on positions as world
/// coordinates, and will return accurate distances for positions in different
/// rooms. This is in contrast to `RoomPosition.getRangeTo` in JavaScript, which
/// will return `Infinity` for positions from different rooms.
/// [`Position::in_range_to`] has a similar difference.
///
/// Besides extending behavior to work between rooms, we've tried to keep
/// methods as in-sync with the JavaScript versions as possible. Everything
/// will "just work", and there should be some speed advantage because of not
/// having to call into JavaScript to perform calculations.
///
/// # Addition and subtraction
///
/// [`Position`] implements `Add<(i32, i32)>`, `Sub<(i32, i32)>` and
/// `Sub<Position>`. All of these implementations work on positions as world
/// positions, and will treat positions from different rooms just as if they're
/// further apart.
///
/// The `Add` implementation can be used to add an offset to a position:
///
/// ```
/// # use std::convert::TryFrom;
/// # use screeps::{Position, RoomCoordinate};
/// let pos1 = Position::new(
///     RoomCoordinate::try_from(0).unwrap(),
///     RoomCoordinate::try_from(0).unwrap(),
///     "E1N1".parse().unwrap(),
/// );
/// let pos2 = Position::new(
///     RoomCoordinate::try_from(40).unwrap(),
///     RoomCoordinate::try_from(20).unwrap(),
///     "E1N1".parse().unwrap(),
/// );
/// assert_eq!(pos1 + (40, 20), pos2);
/// ```
///
/// And the `Sub` implementation can be used to get the offset between two
/// positions:
///
/// ```
/// # use std::convert::TryFrom;
/// # use screeps::{Position, RoomCoordinate};
/// let pos1 = Position::new(
///     RoomCoordinate::try_from(4).unwrap(),
///     RoomCoordinate::try_from(20).unwrap(),
///     "E20S21".parse().unwrap(),
/// );
/// let pos2 = Position::new(
///     RoomCoordinate::try_from(4).unwrap(),
///     RoomCoordinate::try_from(30).unwrap(),
///     "E20S22".parse().unwrap(),
/// );
/// assert_eq!(pos2 - pos1, (0, 60));
///
/// let pos3 = Position::new(
///     RoomCoordinate::try_from(0).unwrap(),
///     RoomCoordinate::try_from(0).unwrap(),
///     "E20S21".parse().unwrap(),
/// );
/// assert_eq!(pos3 - pos1, (-4, -20));
/// ```
///
/// # Ordering
///
/// To facilitate use as a key in a [`BTreeMap`] or other similar data
/// structures, `Position` implements [`PartialOrd`] and [`Ord`].
///
/// `Position`s are ordered first by ascending world `y` position, then by
/// ascending world `x` position. World `x` and `y` here simply extend the x,y
/// coords within the room `E0S0` throughout the map.
///
/// Looking at positions as tuples `(world_x, world_y)`, the sorting obeys rules
/// such as:
///
/// - `(a, 0) < (b, 1)` for any `a`, `b`
/// - `(0, c) < (1, c)` for any `c`
///
/// This follows left-to-right reading order when looking at the Screeps map
/// from above.
///
/// [`bincode`]: https://github.com/servo/bincode
/// [`RoomObject::pos`]: crate::RoomObject::pos
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`serde::Serialize`]: ::serde::Serialize
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldPositionOutOfBoundsError(pub i32, pub i32);

impl Position {
    /// Create a new Position
    ///
    /// # Panics
    ///
    /// Will panic if either `x` or `y` is larger than 49, or if `room_name` is
    /// outside of the range `E127N127 - W127S127`.
    #[inline]
    pub fn new(x: RoomCoordinate, y: RoomCoordinate, room_name: RoomName) -> Self {
        Self::from_coords_adjusted_and_room_packed(x.into(), y.into(), room_name.packed_repr())
    }

    /// Creates a `Position` from x,y coordinates and room coordinates
    /// already adjusted to be positive using `HALF_WORLD_SIZE`.
    ///
    /// Non-public as this doesn't check the bounds for any of these values.
    #[inline]
    const fn from_coords_and_world_coords_adjusted(x: u8, y: u8, room_x: u32, room_y: u32) -> Self {
        Position {
            packed: (room_x << 24) | (room_y << 16) | ((x as u32) << 8) | (y as u32),
        }
    }

    /// Creates a `Position` from x,y coordinates and an already-packed room
    /// representation.
    ///
    /// Non-public as this doesn't check the bounds for any of these values.
    #[inline]
    const fn from_coords_adjusted_and_room_packed(x: u8, y: u8, room_repr_packed: u16) -> Self {
        Position {
            packed: ((room_repr_packed as u32) << 16) | ((x as u32) << 8) | (y as u32),
        }
    }

    #[inline]
    pub const fn packed_repr(self) -> u32 {
        self.packed
    }

    #[inline]
    pub fn from_packed(packed: u32) -> Self {
        let x = packed >> 8 & 0xFF;
        let y = packed & 0xFF;
        assert!(x < ROOM_SIZE as u32, "out of bounds x: {x}");
        assert!(y < ROOM_SIZE as u32, "out of bounds y: {y}");
        Position { packed }
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
    pub fn x(self) -> RoomCoordinate {
        // SAFETY: packed always contains a valid x coordinate
        unsafe { RoomCoordinate::unchecked_new((self.packed >> 8 & 0xFF) as u8) }
    }

    /// Gets this position's in-room y coordinate.
    #[inline]
    pub fn y(self) -> RoomCoordinate {
        // SAFETY: packed always contains a valid y coordinate
        unsafe { RoomCoordinate::unchecked_new((self.packed & 0xFF) as u8) }
    }

    /// Gets this position's in-room [`RoomXY`] coordinate pair
    #[inline]
    pub fn xy(self) -> RoomXY {
        // SAFETY: packed always contains a valid pair
        unsafe {
            RoomXY::unchecked_new((self.packed >> 8 & 0xFF) as u8, (self.packed & 0xFF) as u8)
        }
    }

    #[inline]
    pub fn room_name(self) -> RoomName {
        RoomName::from_packed(((self.packed >> 16) & 0xFFFF) as u16)
    }

    #[inline]
    pub fn set_x(&mut self, x: RoomCoordinate) {
        self.packed = (self.packed & !(0xFF << 8)) | ((u8::from(x) as u32) << 8);
    }

    #[inline]
    pub fn set_y(&mut self, y: RoomCoordinate) {
        self.packed = (self.packed & !0xFF) | (u8::from(y) as u32);
    }

    #[inline]
    pub fn set_room_name(&mut self, room_name: RoomName) {
        let room_repr_packed = room_name.packed_repr() as u32;
        self.packed = (self.packed & 0xFFFF) | (room_repr_packed << 16);
    }

    #[inline]
    pub fn with_x(mut self, x: RoomCoordinate) -> Self {
        self.set_x(x);
        self
    }

    #[inline]
    pub fn with_y(mut self, y: RoomCoordinate) -> Self {
        self.set_y(y);
        self
    }

    #[inline]
    pub fn with_room_name(mut self, room_name: RoomName) -> Self {
        self.set_room_name(room_name);
        self
    }
}

impl PartialOrd for Position {
    #[inline]
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.world_y()
            .cmp(&other.world_y())
            .then_with(|| self.world_x().cmp(&other.world_x()))
    }
}

impl HasPosition for Position {
    fn pos(&self) -> Position {
        *self
    }
}

impl From<RoomPosition> for Position {
    fn from(js_pos: RoomPosition) -> Self {
        Position::from_packed(js_pos.packed())
    }
}

impl From<&RoomPosition> for Position {
    fn from(js_pos: &RoomPosition) -> Self {
        Position::from_packed(js_pos.packed())
    }
}

mod serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{Position, RoomCoordinate, RoomName};

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ReadableFormat {
        room_name: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,
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
                u32::deserialize(deserializer).map(Position::from_packed)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Position, RoomCoordinate};

    fn gen_test_positions() -> Vec<(u32, (RoomCoordinate, RoomCoordinate, &'static str))> {
        unsafe {
            vec![
                (
                    2172526892u32,
                    (
                        RoomCoordinate::unchecked_new(33),
                        RoomCoordinate::unchecked_new(44),
                        "E1N1",
                    ),
                ),
                (
                    2491351576u32,
                    (
                        RoomCoordinate::unchecked_new(2),
                        RoomCoordinate::unchecked_new(24),
                        "E20N0",
                    ),
                ),
                (
                    2139029504u32,
                    (
                        RoomCoordinate::unchecked_new(0),
                        RoomCoordinate::unchecked_new(0),
                        "W0N0",
                    ),
                ),
                (
                    2155806720u32,
                    (
                        RoomCoordinate::unchecked_new(0),
                        RoomCoordinate::unchecked_new(0),
                        "E0N0",
                    ),
                ),
                (
                    2139095040u32,
                    (
                        RoomCoordinate::unchecked_new(0),
                        RoomCoordinate::unchecked_new(0),
                        "W0S0",
                    ),
                ),
                (
                    2155872256u32,
                    (
                        RoomCoordinate::unchecked_new(0),
                        RoomCoordinate::unchecked_new(0),
                        "E0S0",
                    ),
                ),
                (
                    2021333800u32,
                    (
                        RoomCoordinate::unchecked_new(27),
                        RoomCoordinate::unchecked_new(40),
                        "W7N4",
                    ),
                ),
                // this one is in the top left room - which is either sim, or W127N127 if the
                // sim position overrides are inactive
                (
                    1285u32,
                    (
                        RoomCoordinate::unchecked_new(5),
                        RoomCoordinate::unchecked_new(5),
                        if cfg!(feature = "sim") {
                            "sim"
                        } else {
                            "W127N127"
                        },
                    ),
                ),
            ]
        }
    }

    #[test]
    fn from_u32_accurate() {
        for (packed, (x, y, name)) in gen_test_positions().iter().copied() {
            let pos = Position::from_packed(packed);
            assert_eq!(pos.x(), x);
            assert_eq!(pos.y(), y);
            assert_eq!(&*pos.room_name().to_array_string(), name);
        }
    }

    #[test]
    fn from_args_accurate() {
        for (packed, (x, y, name)) in gen_test_positions().iter().copied() {
            let pos = Position::new(x, y, name.parse().unwrap());
            assert_eq!(pos.packed_repr(), packed);
        }
    }
}
