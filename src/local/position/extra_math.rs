//! Math utilities on `Position` which don't exist in the Screeps API
//! proper.
use std::ops::{Add, Sub};

use super::Position;

impl Position {
    /// Returns a new position offset from this position by the specified x
    /// coords and y coords.
    ///
    /// This function operates on world coordinates, and will wrap between rooms
    /// if necessary.
    ///
    /// To return a new position rather than modifying in place, use `pos + (x,
    /// y)`. See the implementation of `Add<(i32, i32)>` for
    /// [`Position`] further down on this page.
    ///
    /// # Panics
    ///
    /// Will panic if the new position overflows the world. See
    /// [`Position::from_world_coords`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use screeps::{Position, RoomCoordinate};
    /// let e21s21 = "E21S21".parse().unwrap();
    /// let e21s22 = "E21S22".parse().unwrap();
    ///
    /// let mut pos = Position::new(
    ///     RoomCoordinate::try_from(21).unwrap(),
    ///     RoomCoordinate::try_from(21).unwrap(),
    ///     e21s21,
    /// );
    /// pos.offset(5, 5);
    /// assert_eq!(
    ///     pos,
    ///     Position::new(
    ///         RoomCoordinate::try_from(26).unwrap(),
    ///         RoomCoordinate::try_from(26).unwrap(),
    ///         e21s21
    ///     )
    /// );
    ///
    /// pos.offset(0, 49);
    /// assert_eq!(
    ///     pos,
    ///     Position::new(
    ///         RoomCoordinate::try_from(26).unwrap(),
    ///         RoomCoordinate::try_from(25).unwrap(),
    ///         e21s22
    ///     )
    /// );
    /// ```
    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        *self = *self + (x, y);
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    /// Adds an `(x, y)` pair to this room position's world coordinates.
    ///
    /// Will change rooms if necessary.
    ///
    /// # Panics
    ///
    /// Will panic if the new position's room is outside bounds. See
    /// [`Position::from_world_coords`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use screeps::{Position, RoomCoordinate};
    /// let w5s6 = "W5S6".parse().unwrap();
    /// let w5s5 = "W5S5".parse().unwrap();
    ///
    /// let pos1 = Position::new(
    ///     RoomCoordinate::try_from(42).unwrap(),
    ///     RoomCoordinate::try_from(42).unwrap(),
    ///     w5s6,
    /// );
    /// let pos2 = pos1 + (7, 7);
    /// assert_eq!(
    ///     pos2,
    ///     Position::new(
    ///         RoomCoordinate::try_from(49).unwrap(),
    ///         RoomCoordinate::try_from(49).unwrap(),
    ///         w5s6
    ///     )
    /// );
    ///
    /// let pos3 = pos2 + (0, -59);
    /// assert_eq!(
    ///     pos3,
    ///     Position::new(
    ///         RoomCoordinate::try_from(49).unwrap(),
    ///         RoomCoordinate::try_from(40).unwrap(),
    ///         w5s5
    ///     )
    /// );
    ///
    /// let pos4 = pos3 - (49, 0);
    /// assert_eq!(
    ///     pos4,
    ///     Position::new(
    ///         RoomCoordinate::try_from(0).unwrap(),
    ///         RoomCoordinate::try_from(40).unwrap(),
    ///         w5s5
    ///     )
    /// );
    /// ```
    #[inline]
    fn add(self, (x, y): (i32, i32)) -> Self {
        let (wx, wy) = self.world_coords();
        Self::from_world_coords(wx + x, wy + y)
    }
}

impl Sub<(i32, i32)> for Position {
    type Output = Position;

    /// See the implementation of `Add<(i32, i32)>` for [`Position`].
    #[inline]
    fn sub(self, (x, y): (i32, i32)) -> Self {
        self + (-x, -y)
    }
}

impl Sub<Position> for Position {
    type Output = (i32, i32);

    /// Subtracts the other room position from this one, extracting the
    /// difference as the output.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use screeps::{Position, RoomCoordinate};
    /// let e5n5 = "E5N5".parse().unwrap();
    /// let e5n6 = "E5N6".parse().unwrap();
    ///
    /// let pos1 = Position::new(
    ///     RoomCoordinate::try_from(40).unwrap(),
    ///     RoomCoordinate::try_from(40).unwrap(),
    ///     e5n5,
    /// );
    /// let pos2 = Position::new(
    ///     RoomCoordinate::try_from(0).unwrap(),
    ///     RoomCoordinate::try_from(20).unwrap(),
    ///     e5n6,
    /// );
    /// assert_eq!(pos1 - pos2, (40, 70));
    /// ```
    #[inline]
    fn sub(self, other: Position) -> (i32, i32) {
        let (mx, my) = self.world_coords();
        let (ox, oy) = other.world_coords();
        (mx - ox, my - oy)
    }
}
