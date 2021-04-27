use std::convert::TryFrom;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

pub const ROOM_SIZE: u8 = 50;
pub const ROOM_AREA: usize = (ROOM_SIZE as usize) * (ROOM_SIZE as usize);

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError(u8);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds coordinate: {}", self.0)
    }
}

#[inline]
pub fn xy_to_linear_index(xy: RoomXY) -> usize {
    ((xy.x.0 as usize) * (ROOM_SIZE as usize)) + (xy.y.0 as usize)
}

#[inline]
pub fn linear_index_to_xy(idx: usize) -> RoomXY {
    assert!(idx < ROOM_AREA, "Out of bounds index: {}", idx);
    // SAFETY: bounds checking above ensures both are within range.
    RoomXY {
        x: unsafe { RoomCoordinate::unchecked_new((idx / (ROOM_SIZE as usize)) as u8) },
        y: unsafe { RoomCoordinate::unchecked_new((idx % (ROOM_SIZE as usize)) as u8) }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoomCoordinate(u8);

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomXY {
    pub x: RoomCoordinate,
    pub y: RoomCoordinate
}

impl fmt::Display for RoomXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl RoomCoordinate {
    // # Safety
    // Calling this method with `coord >= 50` can result in undefined behaviour when used.
    #[inline]
    pub unsafe fn unchecked_new(coord: u8) -> Self {
        debug_assert!(coord < ROOM_SIZE, "Out of bounds unchecked coordinate: {}", coord);
        RoomCoordinate(coord)
    }

    #[inline]
    pub fn val(self) -> u8 {
        self.0
    }
}

impl fmt::Display for RoomCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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
        if coord < ROOM_SIZE {
            Ok(RoomCoordinate(coord))
        } else {
            Err(OutOfBoundsError(coord))
        }
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
            y: RoomCoordinate::try_from(xy.1)?
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

impl Serialize for RoomCoordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

impl<'de> Deserialize<'de> for RoomCoordinate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u8::deserialize(deserializer)?;
        RoomCoordinate::try_from(val).map_err(|_| {
            de::Error::invalid_value(de::Unexpected::Unsigned(val as u64),
                                     &format!("a non-negative integer less-than {}", ROOM_SIZE).as_str())
        })
    }
}
