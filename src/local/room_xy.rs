use std::{cmp::Ordering, fmt};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::room_coordinate::{OutOfBoundsError, RoomCoordinate};
use crate::constants::{Direction, ROOM_AREA, ROOM_SIZE};

mod approximate_offsets;
mod extra_math;
mod game_math;

/// Converts a [`RoomXY`] coordinate pair to a linear index appropriate for use
/// with the internal representation of a [`CostMatrix`] or [`LocalCostMatrix`].
///
/// [`CostMatrix`]: crate::objects::CostMatrix
/// [`LocalCostMatrix`]: crate::local::LocalCostMatrix
#[inline]
pub const fn xy_to_linear_index(xy: RoomXY) -> usize {
    xy.x.u8() as usize * ROOM_SIZE as usize + xy.y.u8() as usize
}

/// Converts a linear index from the internal representation of a [`CostMatrix`]
/// or [`LocalCostMatrix`] to a [`RoomXY`] coordinate pair for the position the
/// index represents.
///
/// [`CostMatrix`]: crate::objects::CostMatrix
/// [`LocalCostMatrix`]: crate::local::LocalCostMatrix
#[inline]
pub fn linear_index_to_xy(idx: usize) -> RoomXY {
    assert!(idx < ROOM_AREA, "Out of bounds index: {idx}");
    // SAFETY: bounds checking above ensures both are within range.
    RoomXY {
        x: unsafe { RoomCoordinate::unchecked_new((idx / (ROOM_SIZE as usize)) as u8) },
        y: unsafe { RoomCoordinate::unchecked_new((idx % (ROOM_SIZE as usize)) as u8) },
    }
}

/// Converts a [`RoomXY`] coordinate pair to a terrain index appropriate for use
/// with the internal representation of [`RoomTerrain`] or [`LocalRoomTerrain`].
///
/// [`RoomTerrain`]: crate::objects::RoomTerrain
/// [`LocalRoomTerrain`]: crate::local::LocalRoomTerrain
#[inline]
pub const fn xy_to_terrain_index(xy: RoomXY) -> usize {
    xy.y.u8() as usize * ROOM_SIZE as usize + xy.x.u8() as usize
}

/// Converts a terrain index from the internal representation of a
/// [`RoomTerrain`] or [`LocalRoomTerrain`] to a [`RoomXY`] coordinate pair for
/// the position the index represents.
///
/// [`RoomTerrain`]: crate::objects::RoomTerrain
/// [`LocalRoomTerrain`]: crate::local::LocalRoomTerrain
#[inline]
pub fn terrain_index_to_xy(idx: usize) -> RoomXY {
    assert!(idx < ROOM_AREA, "Out of bounds index: {idx}");
    // SAFETY: bounds checking above ensures both are within range.
    RoomXY {
        x: unsafe { RoomCoordinate::unchecked_new((idx % (ROOM_SIZE as usize)) as u8) },
        y: unsafe { RoomCoordinate::unchecked_new((idx / (ROOM_SIZE as usize)) as u8) },
    }
}

/// An X/Y pair representing a given coordinate relative to any room.
#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct RoomXY {
    pub x: RoomCoordinate,
    pub y: RoomCoordinate,
}

impl RoomXY {
    /// Create a new `RoomXY` from a pair of `RoomCoordinate`.
    #[inline]
    pub fn new(x: RoomCoordinate, y: RoomCoordinate) -> Self {
        RoomXY { x, y }
    }

    /// Create a new `RoomXY` from a pair of `u8`, checking that they're in
    /// the range of valid values.
    #[inline]
    pub fn checked_new(x: u8, y: u8) -> Result<RoomXY, OutOfBoundsError> {
        RoomXY::try_from((x, y))
    }

    /// Create a `RoomXY` from a pair of `u8`, without checking whether it's in
    /// the range of valid values.
    ///
    /// # Safety
    /// Calling this method with `x >= ROOM_SIZE` or `y >= ROOM_SIZE` can result
    /// in undefined behaviour when the resulting `RoomXY` is used.
    #[inline]
    pub unsafe fn unchecked_new(x: u8, y: u8) -> Self {
        RoomXY {
            x: RoomCoordinate::unchecked_new(x),
            y: RoomCoordinate::unchecked_new(y),
        }
    }

