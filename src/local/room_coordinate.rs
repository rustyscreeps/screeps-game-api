use std::{error::Error, fmt};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::{Direction, ROOM_SIZE};

pub(crate) const ROOM_AREA: usize = (ROOM_SIZE as usize) * (ROOM_SIZE as usize);

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError(u8);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds coordinate: {}", self.0)
    }
}

impl Error for OutOfBoundsError {}

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

/// An X or Y coordinate in a room, restricted to the valid range of
/// coordinates.
#[derive(
    Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(try_from = "u8", into = "u8")]
pub struct RoomCoordinate(u8);

impl RoomCoordinate {
    /// Create a `RoomCoordinate` from a `u8`, returning an error if the
    /// coordinate is not in the valid room size range
    #[inline]
    pub const fn new(coord: u8) -> Result<Self, OutOfBoundsError> {
        if coord < ROOM_SIZE {
            Ok(RoomCoordinate(coord))
        } else {
            Err(OutOfBoundsError(coord))
        }
    }

    /// Create a `RoomCoordinate` from a `u8`, without checking whether it's in
    /// the range of valid values.
    ///
    /// # Safety
    /// Calling this method with `coord >= ROOM_SIZE` can result in undefined
    /// behaviour when the resulting `RoomCoordinate` is used.
    #[inline]
    pub unsafe fn unchecked_new(coord: u8) -> Self {
        debug_assert!(
            coord < ROOM_SIZE,
            "Out of bounds unchecked coordinate: {coord}"
        );
        RoomCoordinate(coord)
    }

    /// Get the integer value of this coordinate
    pub const fn u8(self) -> u8 {
        self.0
    }

    /// Get whether this coordinate represents an edge position (0 or 49)
    pub const fn is_edge(self) -> bool {
        self.0 == 0 || self.0 == ROOM_SIZE - 1
    }

    /// Get the coordinate adjusted by a certain value, returning `None` if the
    /// result is outside the valid range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomCoordinate;
    ///
    /// let zero = RoomCoordinate::new(0).unwrap();
    /// let forty_nine = RoomCoordinate::new(49).unwrap();
    ///
    /// assert_eq!(zero.checked_add(1), Some(RoomCoordinate::new(1).unwrap()));
    /// assert_eq!(zero.checked_add(-1), None);
    /// assert_eq!(zero.checked_add(49), Some(forty_nine));
    /// assert_eq!(forty_nine.checked_add(1), None);
    /// ```
    pub fn checked_add(self, rhs: i8) -> Option<RoomCoordinate> {
        match (self.0 as i8).checked_add(rhs) {
            Some(result) => match result {
                // less than 0
                i8::MIN..=-1 => None,
                // greater than 49
                50..=i8::MAX => None,
                // SAFETY: we've checked that this coord is in the valid range
                c => Some(unsafe { RoomCoordinate::unchecked_new(c as u8) }),
            },
            None => None,
        }
    }

    /// Get the coordinate adjusted by a certain value, saturating at the edges
    /// of the room if the result would be outside of the valid range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomCoordinate;
    ///
    /// let zero = RoomCoordinate::new(0).unwrap();
    /// let forty_nine = RoomCoordinate::new(49).unwrap();
    ///
    /// assert_eq!(zero.saturating_add(1), RoomCoordinate::new(1).unwrap());
    /// assert_eq!(zero.saturating_add(-1), zero);
    /// assert_eq!(zero.saturating_add(i8::MAX), forty_nine);
    /// assert_eq!(forty_nine.saturating_add(1), forty_nine);
    /// assert_eq!(forty_nine.saturating_add(i8::MIN), zero);
    /// ```
    pub fn saturating_add(self, rhs: i8) -> RoomCoordinate {
        let result = match (self.0 as i8).saturating_add(rhs) {
            // less than 0, saturate to 0
            i8::MIN..=-1 => 0,
            // greater than 49, saturate to 49
            50..=i8::MAX => ROOM_SIZE - 1,
            c => c as u8,
        };
        // SAFETY: we've ensured that this coord is in the valid range
        unsafe { RoomCoordinate::unchecked_new(result) }
    }
}

impl fmt::Display for RoomCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An X/Y pair representing a given coordinate relative to any room.
#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct RoomXY {
    pub x: RoomCoordinate,
    pub y: RoomCoordinate,
}

impl RoomXY {
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
    pub const fn is_edge(self) -> bool {
        self.x.is_edge() || self.y.is_edge()
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
}

impl fmt::Display for RoomXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<RoomCoordinate> for u8 {
    fn from(coord: RoomCoordinate) -> u8 {
        coord.0
    }
}

impl TryFrom<u8> for RoomCoordinate {
    type Error = OutOfBoundsError;

    fn try_from(coord: u8) -> Result<Self, Self::Error> {
        RoomCoordinate::new(coord)
    }
}

impl From<RoomXY> for (u8, u8) {
    fn from(xy: RoomXY) -> (u8, u8) {
        (xy.x.0, xy.y.0)
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
