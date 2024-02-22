use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::constants::ROOM_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError(pub u8);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds coordinate: {}", self.0)
    }
}

impl Error for OutOfBoundsError {}

/// An X or Y coordinate in a room, restricted to the valid range of
/// coordinates.
#[derive(
    Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(try_from = "u8", into = "u8")]
pub struct RoomCoordinate(pub u8);

impl RoomCoordinate {
    /// Create a `RoomCoordinate` from a `u8`, returning an error if the
    /// coordinate is not in the valid room size range
    #[inline]
    pub const fn new(coord: u8) -> Result<Self, OutOfBoundsError> {
        if coord < ROOM_SIZE {
            Ok(RoomCoordinate(coord))
        } else {
            Err(OutOfBoundsError(coord))
        }
    }

    /// Create a `RoomCoordinate` from a `u8`, without checking whether it's in
    /// the range of valid values.
    ///
    /// # Safety
    /// Calling this method with `coord >= ROOM_SIZE` can result in undefined
    /// behaviour when the resulting `RoomCoordinate` is used.
    #[inline]
    pub unsafe fn unchecked_new(coord: u8) -> Self {
        debug_assert!(
            coord < ROOM_SIZE,
            "Out of bounds unchecked coordinate: {coord}"
        );
        RoomCoordinate(coord)
    }

    /// Get the integer value of this coordinate
    pub const fn u8(self) -> u8 {
        self.0
    }

    /// Get whether this coordinate represents an edge position (0 or 49)
    pub const fn is_room_edge(self) -> bool {
        self.0 == 0 || self.0 == ROOM_SIZE - 1
    }

    /// Get the coordinate adjusted by a certain value, returning `None` if the
    /// result is outside the valid range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomCoordinate;
    ///
    /// let zero = RoomCoordinate::new(0).unwrap();
    /// let forty_nine = RoomCoordinate::new(49).unwrap();
    ///
    /// assert_eq!(zero.checked_add(1), Some(RoomCoordinate::new(1).unwrap()));
    /// assert_eq!(zero.checked_add(-1), None);
    /// assert_eq!(zero.checked_add(49), Some(forty_nine));
    /// assert_eq!(forty_nine.checked_add(1), None);
    /// ```
    pub fn checked_add(self, rhs: i8) -> Option<RoomCoordinate> {
        match (self.0 as i8).checked_add(rhs) {
            Some(result) => match result {
                // less than 0
                i8::MIN..=-1 => None,
                // greater than 49
                50..=i8::MAX => None,
                // SAFETY: we've checked that this coord is in the valid range
                c => Some(unsafe { RoomCoordinate::unchecked_new(c as u8) }),
            },
            None => None,
        }
    }

    /// Get the coordinate adjusted by a certain value, saturating at the edges
    /// of the room if the result would be outside of the valid range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomCoordinate;
    ///
    /// let zero = RoomCoordinate::new(0).unwrap();
    /// let forty_nine = RoomCoordinate::new(49).unwrap();
    ///
    /// assert_eq!(zero.saturating_add(1), RoomCoordinate::new(1).unwrap());
    /// assert_eq!(zero.saturating_add(-1), zero);
    /// assert_eq!(zero.saturating_add(i8::MAX), forty_nine);
    /// assert_eq!(forty_nine.saturating_add(1), forty_nine);
    /// assert_eq!(forty_nine.saturating_add(i8::MIN), zero);
    /// ```
    pub fn saturating_add(self, rhs: i8) -> RoomCoordinate {
        let result = match (self.0 as i8).saturating_add(rhs) {
            // less than 0, saturate to 0
            i8::MIN..=-1 => 0,
            // greater than 49, saturate to 49
            50..=i8::MAX => ROOM_SIZE - 1,
            c => c as u8,
        };
        // SAFETY: we've ensured that this coord is in the valid range
        unsafe { RoomCoordinate::unchecked_new(result) }
    }
}

impl fmt::Display for RoomCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RoomCoordinate> for u8 {
    fn from(coord: RoomCoordinate) -> u8 {
        coord.0
    }
}

impl TryFrom<u8> for RoomCoordinate {
    type Error = OutOfBoundsError;

    fn try_from(coord: u8) -> Result<Self, Self::Error> {
        RoomCoordinate::new(coord)
    }
}
