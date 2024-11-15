//! Math utilities on `RoomXY` which don't exist in the Screeps API
//! proper.

use std::ops::{Add, Sub};

use super::{RoomOffsetXY, RoomXY, XY};
use crate::constants::Direction;

impl RoomXY {
    /// Returns a new position offset from this position by the specified x
    /// coords and y coords.
    ///
    /// Unlike [`Position::offset`], this function operates on room coordinates,
    /// and will panic if the new position overflows the room.
    ///
    /// To return a new position rather than modifying in place, use `pos + (x,
    /// y)`. See the implementation of `Add<(i8, i8)>` for
    /// [`RoomXY`] further down on this page.
    ///
    /// # Panics
    ///
    /// Will panic if the new position overflows the room.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::RoomXY;
    ///
    /// let mut pos = RoomXY::checked_new(21, 21).unwrap();
    /// pos.offset(5, 5);
    /// assert_eq!(pos, RoomXY::checked_new(26, 26).unwrap());
    ///
    /// let mut pos = RoomXY::checked_new(21, 21).unwrap();
    /// pos.offset(-5, 5);
    /// assert_eq!(pos, RoomXY::checked_new(16, 26).unwrap());
    /// ```
    ///
    /// [`Position::offset`]: crate::local::Position::offset
    #[inline]
    #[track_caller]
    pub fn offset(&mut self, x: i8, y: i8) {
        *self = *self + (x, y);
    }
}

impl Add<(i8, i8)> for RoomXY {
    type Output = RoomXY;

    /// Adds an `(x, y)` pair to this position's coordinates.
    ///
    /// # Panics
    ///
    /// Will panic if the new position is outside standard room bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::RoomXY;
    ///
    /// let pos1 = RoomXY::checked_new(42, 42).unwrap();
    /// let pos2 = pos1 + (7, 7);
    /// assert_eq!(pos2, RoomXY::checked_new(49, 49).unwrap());
    /// ```
    #[inline]
    #[track_caller]
    fn add(self, (x, y): (i8, i8)) -> Self {
        self.checked_add((x, y)).unwrap()
    }
}

impl Add<Direction> for RoomXY {
    type Output = RoomXY;

    /// Adds a `Direction` to this position's coordinates.
    ///
    /// # Panics
    ///
    /// Will panic if the new position is outside standard room bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::{RoomXY, Direction};
    ///
    /// let pos1 = RoomXY::checked_new(49, 40).unwrap();
    /// let pos2 = pos1 + Direction::Top;
    /// assert_eq!(pos2, RoomXY::checked_new(49, 39).unwrap());
    /// ```
    #[inline]
    #[track_caller]
    fn add(self, direction: Direction) -> Self {
        self.checked_add_offset(direction.into()).unwrap()
    }
}

impl Sub<(i8, i8)> for RoomXY {
    type Output = RoomXY;

    /// Subtracts an `(x, y)` pair from this position's coordinates.
    ///
    /// # Panics
    ///
    /// Will panic if the new position is outside standard room bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::RoomXY;
    ///
    /// let pos1 = RoomXY::checked_new(49, 40).unwrap();
    /// let pos2 = pos1 - (49, 0);
    /// assert_eq!(pos2, RoomXY::checked_new(0, 40).unwrap());
    /// ```
    #[inline]
    #[track_caller]
    fn sub(self, (x, y): (i8, i8)) -> Self {
        self.checked_add((-x, -y)).unwrap()
    }
}

impl Sub<Direction> for RoomXY {
    type Output = RoomXY;

    /// Subtracts a `Direction` from this position's coordinates.
    ///
    /// # Panics
    ///
    /// Will panic if the new position is outside standard room bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::{RoomXY, Direction};
    ///
    /// let pos1 = RoomXY::checked_new(49, 40).unwrap();
    /// let pos2 = pos1 - Direction::Top;
    /// assert_eq!(pos2, RoomXY::checked_new(49, 41).unwrap());
    /// ```
    #[inline]
    fn sub(self, direction: Direction) -> Self {
        self.checked_add_offset(-RoomOffsetXY::from(direction))
            .unwrap()
    }
}
