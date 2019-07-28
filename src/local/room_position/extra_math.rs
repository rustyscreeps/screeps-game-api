//! Math utilities on `RoomPosition` which don't exist in the Screeps API
//! proper.
use std::ops::{Add, Sub};

use super::RoomPosition;

impl RoomPosition {
    /// Returns a new position offset from this position by the specified x
    /// coords and y coords.
    ///
    /// This function operates on world coordinates, and will wrap between rooms
    /// if necessary.
    ///
    /// To return a new position rather than modifying in place, use `pos + (x,
    /// y)`. See the implementation of `Add<(i32, i32)>` for
    /// [`RoomPosition`] further down on this page.
    ///
    /// # Panics
    ///
    /// Will panic if the new position overflows the world. See
    /// [`RoomPosition::from_world_coords`].
    ///
    /// # Example
    ///
    /// ```
    /// use screeps::RoomPosition;
    ///
    /// let e21s21 = "E21S21".parse().unwrap();
    /// let e21s22 = "E21S22".parse().unwrap();
    ///
    /// let mut pos = RoomPosition::new(21, 21, e21s21);
    /// pos.offset(5, 5);
    /// assert_eq!(pos, RoomPosition::new(26, 26, e21s21));
    ///
    /// pos.offset(0, 49);
    /// assert_eq!(pos, RoomPosition::new(26, 25, e21s22));
    /// ```
    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        *self = *self + (x, y);
    }
}

impl Add<(i32, i32)> for RoomPosition {
    type Output = RoomPosition;

    /// Adds an `(x, y)` pair to this room position's world coordinates.
    ///
    /// Will change rooms if necessary.
    ///
    /// # Panics
    ///
    /// Will panic if the new position's room is outside bounds. See
    /// [`RoomPosition::from_world_coords`].
    ///
    /// # Example
    ///
    /// ```
    /// use screeps::RoomPosition;
    ///
    /// let w5s6 = "W5S6".parse().unwrap();
    /// let w5s5 = "W5S5".parse().unwrap();
    ///
    /// let pos1 = RoomPosition::new(42, 42, w5s6);
    /// let pos2 = pos1 + (7, 7);
    /// assert_eq!(pos2, RoomPosition::new(49, 49, w5s6));
    ///
    /// let pos3 = pos2 + (0, -59);
    /// assert_eq!(pos3, RoomPosition::new(49, 40, w5s5));
    ///
    /// let pos4 = pos3 - (49, 0);
    /// assert_eq!(pos4, RoomPosition::new(0, 40, w5s5));
    /// ```
    #[inline]
    fn add(self, (x, y): (i32, i32)) -> Self {
        let (wx, wy) = self.world_coords();
        Self::from_world_coords(wx + x, wy + y)
    }
}

impl Sub<(i32, i32)> for RoomPosition {
    type Output = RoomPosition;

    /// See the implementation of `Add<(i32, i32)>` for [`RoomPosition`].
    #[inline]
    fn sub(self, (x, y): (i32, i32)) -> Self {
        self + (-x, -y)
    }
}

impl Sub<RoomPosition> for RoomPosition {
    type Output = (i32, i32);

    /// Subtracts the other room position from this one, extracting the
    /// difference as the output.
    ///
    /// # Example
    ///
    /// ```
    /// use screeps::RoomPosition;
    ///
    /// let e5n5 = "E5N5".parse().unwrap();
    /// let e5n6 = "E5N6".parse().unwrap();
    ///
    /// let pos1 = RoomPosition::new(40, 40, e5n5);
    /// let pos2 = RoomPosition::new(0, 20, e5n6);
    /// assert_eq!(pos1 - pos2, (40, 70));
    /// ```
    #[inline]
    fn sub(self, other: RoomPosition) -> (i32, i32) {
        let (mx, my) = self.world_coords();
        let (ox, oy) = other.world_coords();
        (mx - ox, my - oy)
    }
}
