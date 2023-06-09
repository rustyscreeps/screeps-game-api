//! Utilities for doing math on [`Position`]s which are present in the
//! JavaScript API.
use crate::constants::Direction;

use super::Position;

impl Position {
    /// Gets linear direction to the specified position.
    ///
    /// Note that this chooses between `Top`/`Bottom`/`Left`/`Right` and
    /// `TopLeft`/`TopRight`/`BottomLeft`/`BottomRight` by the magnitude in both
    /// directions. For instance, [`Direction::Top`] can be returned even
    /// if the target has a slightly different `x` coordinate.
    pub fn get_direction_to(self, target: Position) -> Option<Direction> {
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
    /// This operates on positions as "world positions", and will return an
    /// accurate range for positions in different rooms. Note that the
    /// corresponding JavaScript method, `RoomPosition.getRangeTo` returns
    /// `Infinity` if given positions in different rooms.
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::Position;
    /// // (5, 10) in E0N0
    /// let pos_1 = Position::from_world_coords(5, 10);
    /// // (8, 15) in E0N0
    /// let pos_2 = Position::from_world_coords(8, 15);
    /// // The differences are 3 along the X axis and 5 along the Y axis
    /// // so the linear distance is 5.
    /// assert_eq!(pos_1.get_range_to(pos_2), 5);
    /// ```
    #[doc(alias = "distance")]
    #[inline]
    pub fn get_range_to(self, target: Position) -> u32 {
        let (dx, dy) = self - target;
        dx.abs().max(dy.abs()) as u32
    }

    /// Checks whether this position is in the given range of another position.
    ///
    /// Linear range (also called Chebyshev Distance) is an alternate
    /// calculation of distance, calculated as the greater of the distance along
    /// the x axis or the y axis. Most calculations in Screeps use this distance
    /// metric. For more information see [Chebeshev Distance](https://en.wikipedia.org/wiki/Chebyshev_distance).
    ///
    /// This operates on positions as "world positions", and may return true for
    /// positions in different rooms which are still within the given range.
    /// Note that the corresponding JavaScript method, `RoomPosition.inRangeTo`,
    /// will always return `false` for positions from different rooms.
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::Position;
    /// // (5, 10) in E0N0
    /// let pos_1 = Position::from_world_coords(5, 10);
    /// // (8, 10) in E0N0
    /// let pos_2 = Position::from_world_coords(8, 15);
    ///
    /// // The differences are 3 along the X axis and 0 along the Y axis
    /// // so the linear distance is 3.
    /// assert_eq!(pos_1.in_range_to(pos_2, 5), true);
    ///
    /// // (8, 15) in E0N0
    /// let pos_3 = Position::from_world_coords(8, 15);
    ///
    /// // The differences are 3 along the X axis and 5 along the Y axis
    /// // so the linear distance is 5.
    /// // `in_range_to` returns true if the linear distance is equal to the range
    /// assert_eq!(pos_1.in_range_to(pos_3, 5), true);
    ///
    /// // (20, 20) in E0N0
    /// let pos_4 = Position::from_world_coords(20, 20);
    /// // The differences are 15 along the X axis and 10 along the Y axis
    /// // so the linear distance is 15.
    /// assert_eq!(pos_1.in_range_to(pos_4, 5), false);
    /// ```
    #[doc(alias = "distance")]
    #[inline]
    pub fn in_range_to(self, target: Position, range: u32) -> bool {
        self.get_range_to(target) <= range
    }

    /// Checks whether this position is the same as the specified position.
    ///
    /// Note that this is equivalent to `this_pos == target.pos()`.
    #[inline]
    pub fn is_equal_to(self, target: Position) -> bool {
        self == target
    }

    /// True if this position is in the same room as the target, and the range
    /// is at most 1.
    #[inline]
    pub fn is_near_to(self, target: Position) -> bool {
        self.room_name() == target.room_name()
            && (u8::from(self.x()) as i32 - u8::from(target.x()) as i32).abs() <= 1
            && (u8::from(self.y()) as i32 - u8::from(target.y()) as i32).abs() <= 1
    }
}

#[cfg(test)]
mod test {
    use crate::{local::RoomCoordinate, Direction, Position, RoomName};

    #[test]
    fn test_direction_to() {
        let one = unsafe { RoomCoordinate::unchecked_new(1) };
        let two = unsafe { RoomCoordinate::unchecked_new(2) };
        let a = Position::new(one, one, RoomName::from_coords(1, 1).unwrap());
        let b = Position::new(two, two, RoomName::from_coords(1, 1).unwrap());
        assert_eq!(a.get_direction_to(b), Some(Direction::BottomRight));
    }
}
