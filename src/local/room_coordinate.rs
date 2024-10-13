use std::{
    error::Error,
    fmt,
    hint::assert_unchecked,
    ops::{Index, IndexMut},
};

use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;

use crate::constants::{ROOM_AREA, ROOM_SIZE, ROOM_USIZE};

#[derive(Debug, Clone, Copy)]
pub struct OutOfBoundsError(pub u8);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds coordinate: {}", self.0)
    }
}

impl Error for OutOfBoundsError {}

/// An X or Y coordinate in a room, restricted to the valid range of
/// coordinates. This restriction can be used in safety constraints, and is
/// enforced by all safe `RoomCoordinate` constructors.
#[derive(
    Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(try_from = "u8", into = "u8")]
pub struct RoomCoordinate(u8);

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

    /// Provides a hint to the compiler that the contained `u8` is smaller than
    /// `ROOM_SIZE`. Allows for better optimized safe code that uses this
    /// property.
    pub fn assume_size_constraint(self) {
        debug_assert!(self.0 < ROOM_SIZE);
        // SAFETY: It is only safe to construct `RoomCoordinate` when self.0 <
        // ROOM_SIZE.
        unsafe {
            assert_unchecked(self.0 < ROOM_SIZE);
        }
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
        self.assume_size_constraint();
        RoomCoordinate::new(self.0.checked_add_signed(rhs)?).ok()
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
        self.assume_size_constraint();
        let result = self.0.saturating_add_signed(rhs).min(ROOM_SIZE - 1);
        // Optimizer will see the return is always Ok
        RoomCoordinate::new(result).unwrap_throw()
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

impl<T> Index<RoomCoordinate> for [T; ROOM_USIZE] {
    type Output = T;

    fn index(&self, index: RoomCoordinate) -> &Self::Output {
        index.assume_size_constraint();
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<RoomCoordinate> for [T; ROOM_USIZE] {
    fn index_mut(&mut self, index: RoomCoordinate) -> &mut Self::Output {
        index.assume_size_constraint();
        &mut self[index.0 as usize]
    }
}

impl<T> Index<RoomCoordinate> for [T; ROOM_AREA] {
    type Output = [T; ROOM_USIZE];

    fn index(&self, index: RoomCoordinate) -> &Self::Output {
        // SAFETY: ROOM_USIZE * ROOM_USIZE = ROOM_AREA, so [T; ROOM_AREA] and [[T;
        // ROOM_USIZE]; ROOM_USIZE] have the same layout.
        let this =
            unsafe { &*(self as *const [T; ROOM_AREA] as *const [[T; ROOM_USIZE]; ROOM_USIZE]) };
        &this[index]
    }
}

impl<T> IndexMut<RoomCoordinate> for [T; ROOM_AREA] {
    fn index_mut(&mut self, index: RoomCoordinate) -> &mut Self::Output {
        // SAFETY: ROOM_USIZE * ROOM_USIZE = ROOM_AREA, so [T; ROOM_AREA] and [[T;
        // ROOM_USIZE]; ROOM_USIZE] have the same layout.
        let this =
            unsafe { &mut *(self as *mut [T; ROOM_AREA] as *mut [[T; ROOM_USIZE]; ROOM_USIZE]) };
        &mut this[index]
    }
}
