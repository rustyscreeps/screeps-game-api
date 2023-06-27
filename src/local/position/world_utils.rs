use crate::local::{position::WorldPositionOutOfBoundsError, VALID_WORLD_POSITIONS};

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
    /// For a checked variant of this function, see
    /// [`Position::checked_from_world_coords`].
    ///
    /// See [`Position::world_coords`].
    #[inline]
    #[track_caller]
    pub fn from_world_coords(x: i32, y: i32) -> Self {
        Self::checked_from_world_coords(x, y).unwrap()
    }

    /// Creates a room position from world coords if they are within the range
    /// `-128 * 50 .. +128 * 50`. Otherwise returns `None`.
    ///
    /// For a panicing variant of this function, see
    /// [`Position::from_world_coords`].
    ///
    /// See [`Position::world_coords`].
    #[inline]
    pub fn checked_from_world_coords(
        x: i32,
        y: i32,
    ) -> Result<Self, WorldPositionOutOfBoundsError> {
        if VALID_WORLD_POSITIONS.contains(&x) && VALID_WORLD_POSITIONS.contains(&y) {
            // We do the `HALF_WORLD_SIZE` transition here first so that the division and
            // modulo operations work correctly.
            let pos_x = (x + HALF_WORLD_SIZE * 50) as u32;
            let pos_y = (y + HALF_WORLD_SIZE * 50) as u32;
            let room_x = pos_x / 50;
            let room_y = pos_y / 50;
            let x = (pos_x % 50) as u8;
            let y = (pos_y % 50) as u8;

            Ok(Self::from_coords_and_world_coords_adjusted(
                x, y, room_x, room_y,
            ))
        } else {
            Err(WorldPositionOutOfBoundsError(x, y))
        }
    }
}

#[cfg(test)]
mod test {
    use super::Position;
    use crate::{
        local::{position::WorldPositionOutOfBoundsError, RoomCoordinate},
        ROOM_SIZE,
    };
    use core::ops::Range;

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

    #[test]
    fn checked_world_coords() {
        // this tests:
        // - the 16 rooms around the center of the world
        // - the 16 rooms around each corner of the max world size (12 of them are out
        //   of bounds)

        const ROOM_RANGE: Range<i32> = -((ROOM_SIZE as i32) * 2)..((ROOM_SIZE as i32) * 2);
        for x in ROOM_RANGE {
            for y in ROOM_RANGE {
                let room_x = x.div_euclid(50);
                let room_y = y.div_euclid(50);
                let pos_x = x.rem_euclid(50) as u8;
                let pos_y = y.rem_euclid(50) as u8;

                let new_pos = Position::checked_from_world_coords(x, y).unwrap();
                assert_eq!(room_x, new_pos.room_x());
                assert_eq!(room_y, new_pos.room_y());
                assert_eq!(pos_x, new_pos.x().u8());
                assert_eq!(pos_y, new_pos.y().u8());
            }
        }

        const CORNERS: [(i32, i32); 4] =
            [(-6400, 6399), (6399, 6399), (-6400, -6400), (6399, 6400)];
        for (corner_x, corner_y) in CORNERS {
            for x in ROOM_RANGE {
                for y in ROOM_RANGE {
                    let x = corner_x + x;
                    let y = corner_y + y;

                    if x < -6400 || x > 6399 || y < -6400 || y > 6399 {
                        assert_eq!(
                            Err(WorldPositionOutOfBoundsError(x, y)),
                            Position::checked_from_world_coords(x, y)
                        );
                    } else {
                        let room_x = x.div_euclid(50);
                        let room_y = y.div_euclid(50);
                        let pos_x = x.rem_euclid(50) as u8;
                        let pos_y = y.rem_euclid(50) as u8;

                        let new_pos = Position::checked_from_world_coords(x, y).unwrap();
                        assert_eq!(room_x, new_pos.room_x());
                        assert_eq!(room_y, new_pos.room_y());
                        assert_eq!(pos_x, new_pos.x().u8());
                        assert_eq!(pos_y, new_pos.y().u8());
                    }
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "WorldPositionOutOfBoundsError(6400, 6400)")]
    fn oob_world_coords_panic() {
        // note: world coords are -6400..6400 (not including the end)
        let _val = Position::from_world_coords(6400, 6400);
    }

    #[test]
    #[should_panic(expected = "WorldPositionOutOfBoundsError(6400, 6399)")]
    fn oob_coords_add() {
        let pos = Position::from_world_coords(6395, 6399);
        let _new_pos = pos + (5, 0);
    }

    // don't run this test if debug assertions are enabled, it won't complete
    #[cfg(not(debug_assertions))]
    #[test]
    fn exhaustive_checked_world_coords() {
        use crate::local::VALID_WORLD_POSITIONS;
        // Test that the entire input space returns `Some` or `None` as expected.
        // If this test completes in release mode, it means that the compiler is able to
        // prove enough to optimize it away. If the test stops being instant,
        // something went wrong with the implementation or the compiler (probably the
        // implementation).
        for x in i32::MIN..=i32::MAX {
            for y in i32::MIN..=i32::MAX {
                if VALID_WORLD_POSITIONS.contains(&x) && VALID_WORLD_POSITIONS.contains(&y) {
                    assert!(matches!(Position::checked_from_world_coords(x, y), Ok(_)));
                } else {
                    assert_eq!(
                        Err(WorldPositionOutOfBoundsError(x, y)),
                        Position::checked_from_world_coords(x, y)
                    );
                }
            }
        }
    }
}