    /// Get whether this coordinate pair represents an edge position (0 or 49
    /// for either coordinate)
    pub const fn is_room_edge(self) -> bool {
        self.x.is_room_edge() || self.y.is_room_edge()
    }

    /// Get the coordinate adjusted by a certain value, returning `None` if the
    /// result is outside the valid room area.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomXY;
    ///
    /// let zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let one = unsafe { RoomXY::unchecked_new(1, 1) };
    /// let forty_nine = unsafe { RoomXY::unchecked_new(49, 49) };
    ///
    /// assert_eq!(zero.checked_add((1, 1)), Some(one));
    /// assert_eq!(zero.checked_add((-1, 0)), None);
    /// assert_eq!(zero.checked_add((49, 49)), Some(forty_nine));
    /// assert_eq!(forty_nine.checked_add((1, 1)), None);
    /// ```
    pub fn checked_add(self, rhs: (i8, i8)) -> Option<RoomXY> {
        let x = match self.x.checked_add(rhs.0) {
            Some(x) => x,
            None => return None,
        };
        let y = match self.y.checked_add(rhs.1) {
            Some(y) => y,
            None => return None,
        };
        Some(RoomXY { x, y })
    }

    /// Get the coordinate adjusted by a certain value, saturating at the edges
    /// of the room if the result would be outside the valid room area.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomXY;
    ///
    /// let zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let one = unsafe { RoomXY::unchecked_new(1, 1) };
    /// let forty_nine = unsafe { RoomXY::unchecked_new(49, 49) };
    ///
    /// assert_eq!(zero.saturating_add((1, 1)), one);
    /// assert_eq!(zero.saturating_add((-1, 0)), zero);
    /// assert_eq!(zero.saturating_add((49, 49)), forty_nine);
    /// assert_eq!(zero.saturating_add((i8::MAX, i8::MAX)), forty_nine);
    /// assert_eq!(forty_nine.saturating_add((1, 1)), forty_nine);
    /// assert_eq!(forty_nine.saturating_add((i8::MIN, i8::MIN)), zero);
    /// ```
    pub fn saturating_add(self, rhs: (i8, i8)) -> RoomXY {
        let x = self.x.saturating_add(rhs.0);
        let y = self.y.saturating_add(rhs.1);
        RoomXY { x, y }
    }

    /// Get the neighbor of a given `RoomXY` in the given direction, returning
    /// `None` if the result is outside the valid room area.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::{constants::Direction::*, local::RoomXY};
    ///
    /// let zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let one = unsafe { RoomXY::unchecked_new(1, 1) };
    /// let forty_nine = unsafe { RoomXY::unchecked_new(49, 49) };
    ///
    /// assert_eq!(zero.checked_add_direction(BottomRight), Some(one));
    /// assert_eq!(zero.checked_add_direction(TopLeft), None);
    /// assert_eq!(one.checked_add_direction(TopLeft), Some(zero));
    /// assert_eq!(forty_nine.checked_add_direction(BottomRight), None);
    /// ```
    pub fn checked_add_direction(self, rhs: Direction) -> Option<RoomXY> {
        let (dx, dy) = rhs.into();
        self.checked_add((dx as i8, dy as i8))
    }

    /// Get the neighbor of a given `RoomXY` in the given direction, saturating
    /// at the edges if the result is outside the valid room area.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::{constants::Direction::*, local::RoomXY};
    ///
    /// let zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let one = unsafe { RoomXY::unchecked_new(1, 1) };
    /// let forty_nine = unsafe { RoomXY::unchecked_new(49, 49) };
    ///
    /// assert_eq!(zero.saturating_add_direction(BottomRight), one);
    /// assert_eq!(zero.saturating_add_direction(TopLeft), zero);
    /// assert_eq!(one.saturating_add_direction(TopLeft), zero);
    /// assert_eq!(forty_nine.saturating_add_direction(BottomRight), forty_nine);
    /// ```
    pub fn saturating_add_direction(self, rhs: Direction) -> RoomXY {
        let (dx, dy) = rhs.into();
        self.saturating_add((dx as i8, dy as i8))
    }

