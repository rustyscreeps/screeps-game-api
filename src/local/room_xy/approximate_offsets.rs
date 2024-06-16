//! Methods related to approximating in-room positions
//! between other in-room positions.

use super::RoomXY;

impl RoomXY {
    /// Calculates an approximate midpoint between this point and the target.
    ///
    /// In case of a tie, rounds towards this point.
    ///
    /// If `distance_towards_target` is bigger than the distance to the target,
    /// the target is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::RoomXY;
    ///
    /// // Exact distances
    /// let start = RoomXY::checked_new(10, 10).unwrap();
    /// let target = RoomXY::checked_new(10, 15).unwrap();
    /// assert_eq!(
    ///     start.towards(target, 1),
    ///     RoomXY::checked_new(10, 11).unwrap()
    /// );
    /// assert_eq!(
    ///     start.towards(target, 4),
    ///     RoomXY::checked_new(10, 14).unwrap()
    /// );
    /// assert_eq!(
    ///     start.towards(target, 10),
    ///     RoomXY::checked_new(10, 15).unwrap()
    /// );
    ///
    /// // Approximate/rounded distances
    /// let start = RoomXY::checked_new(10, 10).unwrap();
    /// let target_1 = RoomXY::checked_new(15, 20).unwrap();
    /// let target_2 = RoomXY::checked_new(0, 5).unwrap();
    /// assert_eq!(
    ///     start.towards(target_1, 1),
    ///     RoomXY::checked_new(10, 11).unwrap()
    /// );
    /// assert_eq!(
    ///     start.towards(target_1, 9),
    ///     RoomXY::checked_new(14, 19).unwrap()
    /// );
    /// assert_eq!(
    ///     start.towards(target_2, 1),
    ///     RoomXY::checked_new(9, 10).unwrap()
    /// );
    /// ```
    pub fn towards(self, target: RoomXY, distance_towards_target: i8) -> RoomXY {
        let (offset_x, offset_y) = target - self;
        let total_distance = offset_x.abs().max(offset_y.abs());
        if distance_towards_target > total_distance {
            return target;
        }

        let new_offset_x = (offset_x * distance_towards_target) / total_distance;
        let new_offset_y = (offset_y * distance_towards_target) / total_distance;

        self + (new_offset_x, new_offset_y)
    }

    /// Calculates an approximate midpoint between this point and the target.
    ///
    /// In case of a tie, rounds towards the target.
    ///
    /// If `distance_from_target` is bigger than the distance to the target,
    /// this position is returned.
    ///
    /// Note: This is essentially the same as [`RoomXY::towards`], just rounding
    ///       towards the target instead of the starting position.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::RoomXY;
    ///
    /// // Exact distances
    /// let start = RoomXY::checked_new(10, 15).unwrap();
    /// let target = RoomXY::checked_new(10, 10).unwrap();
    /// assert_eq!(
    ///     start.between(target, 1),
    ///     RoomXY::checked_new(10, 11).unwrap()
    /// );
    /// assert_eq!(
    ///     start.between(target, 4),
    ///     RoomXY::checked_new(10, 14).unwrap()
    /// );
    /// assert_eq!(
    ///     start.between(target, 10),
    ///     RoomXY::checked_new(10, 15).unwrap()
    /// );
    ///
    /// // Approximate/rounded distances
    /// let start_1 = RoomXY::checked_new(15, 20).unwrap();
    /// let start_2 = RoomXY::checked_new(0, 5).unwrap();
    /// let target = RoomXY::checked_new(10, 10).unwrap();
    /// assert_eq!(
    ///     start_1.between(target, 1),
    ///     RoomXY::checked_new(10, 11).unwrap()
    /// );
    /// assert_eq!(
    ///     start_1.between(target, 9),
    ///     RoomXY::checked_new(14, 19).unwrap()
    /// );
    /// assert_eq!(
    ///     start_2.between(target, 1),
    ///     RoomXY::checked_new(9, 10).unwrap()
    /// );
    /// ```
    pub fn between(self, target: RoomXY, distance_from_target: i8) -> RoomXY {
        target.towards(self, distance_from_target)
    }

