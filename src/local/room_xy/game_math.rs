//! Utilities for doing math on [`RoomXY`]s which are present in the
//! JavaScript API.

use crate::constants::Direction;

use super::RoomXY;

impl RoomXY {
    /// Gets linear direction to the specified position.
    ///
    /// Note that this chooses between `Top`/`Bottom`/`Left`/`Right` and
    /// `TopLeft`/`TopRight`/`BottomLeft`/`BottomRight` by the magnitude in both
    /// directions. For instance, [`Direction::Top`] can be returned even
    /// if the target has a slightly different `x` coordinate.
    pub fn get_direction_to(self, target: RoomXY) -> Option<Direction> {
        // Logic copied from https://github.com/screeps/engine/blob/020ba168a1fde9a8072f9f1c329d5c0be8b440d7/src/utils.js#L73-L107
        let (dx, dy) = target - self;
        if dx.abs() > dy.abs() * 2 {
            if dx > 0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else if dy.abs() > dx.abs() * 2 {
            if dy > 0 {
                Some(Direction::Bottom)
            } else {
                Some(Direction::Top)
            }
        } else if dx > 0 && dy > 0 {
            Some(Direction::BottomRight)
        } else if dx > 0 && dy < 0 {
            Some(Direction::TopRight)
        } else if dx < 0 && dy > 0 {
            Some(Direction::BottomLeft)
        } else if dx < 0 && dy < 0 {
            Some(Direction::TopLeft)
        } else {
            None
        }
    }

    /// Gets linear range to the specified position.
    ///
    /// Linear range (also called Chebyshev Distance) is an alternate
    /// calculation of distance, calculated as the greater of the distance along
    /// the x axis or the y axis. Most calculations in Screeps use this distance
    /// metric. For more information see [Chebeshev Distance](https://en.wikipedia.org/wiki/Chebyshev_distance).
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::RoomXY;
    /// let pos_1 = RoomXY::checked_new(5, 10).unwrap();
    /// let pos_2 = RoomXY::checked_new(8, 15).unwrap();
    /// // The differences are 3 along the X axis and 5 along the Y axis
    /// // so the linear distance is 5.
    /// assert_eq!(pos_1.get_range_to(pos_2), 5);
    /// ```
    #[doc(alias = "distance")]
    #[inline]
    pub fn get_range_to(self, target: RoomXY) -> u8 {
        let (dx, dy) = self - target;
        dx.unsigned_abs().max(dy.unsigned_abs())
    }

    /// Checks whether this position is in the given range of another position.
    ///
    /// Linear range (also called Chebyshev Distance) is an alternate
    /// calculation of distance, calculated as the greater of the distance along
    /// the x axis or the y axis. Most calculations in Screeps use this distance
    /// metric. For more information see [Chebeshev Distance](https://en.wikipedia.org/wiki/Chebyshev_distance).
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::RoomXY;
    /// let pos_1 = RoomXY::checked_new(5, 10).unwrap();
    /// let pos_2 = RoomXY::checked_new(8, 10).unwrap();
    ///
    /// // The differences are 3 along the X axis and 0 along the Y axis
    /// // so the linear distance is 3.
    /// assert_eq!(pos_1.in_range_to(pos_2, 5), true);
    ///
    /// let pos_3 = RoomXY::checked_new(8, 15).unwrap();
    ///
    /// // The differences are 3 along the X axis and 5 along the Y axis
    /// // so the linear distance is 5.
    /// // `in_range_to` returns true if the linear distance is equal to the range
    /// assert_eq!(pos_1.in_range_to(pos_3, 5), true);
    ///
    /// let pos_4 = RoomXY::checked_new(20, 20).unwrap();
    /// // The differences are 15 along the X axis and 10 along the Y axis
    /// // so the linear distance is 15.
    /// assert_eq!(pos_1.in_range_to(pos_4, 5), false);
    /// assert_eq!(pos_1.in_range_to(pos_4, 10), false);
    /// assert_eq!(pos_1.in_range_to(pos_4, 15), true);
    /// ```
    #[doc(alias = "distance")]
    #[inline]
    pub fn in_range_to(self, target: RoomXY, range: u8) -> bool {
        self.get_range_to(target) <= range
    }

    /// Checks whether this position is the same as the specified position.
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::RoomXY;
    /// let pos_1 = RoomXY::checked_new(5, 10).unwrap();
    /// let pos_2 = RoomXY::checked_new(5, 10).unwrap();
    /// let pos_3 = RoomXY::checked_new(4, 9).unwrap();
    ///
    /// assert_eq!(pos_1.is_equal_to(pos_2), true);
    /// assert_eq!(pos_1.is_equal_to(pos_3), false);
    /// ```
    #[inline]
    pub fn is_equal_to(self, target: RoomXY) -> bool {
        self == target
    }

    /// True if the range from this position to the target is at most 1.
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::RoomXY;
    /// let pos_1 = RoomXY::checked_new(5, 10).unwrap();
    /// let pos_2 = RoomXY::checked_new(6, 10).unwrap();
    /// let pos_3 = RoomXY::checked_new(4, 9).unwrap();
    /// let pos_4 = RoomXY::checked_new(20, 20).unwrap();
    ///
    /// assert_eq!(pos_1.is_near_to(pos_2), true);
    /// assert_eq!(pos_1.is_near_to(pos_3), true);
    /// assert_eq!(pos_1.is_near_to(pos_4), false);
    /// ```
    #[inline]
    pub fn is_near_to(self, target: RoomXY) -> bool {
        (u8::from(self.x) as i32 - u8::from(target.x) as i32).abs() <= 1
            && (u8::from(self.y) as i32 - u8::from(target.y) as i32).abs() <= 1
    }
}

#[cfg(test)]
mod test {
    use crate::{Direction, RoomXY};

    #[test]
    fn test_direction_to() {
        let a = RoomXY::checked_new(1, 1).unwrap();
        let b = RoomXY::checked_new(2, 2).unwrap();
        assert_eq!(a.get_direction_to(b), Some(Direction::BottomRight));
    }
}
