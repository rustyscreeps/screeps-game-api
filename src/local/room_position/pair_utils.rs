//! Utilities for working with Position and coordinate pairs
use super::Position;
use crate::local::RoomXY;

macro_rules! int_pair_from_position {
    ($($num_type:ty),+) => {$(
            impl From<Position> for ($num_type, $num_type) {
                #[inline]
                fn from(pos: Position) -> Self {
                    (u8::from(pos.x()) as $num_type, u8::from(pos.y()) as $num_type)
                }
            }
    )+}
}

int_pair_from_position!(u8, u16, u32, u64, i8, i16, i32, i64);

impl Position {
    /// Returns this position's in-room coordinates as a pair of unsigned
    /// integers.
    #[inline]
    pub fn coords(&self) -> (u8, u8) {
        (self.x().into(), self.y().into())
    }

    /// Returns this position's in-room coordinates as a pair of signed
    /// integers.
    #[inline]
    pub fn coords_signed(&self) -> (i8, i8) {
        (u8::from(self.x()) as i8, u8::from(self.y()) as i8)
    }
}

impl From<Position> for RoomXY {
    #[inline]
    fn from(pos: Position) -> RoomXY {
        RoomXY {
            x: pos.x(),
            y: pos.y(),
        }
    }
}