    /// Calculates an approximate midpoint between this point and the target.
    ///
    /// In case of a tie, rounds towards the target.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::RoomXY;
    ///
    /// // Exact distances
    /// let start = RoomXY::checked_new(10, 10).unwrap();
    ///
    /// let target_1 = RoomXY::checked_new(10, 16).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_1),
    ///     RoomXY::checked_new(10, 13).unwrap()
    /// );
    ///
    /// let target_2 = RoomXY::checked_new(20, 10).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_2),
    ///     RoomXY::checked_new(15, 10).unwrap()
    /// );
    ///
    /// let target_3 = RoomXY::checked_new(12, 12).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_3),
    ///     RoomXY::checked_new(11, 11).unwrap()
    /// );
    ///
    /// let target_4 = RoomXY::checked_new(4, 4).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_4),
    ///     RoomXY::checked_new(7, 7).unwrap()
    /// );
    ///
    /// // Approximate/rounded distances
    /// let start = RoomXY::checked_new(10, 10).unwrap();
    ///
    /// let target_1 = RoomXY::checked_new(10, 15).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_1),
    ///     RoomXY::checked_new(10, 13).unwrap()
    /// );
    ///
    /// let target_2 = RoomXY::checked_new(19, 10).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_2),
    ///     RoomXY::checked_new(15, 10).unwrap()
    /// );
    ///
    /// let target_3 = RoomXY::checked_new(11, 11).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_3),
    ///     RoomXY::checked_new(11, 11).unwrap()
    /// );
    ///
    /// let target_4 = RoomXY::checked_new(15, 15).unwrap();
    /// assert_eq!(
    ///     start.midpoint_between(target_4),
    ///     RoomXY::checked_new(13, 13).unwrap()
    /// );
    /// ```
    pub fn midpoint_between(self, target: RoomXY) -> RoomXY {
        let (offset_x, offset_y) = self - target;

        let new_offset_x = offset_x / 2;
        let new_offset_y = offset_y / 2;

        target + (new_offset_x, new_offset_y)
    }
}

#[cfg(test)]
mod test {
    use super::RoomXY;

    fn pos(x: u8, y: u8) -> RoomXY {
        RoomXY::checked_new(x, y).unwrap()
    }

    #[test]
    fn towards_accurate() {
        let start = pos(10, 10);
        assert_eq!(start.towards(pos(10, 15), 1), pos(10, 11));
        assert_eq!(start.towards(pos(10, 15), 4), pos(10, 14));
        assert_eq!(start.towards(pos(10, 15), 10), pos(10, 15));
        assert_eq!(start.towards(pos(15, 15), 1), pos(11, 11));
        assert_eq!(start.towards(pos(15, 15), 3), pos(13, 13));
        assert_eq!(start.towards(pos(15, 20), 2), pos(11, 12));
        assert_eq!(start.towards(pos(0, 5), 2), pos(8, 9));
    }
    #[test]
    fn towards_approximate() {
        let start = pos(10, 10);
        assert_eq!(start.towards(pos(15, 20), 1), pos(10, 11));
        assert_eq!(start.towards(pos(15, 20), 9), pos(14, 19));
        assert_eq!(start.towards(pos(0, 5), 1), pos(9, 10));
    }
    #[test]
    fn midpoint_accurate() {
        let start = pos(10, 10);
        assert_eq!(start.midpoint_between(pos(10, 16)), pos(10, 13));
        assert_eq!(start.midpoint_between(pos(20, 10)), pos(15, 10));
        assert_eq!(start.midpoint_between(pos(12, 12)), pos(11, 11));
        assert_eq!(start.midpoint_between(pos(4, 4)), pos(7, 7));
    }
    #[test]
    fn midpoint_approximate() {
        let start = pos(10, 10);
        assert_eq!(start.midpoint_between(pos(10, 15)), pos(10, 13));
        assert_eq!(start.midpoint_between(pos(19, 10)), pos(15, 10));
        assert_eq!(start.midpoint_between(pos(11, 11)), pos(11, 11));
        assert_eq!(start.midpoint_between(pos(15, 15)), pos(13, 13));
        assert_eq!(start.midpoint_between(pos(15, 25)), pos(13, 18));
        assert_eq!(start.midpoint_between(pos(9, 10)), pos(9, 10));
        assert_eq!(start.midpoint_between(pos(7, 10)), pos(8, 10));
        assert_eq!(start.midpoint_between(pos(1, 3)), pos(5, 6));
    }
}
