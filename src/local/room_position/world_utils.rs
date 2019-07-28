use super::{LocalRoomPosition, HALF_WORLD_SIZE};

impl LocalRoomPosition {
    /// Returns this position's horizontal "world coordinate".
    ///
    /// The value is equal to `50 * room_x + x`, where `room_x` is defined as
    /// `room_x = -xx - 1` for `Wxx` rooms and as `room_x = xx` for `Exx` rooms.
    #[inline]
    pub fn world_x(self) -> i32 {
        self.room_x() * 50 + (self.x() as i32)
    }

    /// Returns this position's vertical "world coordinate".
    ///
    /// The value is equal to `50 * room_y + y`, where `room_y` is defined as
    /// `room_y = -yy - 1` for `Wyy` rooms and as `room_y = yy` for `Eyy` rooms.
    #[inline]
    pub fn world_y(self) -> i32 {
        self.room_y() * 50 + (self.y() as i32)
    }

    /// Returns this position's "world coordinates".
    ///
    /// The first value is equal to `50 * room_x + x`, where `room_x` is defined
    /// as `room_x = -xx - 1` for `Wxx` rooms and as `room_x = xx` for `Exx`
    /// rooms.
    ///
    /// The second value is equal to `50 * room_y + y`, where `room_y` is
    /// defined as `room_y = -yy - 1` for `Wyy` rooms and as `room_y = yy`
    /// for `Eyy` rooms.
    ///
    /// See also [`LocalRoomPosition::world_x`] and
    /// [`LocalRoomPosition::world_y`].
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
    /// See [`LocalRoomPosition::world_coords`].
    #[inline]
    pub fn from_world_coords(x: i32, y: i32) -> Self {
        // not inclusive since the range for world coords in `-128..=127`, and the range
        // for room coords is `0..=49`.
        assert!(
            (-128 * 50..128 * 50).contains(&x),
            "out of bounds world x: {}",
            x
        );
        assert!(
            (-128 * 50..128 * 50).contains(&y),
            "out of bounds world y: {}",
            y
        );

        let pos_x = (x + HALF_WORLD_SIZE * 50) as u32;
        let pos_y = (y + HALF_WORLD_SIZE * 50) as u32;
        let room_x = pos_x / 50;
        let room_y = pos_y / 50;
        let x = pos_x % 50;
        let y = pos_y % 50;

        LocalRoomPosition {
            packed: (room_x << 24) | (room_y << 16) | (x << 8) | y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::LocalRoomPosition;

    const TEST_ROOM_NAMES: &[&str] = &[
        "E1N1", "E20N0", "W0N0", "E0N0", "W0S0", "E0S0", "W0N0", "E0N0", "W0S0", "E0S0", "W50S20",
        "W127S127", "W127N127", "E127S127", "E127N127",
    ];
    const TEST_COORDS: &[u32] = &[0, 21, 44, 49];

    #[test]
    fn world_coords_round_trip() {
        for room_name in TEST_ROOM_NAMES {
            for x in TEST_COORDS.iter().cloned() {
                for y in TEST_COORDS.iter().cloned() {
                    let original_pos = LocalRoomPosition::new(x, y, room_name.parse().unwrap());
                    let (wx, wy) = original_pos.world_coords();
                    let new = LocalRoomPosition::from_world_coords(wx, wy);
                    assert_eq!(original_pos, new);
                }
            }
        }
    }
}
