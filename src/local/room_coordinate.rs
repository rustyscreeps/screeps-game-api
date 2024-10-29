use std::{
    error::Error,
    fmt,
    hint::assert_unchecked,
    ops::{Index, IndexMut, Neg, Sub},
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
#[repr(transparent)]
pub struct RoomCoordinate(u8);

impl RoomCoordinate {
    pub const MAX: Self = Self(ROOM_SIZE - 1);
    pub const MIN: Self = Self(0);

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
    pub fn assume_bounds_constraint(self) {
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
        self.assume_bounds_constraint();
        // Why this works, assuming ROOM_SIZE < i8::MAX + 1 == 128 and ignoring the
        // test:
        //   - if rhs < 0: the smallest value this can produce is -128, which casted to
        //     u8 is 128. The closer rhs is to 0, the larger the cast sum is. So if
        //     ROOM_SIZE <= i8::MAX, any underflow will fail the x < ROOM_SIZE check.
        //   - if rhs > 0: as long as self.0 <= i8::MAX, self.0 + rhs <= 2 * i8::MAX <
        //     256, so there isn't unsigned overflow.
        RoomCoordinate::new(self.0.wrapping_add_signed(rhs)).ok()
    }

    /// [`checked_add`](Self::checked_add) that accepts a [`RoomOffset`].
    pub fn checked_add_offset(self, rhs: RoomOffset) -> Option<RoomCoordinate> {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        RoomCoordinate::new(self.0.wrapping_add_signed(rhs.0)).ok()
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
        self.assume_bounds_constraint();
        let (res, overflow) = self.0.overflowing_add_signed(rhs);
        if overflow {
            RoomCoordinate::MIN
        } else {
            // Optimizer will see the return is always Ok
            RoomCoordinate::new(res.min(ROOM_SIZE - 1)).unwrap_throw()
        }
    }

    /// [`saturating_add`](Self::saturating_add) that accepts a [`RoomOffset`].
    pub fn saturating_add_offset(self, rhs: RoomOffset) -> Self {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        let result = (self.0 as i8 + rhs.0).clamp(0, ROOM_SIZE_I8 - 1);
        RoomCoordinate::new(result as u8).unwrap_throw()
    }

    /// Get the coordinate adjusted by a certain value, wrapping around ta the
    /// edges of the room if the result would be outside of the valid range.
    /// Returns a [`bool`] indicating whether there was wrapping.
    ///
    /// Can be used to e.g. implement addition for
    /// [`Position`](crate::Position)s.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomCoordinate;
    ///
    /// assert_eq!(
    ///     RoomCoordinate::MIN.overflowing_add(1),
    ///     (RoomCoordinate::new(1).unwrap(), false)
    /// );
    /// assert_eq!(
    ///     RoomCoordinate::MIN.overflowing_add(-1),
    ///     (RoomCoordinate::MAX, true)
    /// );
    /// assert_eq!(
    ///     RoomCoordinate::MAX.overflowing_add(1),
    ///     (RoomCoordinate::MIN, true)
    /// );
    /// ```
    pub fn overflowing_add(self, rhs: i8) -> (RoomCoordinate, bool) {
        self.assume_bounds_constraint();
        let raw = self.0 as i16 + rhs as i16;
        if raw >= ROOM_SIZE as i16 {
            (
                RoomCoordinate::new((raw % ROOM_SIZE as i16) as u8).unwrap_throw(),
                true,
            )
        } else if raw < 0 {
            (
                RoomCoordinate::new(((raw + 150) % ROOM_SIZE as i16) as u8).unwrap_throw(),
                true,
            )
        } else {
            (RoomCoordinate::new(raw as u8).unwrap_throw(), false)
        }
    }

    /// [`overflowing_add`](Self::overflowing_add) that accepts a
    /// [`RoomOffset`].
    pub fn overflowing_add_offset(self, rhs: RoomOffset) -> (RoomCoordinate, bool) {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        let raw = self.0 as i8 + rhs.0;
        if raw >= ROOM_SIZE_I8 {
            (
                RoomCoordinate::new((raw - ROOM_SIZE_I8) as u8).unwrap_throw(),
                true,
            )
        } else if raw < 0 {
            (
                RoomCoordinate::new((raw + ROOM_SIZE_I8) as u8).unwrap_throw(),
                true,
            )
        } else {
            (RoomCoordinate::new(raw as u8).unwrap_throw(), false)
        }
    }

    /// Get the coordinate adjusted by a certain value, wrapping around ta the
    /// edges of the room if the result would be outside of the valid range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomCoordinate;
    ///
    /// assert_eq!(
    ///     RoomCoordinate::MIN.wrapping_add(1),
    ///     RoomCoordinate::new(1).unwrap()
    /// );
    /// assert_eq!(RoomCoordinate::MIN.wrapping_add(-1), RoomCoordinate::MAX);
    /// assert_eq!(RoomCoordinate::MAX.wrapping_add(1), RoomCoordinate::MIN);
    /// ```
    pub fn wrapping_add(self, rhs: i8) -> Self {
        self.overflowing_add(rhs).0
    }

    /// [`wrapping_add`](Self::wrapping_add) that accepts a [`RoomOffset`].
    pub fn wrapping_add_offset(self, rhs: RoomOffset) -> Self {
        self.overflowing_add_offset(rhs).0
    }

    /// Get the coordinate adjusted by a certain value.
    ///
    /// # Safety
    ///
    /// After adding rhs to the integer coordinate of self, the result must lie
    /// within `[0, ROOM_SIZE)`.
    pub unsafe fn unchecked_add(self, rhs: i8) -> Self {
        self.assume_bounds_constraint();
        Self::unchecked_new((self.0 as i8).unchecked_add(rhs) as u8)
    }

    /// [`unchecked_add`](Self::unchecked_add) that accepts a [`RoomOffset`].
    ///
    /// # Safety
    ///
    /// The result of adding the integer coordinate of self and the integer
    /// offset in `rhs` must lie within `[0, ROOM_SIZE)`.
    pub unsafe fn unchecked_add_offset(self, rhs: RoomOffset) -> Self {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        Self::unchecked_new((self.0 as i8).unchecked_add(rhs.0) as u8)
    }
}

impl fmt::Display for RoomCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
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

impl AsRef<u8> for RoomCoordinate {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

impl<T> Index<RoomCoordinate> for [T; ROOM_USIZE] {
    type Output = T;

    fn index(&self, index: RoomCoordinate) -> &Self::Output {
        index.assume_bounds_constraint();
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<RoomCoordinate> for [T; ROOM_USIZE] {
    fn index_mut(&mut self, index: RoomCoordinate) -> &mut Self::Output {
        index.assume_bounds_constraint();
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

impl Sub for RoomCoordinate {
    type Output = RoomOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        RoomOffset::new(self.0 as i8 - rhs.0 as i8).unwrap_throw()
    }
}

const ROOM_SIZE_I8: i8 = {
    // If this fails, we need to rework the arithmetic code
    debug_assert!(2 * ROOM_SIZE <= i8::MAX as u8);
    ROOM_SIZE as i8
};

/// An offset between two coordinates in a room. Restricted to the open range
/// (-[`ROOM_SIZE`], [`ROOM_SIZE`]). This bound can be used in safety
/// constraints.
#[derive(
    Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(try_from = "i8", into = "i8")]
#[repr(transparent)]
pub struct RoomOffset(i8);

impl RoomOffset {
    pub const MAX: Self = Self(ROOM_SIZE_I8 - 1);
    pub const MIN: Self = Self(1 - ROOM_SIZE_I8);

    /// Create a `RoomOffset` from an `i8`, returning an error if it's not
    /// within the valid range.
    pub const fn new(offset: i8) -> Result<Self, OffsetOutOfBoundsError> {
        if -ROOM_SIZE_I8 < offset && offset < ROOM_SIZE_I8 {
            Ok(Self(offset))
        } else {
            Err(OffsetOutOfBoundsError(offset))
        }
    }

    /// Create a `RoomOffset` from an `i8`, without checking whether it's in the
    /// range of valid values.
    ///
    /// # Safety
    /// Calling this method with `offset.abs() >= ROOM_SIZE_I8` can result in
    /// undefined behaviour when the resulting `RoomOffset` is used.
    pub unsafe fn unchecked_new(offset: i8) -> Self {
        debug_assert!(
            -ROOM_SIZE_I8 < offset && offset < ROOM_SIZE_I8,
            "Out of bounds unchecked offset: {offset}"
        );
        Self(offset)
    }

    /// Provides a hint to the compiler that the contained `i8` is within
    /// `(-ROOM_SIZE_I8, ROOM_SIZE_I8)`. Allows for better optimized safe code
    /// that uses this property.
    pub fn assume_bounds_constraint(self) {
        debug_assert!(-ROOM_SIZE_I8 < self.0 && self.0 < ROOM_SIZE_I8);
        // SAFETY: It is only safe to construct `RoomOffset` when `-ROOM_SIZE_I8 <
        // self.0 < ROOM_SIZE_I8`.
        unsafe {
            assert_unchecked(-ROOM_SIZE_I8 < self.0 && self.0 < ROOM_SIZE_I8);
        }
    }

    /// Add two offsets together, returning `None` if the result would be
    /// outside the valid range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomOffset;
    ///
    /// let zero = RoomOffset::new(0).unwrap();
    /// let one = RoomOffset::new(1).unwrap();
    ///
    /// assert_eq!(RoomOffset::MIN.checked_add(RoomOffset::MAX), Some(zero));
    /// assert_eq!(RoomOffset::MAX.checked_add(one), None);
    /// assert_eq!(RoomOffset::MIN.checked_add(-one), None);
    /// ```
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        Self::new(self.0 + rhs.0).ok()
    }

    /// Add two offsets together, saturating at the boundaries of the valid
    /// range if the result would be outside.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomOffset;
    ///
    /// let zero = RoomOffset::new(0).unwrap();
    /// let one = RoomOffset::new(1).unwrap();
    ///
    /// assert_eq!(RoomOffset::MIN.saturating_add(RoomOffset::MAX), zero);
    /// assert_eq!(RoomOffset::MAX.saturating_add(one), RoomOffset::MAX);
    /// assert_eq!(RoomOffset::MIN.saturating_add(-one), RoomOffset::MIN);
    /// ```
    pub fn saturating_add(self, rhs: Self) -> Self {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        Self::new((self.0 + rhs.0).clamp(-ROOM_SIZE_I8 + 1, ROOM_SIZE_I8 - 1)).unwrap_throw()
    }

    /// Add two offsets together, wrapping around at the ends of the valid
    /// range. Returns a [`bool`] indicating whether there was wrapping.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomOffset;
    ///
    /// let zero = RoomOffset::new(0).unwrap();
    /// let one = RoomOffset::new(1).unwrap();
    ///
    /// assert_eq!(
    ///     RoomOffset::MAX.overflowing_add(one),
    ///     (RoomOffset::MIN, true)
    /// );
    /// assert_eq!(
    ///     RoomOffset::MIN.overflowing_add(-one),
    ///     (RoomOffset::MAX, true)
    /// );
    /// assert_eq!(
    ///     RoomOffset::MIN.overflowing_add(RoomOffset::MAX),
    ///     (zero, false)
    /// );
    /// ```
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        const RANGE_WIDTH: i8 = 2 * ROOM_SIZE_I8 - 1;
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        let raw = self.0 + rhs.0;
        if raw <= -ROOM_SIZE_I8 {
            (Self::new(raw + RANGE_WIDTH).unwrap_throw(), true)
        } else if raw >= ROOM_SIZE_I8 {
            (Self::new(raw - RANGE_WIDTH).unwrap_throw(), true)
        } else {
            (Self::new(raw).unwrap_throw(), false)
        }
    }

    /// Add two offsets together, wrapping around at the ends of the valid
    /// range.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomOffset;
    ///
    /// let zero = RoomOffset::new(0).unwrap();
    /// let one = RoomOffset::new(1).unwrap();
    ///
    /// assert_eq!(RoomOffset::MAX.wrapping_add(one), RoomOffset::MIN);
    /// assert_eq!(RoomOffset::MIN.wrapping_add(-one), RoomOffset::MAX);
    /// assert_eq!(RoomOffset::MIN.wrapping_add(RoomOffset::MAX), zero);
    /// ```
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    /// Add two offsets together, without checking that the result is in the
    /// valid range.
    ///
    /// # Safety
    ///
    /// The result of adding the two offsets as integers must lie within
    /// `(-ROOM_SIZE_I8, ROOM_SIZE_I8)`.
    pub unsafe fn unchecked_add(self, rhs: Self) -> Self {
        self.assume_bounds_constraint();
        rhs.assume_bounds_constraint();
        Self::unchecked_new(self.0.unchecked_add(rhs.0))
    }

    /// Get the absolute value of the offset.
    ///
    /// Can be used for distance computations, e.g.
    /// ```
    /// use screeps::local::{RoomOffset, RoomXY};
    ///
    /// fn get_movement_distance(a: RoomXY, b: RoomXY) -> u8 {
    ///     (a.x - b.x).abs().max((a.y - b.y).abs())
    /// }
    /// ```
    pub fn abs(self) -> u8 {
        self.assume_bounds_constraint();
        self.0.unsigned_abs()
    }
}

impl From<RoomOffset> for i8 {
    fn from(offset: RoomOffset) -> i8 {
        offset.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OffsetOutOfBoundsError(pub i8);

impl fmt::Display for OffsetOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out of bounds offset: {}", self.0)
    }
}

impl TryFrom<i8> for RoomOffset {
    type Error = OffsetOutOfBoundsError;

    fn try_from(offset: i8) -> Result<Self, Self::Error> {
        Self::new(offset)
    }
}

impl AsRef<i8> for RoomOffset {
    fn as_ref(&self) -> &i8 {
        &self.0
    }
}

impl Neg for RoomOffset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.assume_bounds_constraint();
        Self::new(-self.0).unwrap_throw()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checked_add() {
        for coord_inner in 0..ROOM_SIZE {
            let coord = RoomCoordinate::new(coord_inner).unwrap();
            for rhs in i8::MIN..=i8::MAX {
                let sum = coord.checked_add(rhs);
                assert_eq!(
                    sum.is_some(),
                    (0..ROOM_SIZE as i16).contains(&(coord_inner as i16 + rhs as i16))
                );
                if let Some(res) = sum {
                    assert_eq!(res.u8(), (coord_inner as i16 + rhs as i16) as u8);
                }
            }
        }
    }

    #[test]
    fn saturating_add() {
        for coord_inner in 0..ROOM_SIZE {
            let coord = RoomCoordinate::new(coord_inner).unwrap();
            for rhs in i8::MIN..=i8::MAX {
                assert_eq!(
                    coord.saturating_add(rhs).u8(),
                    (coord_inner as i16 + rhs as i16).clamp(0, ROOM_SIZE as i16 - 1) as u8
                )
            }
        }
    }

    #[test]
    fn index_room_size() {
        let mut base: Box<[u8; ROOM_USIZE]> = (0..50)
            .collect::<Vec<u8>>()
            .into_boxed_slice()
            .try_into()
            .unwrap();
        for i in 0..ROOM_SIZE {
            let coord = RoomCoordinate::new(i).unwrap();
            assert_eq!(base[coord], i);
            base[coord] += 1;
        }
        base.iter()
            .copied()
            .zip(1..(ROOM_SIZE + 1))
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }

    #[test]
    fn index_room_area() {
        let mut base: Box<[u16; ROOM_AREA]> = Box::new([0; ROOM_AREA]);
        for i in 0..ROOM_USIZE {
            for j in 0..ROOM_USIZE {
                base[i * ROOM_USIZE + j] = i as u16 * ROOM_SIZE as u16;
            }
        }

        for i in 0..ROOM_SIZE {
            let coord = RoomCoordinate::new(i).unwrap();
            assert!(base[coord]
                .iter()
                .copied()
                .all(|val| val == i as u16 * ROOM_SIZE as u16));
            for j in 0..ROOM_USIZE {
                base[coord][j] += j as u16;
            }
        }

        assert_eq!(
            (0..ROOM_AREA as u16).collect::<Vec<u16>>().as_slice(),
            base.as_slice()
        );
    }
}
