use super::{Position, HALF_WORLD_SIZE};

impl Position {
    /// Returns this position's horizontal "world coordinate".
    ///
    /// The value is equal to `50 * room_x + x`, where `room_x` is defined as
    /// `room_x = -xx - 1` for `Wxx` rooms and as `room_x = xx` for `Exx` rooms.
    #[inline]
    pub fn world_x(self) -> i32 {
        self.room_x() * 50 + (u8::from(self.x()) as i32)
    }

    /// Returns this position's vertical "world coordinate".
    ///
    /// The value is equal to `50 * room_y + y`, where `room_y` is defined as
    /// `room_y = -yy - 1` for `Nyy` rooms and as `room_y = yy` for `Syy` rooms.
    #[inline]
    pub fn world_y(self) -> i32 {
        self.room_y() * 50 + (u8::from(self.y()) as i32)
    }

    /// Returns this position's "world coordinates".
    ///
    /// The first value is equal to `50 * room_x + x`, where `room_x` is defined
    /// as `room_x = -xx - 1` for `Wxx` rooms and as `room_x = xx` for `Exx`
    /// rooms.
    ///
    /// The second value is equal to `50 * room_y + y`, where `room_y` is
    /// defined as `room_y = -yy - 1` for `Nyy` rooms and as `room_y = yy`
    /// for `Syy` rooms.
    ///
    /// See also [`Position::world_x`] and
    /// [`Position::world_y`].
    #[inline]
    pub fn world_coords(self) -> (i32, i32) {
        (self.world_x(), self.world_y())
    }

    /// Creates a room position from world coords.
    ///
    /// # Panics
    ///
    /// Panics if either x or y is out of the range `-128 * 50 .. +128 * 50`.
    ///
    /// See [`Position::world_coords`].
    #[inline]
    pub fn from_world_coords(x: i32, y: i32) -> Self {
        // not inclusive since the range for world coords in `-128..=127`, and the range
        // for room coords is `0..=49`.
        assert!(
            (-HALF_WORLD_SIZE * 50..HALF_WORLD_SIZE * 50).contains(&x),
            "out of bounds world x: {}",
            x
        );
        assert!(
            (-HALF_WORLD_SIZE * 50..HALF_WORLD_SIZE * 50).contains(&y),
            "out of bounds world y: {}",
            y
        );

        // We do the `HALF_WORLD_SIZE` transition here first so that the division and
        // modulo operations work correctly.
        let pos_x = (x + HALF_WORLD_SIZE * 50) as u32;
        let pos_y = (y + HALF_WORLD_SIZE * 50) as u32;
        let room_x = pos_x / 50;
        let room_y = pos_y / 50;
        let x = (pos_x % 50) as u8;
        let y = (pos_y % 50) as u8;

        Self::from_coords_and_world_coords_adjusted(x, y, room_x, room_y)
    }
}

#[cfg(test)]
mod test {
    use super::Position;
    use crate::local::RoomCoordinate;

    const TEST_ROOM_NAMES: &[&str] = &[
        "E1N1", "E20N0", "W0N0", "E0N0", "W0S0", "E0S0", "W0N0", "E0N0", "W0S0", "E0S0", "W50S20",
        "W127S127", "W127N127", "E127S127", "E127N127",
    ];

    fn gen_test_coords() -> [RoomCoordinate; 4] {
        unsafe {
            [
                RoomCoordinate::unchecked_new(0),
                RoomCoordinate::unchecked_new(21),
                RoomCoordinate::unchecked_new(44),
                RoomCoordinate::unchecked_new(49),
            ]
        }
    }

    #[test]
    fn world_coords_round_trip() {
        for room_name in TEST_ROOM_NAMES {
            for x in gen_test_coords().iter().cloned() {
                for y in gen_test_coords().iter().cloned() {
                    let original_pos = Position::new(x, y, room_name.parse().unwrap());
                    let (wx, wy) = original_pos.world_coords();
                    let new = Position::from_world_coords(wx, wy);
                    assert_eq!(original_pos, new);
                }
            }
        }
    }
}
