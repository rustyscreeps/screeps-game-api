use std::convert::{TryFrom, TryInto};
use std::fmt;

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

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
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
            x: xy.0.try_into()?,
            y: xy.1.try_into()?
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