    /// Get all the valid neighbors of a given `RoomXY`.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomXY;
    ///
    /// let zero_zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let zero_one = unsafe { RoomXY::unchecked_new(0, 1) };
    /// let one_zero = unsafe { RoomXY::unchecked_new(1, 0) };
    /// let one_one = unsafe { RoomXY::unchecked_new(1, 1) };
    ///
    /// let zero_two = unsafe { RoomXY::unchecked_new(0, 2) };
    /// let one_two = unsafe { RoomXY::unchecked_new(1, 2) };
    /// let two_two = unsafe { RoomXY::unchecked_new(2, 2) };
    /// let two_one = unsafe { RoomXY::unchecked_new(2, 1) };
    /// let two_zero = unsafe { RoomXY::unchecked_new(2, 0) };
    ///
    /// let zero_zero_neighbors = zero_zero.neighbors();
    ///
    /// assert_eq!(zero_zero_neighbors.len(), 3);
    /// assert!(zero_zero_neighbors.contains(&zero_one));
    /// assert!(zero_zero_neighbors.contains(&one_one));
    /// assert!(zero_zero_neighbors.contains(&one_zero));
    ///
    /// let one_one_neighbors = one_one.neighbors();
    ///
    /// assert_eq!(one_one_neighbors.len(), 8);
    /// assert!(one_one_neighbors.contains(&zero_zero));
    /// assert!(one_one_neighbors.contains(&zero_one));
    /// assert!(one_one_neighbors.contains(&one_zero));
    /// assert!(one_one_neighbors.contains(&zero_two));
    /// assert!(one_one_neighbors.contains(&one_two));
    /// assert!(one_one_neighbors.contains(&two_two));
    /// assert!(one_one_neighbors.contains(&two_one));
    /// assert!(one_one_neighbors.contains(&two_zero));
    /// ```
    pub fn neighbors(self) -> Vec<RoomXY> {
        Direction::iter()
            .filter_map(|dir| self.checked_add_direction(*dir))
            .collect()
    }
}

impl PartialOrd for RoomXY {
    #[inline]
    fn partial_cmp(&self, other: &RoomXY) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoomXY {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl fmt::Display for RoomXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<RoomXY> for (u8, u8) {
    fn from(xy: RoomXY) -> (u8, u8) {
        (xy.x.u8(), xy.y.u8())
    }
}

impl TryFrom<(u8, u8)> for RoomXY {
    type Error = OutOfBoundsError;

    fn try_from(xy: (u8, u8)) -> Result<RoomXY, OutOfBoundsError> {
        Ok(RoomXY {
            x: RoomCoordinate::try_from(xy.0)?,
            y: RoomCoordinate::try_from(xy.1)?,
        })
    }
}

impl From<(RoomCoordinate, RoomCoordinate)> for RoomXY {
    fn from(xy: (RoomCoordinate, RoomCoordinate)) -> RoomXY {
        RoomXY { x: xy.0, y: xy.1 }
    }
}

impl From<RoomXY> for (RoomCoordinate, RoomCoordinate) {
    fn from(xy: RoomXY) -> (RoomCoordinate, RoomCoordinate) {
        (xy.x, xy.y)
    }
}

#[derive(Serialize, Deserialize)]
struct ReadableXY {
    x: RoomCoordinate,
    y: RoomCoordinate,
}

impl From<ReadableXY> for RoomXY {
    fn from(ReadableXY { x, y }: ReadableXY) -> RoomXY {
        RoomXY { x, y }
    }
}

impl From<RoomXY> for ReadableXY {
    fn from(RoomXY { x, y }: RoomXY) -> ReadableXY {
        ReadableXY { x, y }
    }
}

impl Serialize for RoomXY {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            ReadableXY::from(*self).serialize(serializer)
        } else {
            let xy: (u8, u8) = (*self).into();
            let packed: u16 = ((xy.0 as u16) << 8) | (xy.1 as u16);
            packed.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for RoomXY {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            ReadableXY::deserialize(deserializer).map(Into::into)
        } else {
            let packed = u16::deserialize(deserializer)?;
            let xy = (((packed >> 8) & 0xFF) as u8, (packed & 0xFF) as u8);
            RoomXY::try_from(xy).map_err(|err: OutOfBoundsError| {
                de::Error::invalid_value(
                    de::Unexpected::Unsigned(err.0 as u64),
                    &format!("a non-negative integer less-than {ROOM_SIZE}").as_str(),
                )
            })
        }
    }
}
