use std::{
    cmp::Ordering,
    fmt,
    ops::{Index, IndexMut, Sub},
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::UnwrapThrowExt;

use super::{
    room_coordinate::{OutOfBoundsError, RoomCoordinate, RoomOffset},
    OffsetOutOfBoundsError,
};
use crate::constants::{Direction, ROOM_AREA, ROOM_USIZE};

mod approximate_offsets;
mod extra_math;
mod game_math;

/// Converts a [`RoomXY`] coordinate pair to a linear index appropriate for use
/// with the internal representation of a [`CostMatrix`] or [`LocalCostMatrix`].
///
/// [`CostMatrix`]: crate::objects::CostMatrix
/// [`LocalCostMatrix`]: crate::local::LocalCostMatrix
#[inline]
pub const fn xy_to_linear_index(xy: RoomXY) -> usize {
    xy.x.u8() as usize * ROOM_USIZE + xy.y.u8() as usize
}

/// Converts a linear index from the internal representation of a [`CostMatrix`]
/// or [`LocalCostMatrix`] to a [`RoomXY`] coordinate pair for the position the
/// index represents.
///
/// [`CostMatrix`]: crate::objects::CostMatrix
/// [`LocalCostMatrix`]: crate::local::LocalCostMatrix
#[inline]
pub fn linear_index_to_xy(idx: usize) -> RoomXY {
    assert!(idx < ROOM_AREA, "Out of bounds index: {idx}");
    // SAFETY: bounds checking above ensures both are within range.
    RoomXY {
        x: unsafe { RoomCoordinate::unchecked_new((idx / (ROOM_USIZE)) as u8) },
        y: unsafe { RoomCoordinate::unchecked_new((idx % (ROOM_USIZE)) as u8) },
    }
}

/// Converts a [`RoomXY`] coordinate pair to a terrain index appropriate for use
/// with the internal representation of [`RoomTerrain`] or [`LocalRoomTerrain`].
///
/// [`RoomTerrain`]: crate::objects::RoomTerrain
/// [`LocalRoomTerrain`]: crate::local::LocalRoomTerrain
#[inline]
pub const fn xy_to_terrain_index(xy: RoomXY) -> usize {
    xy.y.u8() as usize * ROOM_USIZE + xy.x.u8() as usize
}

/// Converts a terrain index from the internal representation of a
/// [`RoomTerrain`] or [`LocalRoomTerrain`] to a [`RoomXY`] coordinate pair for
/// the position the index represents.
///
/// [`RoomTerrain`]: crate::objects::RoomTerrain
/// [`LocalRoomTerrain`]: crate::local::LocalRoomTerrain
#[inline]
pub fn terrain_index_to_xy(idx: usize) -> RoomXY {
    assert!(idx < ROOM_AREA, "Out of bounds index: {idx}");
    // SAFETY: bounds checking above ensures both are within range.
    RoomXY {
        x: unsafe { RoomCoordinate::unchecked_new((idx % (ROOM_USIZE)) as u8) },
        y: unsafe { RoomCoordinate::unchecked_new((idx / (ROOM_USIZE)) as u8) },
    }
}

/// A generic x-y pair of values.
#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

/// An X/Y pair representing a given coordinate relative to any room.
pub type RoomXY = XY<RoomCoordinate>;

/// An X/Y pair representing a given offset relative to any room.
pub type RoomOffsetXY = XY<RoomOffset>;

impl<T> XY<T> {
    /// Create a new `XY<T>` from a pair of `T`s.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Try to create a new `XY` from a pair of convertable values.
    #[inline]
    pub fn checked_new<U>(x: U, y: U) -> Result<Self, <Self as TryFrom<(U, U)>>::Error>
    where
        Self: TryFrom<(U, U)>,
    {
        Self::try_from((x, y))
    }
}

impl RoomXY {
    /// Create a `RoomXY` from a pair of `u8`, without checking whether it's in
    /// the range of valid values.
    ///
    /// # Safety
    /// Calling this method with `x >= ROOM_SIZE` or `y >= ROOM_SIZE` can
    /// result in undefined behaviour when the resulting `RoomXY` is used.
    #[inline]
    pub unsafe fn unchecked_new(x: u8, y: u8) -> Self {
        RoomXY {
            x: RoomCoordinate::unchecked_new(x),
            y: RoomCoordinate::unchecked_new(y),
        }
    }

    /// Get whether this coordinate pair represents an edge position (0 or 49
    /// for either coordinate)
    pub const fn is_room_edge(self) -> bool {
        self.x.is_room_edge() || self.y.is_room_edge()
    }

    /// Get the coordinate adjusted by a certain value, returning `None` if the
    /// result is outside the valid room area.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomXY;
    ///
    /// let zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let one = unsafe { RoomXY::unchecked_new(1, 1) };
    /// let forty_nine = unsafe { RoomXY::unchecked_new(49, 49) };
    ///
    /// assert_eq!(zero.checked_add((1, 1)), Some(one));
    /// assert_eq!(zero.checked_add((-1, 0)), None);
    /// assert_eq!(zero.checked_add((49, 49)), Some(forty_nine));
    /// assert_eq!(forty_nine.checked_add((1, 1)), None);
    /// ```
    pub fn checked_add(self, rhs: (i8, i8)) -> Option<RoomXY> {
        let x = self.x.checked_add(rhs.0)?;
        let y = self.y.checked_add(rhs.1)?;
        Some(RoomXY { x, y })
    }

    pub fn checked_add_offset(self, rhs: RoomOffsetXY) -> Option<Self> {
        let x = self.x.checked_add_offset(rhs.x)?;
        let y = self.y.checked_add_offset(rhs.y)?;
        Some(Self { x, y })
    }

    /// Get the coordinate adjusted by a certain value, saturating at the edges
    /// of the room if the result would be outside the valid room area.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomXY;
    ///
    /// let zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let one = unsafe { RoomXY::unchecked_new(1, 1) };
    /// let forty_nine = unsafe { RoomXY::unchecked_new(49, 49) };
    ///
    /// assert_eq!(zero.saturating_add((1, 1)), one);
    /// assert_eq!(zero.saturating_add((-1, 0)), zero);
    /// assert_eq!(zero.saturating_add((49, 49)), forty_nine);
    /// assert_eq!(zero.saturating_add((i8::MAX, i8::MAX)), forty_nine);
    /// assert_eq!(forty_nine.saturating_add((1, 1)), forty_nine);
    /// assert_eq!(forty_nine.saturating_add((i8::MIN, i8::MIN)), zero);
    /// ```
    pub fn saturating_add(self, rhs: (i8, i8)) -> RoomXY {
        let x = self.x.saturating_add(rhs.0);
        let y = self.y.saturating_add(rhs.1);
        RoomXY { x, y }
    }

    pub fn saturating_add_offset(self, rhs: RoomOffsetXY) -> Self {
        let x = self.x.saturating_add_offset(rhs.x);
        let y = self.y.saturating_add_offset(rhs.y);
        Self { x, y }
    }

    pub fn overflowing_add(self, rhs: (i8, i8)) -> (Self, (bool, bool)) {
        let (x, x_overflow) = self.x.overflowing_add(rhs.0);
        let (y, y_overflow) = self.y.overflowing_add(rhs.1);
        (Self { x, y }, (x_overflow, y_overflow))
    }

    pub fn overflowing_add_offset(self, rhs: RoomOffsetXY) -> (Self, XY<bool>) {
        let (x, x_overflow) = self.x.overflowing_add_offset(rhs.x);
        let (y, y_overflow) = self.y.overflowing_add_offset(rhs.y);
        (
            Self { x, y },
            XY {
                x: x_overflow,
                y: y_overflow,
            },
        )
    }

    pub fn wrapping_add(self, rhs: (i8, i8)) -> Self {
        self.overflowing_add(rhs).0
    }

    pub fn wrapping_add_offset(self, rhs: RoomOffsetXY) -> Self {
        self.overflowing_add_offset(rhs).0
    }

    pub unsafe fn unchecked_add(self, rhs: (i8, i8)) -> Self {
        let x = self.x.unchecked_add(rhs.0);
        let y = self.y.unchecked_add(rhs.1);
        Self { x, y }
    }

    pub unsafe fn unchecked_add_offset(self, rhs: RoomOffsetXY) -> Self {
        let x = self.x.unchecked_add_offset(rhs.x);
        let y = self.y.unchecked_add_offset(rhs.y);
        Self { x, y }
    }

    /// Get all the valid neighbors of a given `RoomXY`.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::local::RoomXY;
    ///
    /// let zero_zero = unsafe { RoomXY::unchecked_new(0, 0) };
    /// let zero_one = unsafe { RoomXY::unchecked_new(0, 1) };
    /// let one_zero = unsafe { RoomXY::unchecked_new(1, 0) };
    /// let one_one = unsafe { RoomXY::unchecked_new(1, 1) };
    ///
    /// let zero_two = unsafe { RoomXY::unchecked_new(0, 2) };
    /// let one_two = unsafe { RoomXY::unchecked_new(1, 2) };
    /// let two_two = unsafe { RoomXY::unchecked_new(2, 2) };
    /// let two_one = unsafe { RoomXY::unchecked_new(2, 1) };
    /// let two_zero = unsafe { RoomXY::unchecked_new(2, 0) };
    ///
    /// let zero_zero_neighbors = zero_zero.neighbors();
    ///
    /// assert_eq!(zero_zero_neighbors.len(), 3);
    /// assert!(zero_zero_neighbors.contains(&zero_one));
    /// assert!(zero_zero_neighbors.contains(&one_one));
    /// assert!(zero_zero_neighbors.contains(&one_zero));
    ///
    /// let one_one_neighbors = one_one.neighbors();
    ///
    /// assert_eq!(one_one_neighbors.len(), 8);
    /// assert!(one_one_neighbors.contains(&zero_zero));
    /// assert!(one_one_neighbors.contains(&zero_one));
    /// assert!(one_one_neighbors.contains(&one_zero));
    /// assert!(one_one_neighbors.contains(&zero_two));
    /// assert!(one_one_neighbors.contains(&one_two));
    /// assert!(one_one_neighbors.contains(&two_two));
    /// assert!(one_one_neighbors.contains(&two_one));
    /// assert!(one_one_neighbors.contains(&two_zero));
    /// ```
    pub fn neighbors(self) -> Vec<RoomXY> {
        Direction::iter()
            .filter_map(|&dir| self.checked_add_offset(dir.into()))
            .collect()
    }
}

impl PartialOrd for RoomXY {
    #[inline]
    fn partial_cmp(&self, other: &RoomXY) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoomXY {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl fmt::Display for RoomXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl RoomOffsetXY {
    #[inline]
    pub unsafe fn unchecked_new(x: i8, y: i8) -> Self {
        Self {
            x: RoomOffset::unchecked_new(x),
            y: RoomOffset::unchecked_new(y),
        }
    }

    pub fn manhattan_distance(self) -> u8 {
        self.x.abs() + self.y.abs()
    }

    pub fn chebyshev_distance(self) -> u8 {
        self.x.abs().max(self.y.abs())
    }
}

impl std::ops::Neg for RoomOffsetXY {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for RoomOffsetXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T, U> From<XY<T>> for (U, U)
where
    U: From<T>,
{
    fn from(XY { x, y }: XY<T>) -> Self {
        (x.into(), y.into())
    }
}

impl<T, U> From<(U, U)> for XY<T>
where
    T: From<U>,
{
    fn from((x, y): (U, U)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<A, B> Sub<XY<B>> for XY<A>
where
    A: Sub<B>,
{
    type Output = XY<A::Output>;

    /// Implements subtraction between [`XY<T>`] values when the contained types can be subtracted from each other as scalars.
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::{RoomXY, XY, RoomOffsetXY};
    ///
    /// assert_eq!(XY {x: 5, y: 4} - XY {x: 1, y: 1}, XY {x: 4, y: 3});
    /// let pos1 = RoomXY::checked_new(40, 40).unwrap();
    /// let pos2 = RoomXY::checked_new(0, 20).unwrap();
    /// assert_eq!(pos1 - pos2, RoomOffsetXY::checked_new(40, 20).unwrap());
    ///
    /// let pos3 = RoomXY::checked_new(45, 45).unwrap();
    /// assert_eq!(pos1 - pos3, RoomOffsetXY::checked_new(-5, -5).unwrap());
    /// ```
    fn sub(self, rhs: XY<B>) -> XY<A::Output> {
        XY {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl TryFrom<(u8, u8)> for RoomXY {
    type Error = OutOfBoundsError;

    fn try_from((x, y): (u8, u8)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl TryFrom<(i8, i8)> for RoomOffsetXY {
    type Error = OffsetOutOfBoundsError;

    fn try_from((x, y): (i8, i8)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl From<Direction> for RoomOffsetXY {
    fn from(value: Direction) -> Self {
        use Direction::*;
        let y = match value {
            Top | TopLeft | TopRight => RoomOffset::new(-1),
            Right | Left => RoomOffset::new(0),
            Bottom | BottomLeft | BottomRight => RoomOffset::new(1),
        }
        .unwrap_throw();
        let x = match value {
            Left | TopLeft | BottomLeft => RoomOffset::new(-1),
            Top | Bottom => RoomOffset::new(0),
            Right | TopRight | BottomRight => RoomOffset::new(1),
        }
        .unwrap_throw();
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize)]
struct ReadableXY<T> {
    x: T,
    y: T,
}

impl<T> From<ReadableXY<T>> for XY<T> {
    fn from(ReadableXY { x, y }: ReadableXY<T>) -> XY<T> {
        Self { x, y }
    }
}

impl<T> From<XY<T>> for ReadableXY<T> {
    fn from(XY { x, y }: XY<T>) -> Self {
        Self { x, y }
    }
}

impl Serialize for RoomXY {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            ReadableXY::<RoomCoordinate>::from(*self).serialize(serializer)
        } else {
            let xy: (u8, u8) = (*self).into();
            let packed: u16 = ((xy.0 as u16) << 8) | (xy.1 as u16);
            packed.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for RoomXY {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            ReadableXY::<RoomCoordinate>::deserialize(deserializer).map(Into::into)
        } else {
            let packed = u16::deserialize(deserializer)?;
            let xy = (((packed >> 8) & 0xFF) as u8, (packed & 0xFF) as u8);
            RoomXY::try_from(xy).map_err(|err: OutOfBoundsError| {
                de::Error::invalid_value(
                    de::Unexpected::Unsigned(err.0 as u64),
                    &format!("a non-negative integer less-than {ROOM_USIZE}").as_str(),
                )
            })
        }
    }
}

impl Serialize for RoomOffsetXY {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            ReadableXY::<RoomOffset>::from(*self).serialize(serializer)
        } else {
            let xy: (i8, i8) = (*self).into();
            let packed: u16 = ((xy.0 as u8 as u16) << 8) | (xy.1 as u8 as u16);
            packed.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for RoomOffsetXY {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            ReadableXY::<RoomOffset>::deserialize(deserializer).map(Into::into)
        } else {
            let packed = u16::deserialize(deserializer)?;
            let xy = (
                ((packed >> 8) & 0xFF) as u8 as i8,
                (packed & 0xFF) as u8 as i8,
            );
            RoomOffsetXY::try_from(xy).map_err(|err| {
                de::Error::invalid_value(
                    de::Unexpected::Signed(err.0 as i64),
                    &format!("an integer with absolute value less-than {ROOM_USIZE}").as_str(),
                )
            })
        }
    }
}

/// A wrapper struct indicating that the inner array should be indexed X major,
/// i.e. ```
/// use screeps::{
///     constants::ROOM_USIZE,
///     local::{XMajor, XY},
/// };
///
/// let mut x_major = XMajor([[0_u8; ROOM_USIZE]; ROOM_USIZE]);
/// x_major.0[10][0] = 1;
/// let xy = RoomXY::checked_new(10, 0).unwrap();
/// assert_eq!(x_major[xy], 1);
/// ```
#[repr(transparent)]
pub struct XMajor<T>(pub [[T; ROOM_USIZE]; ROOM_USIZE]);

impl<T> XMajor<T> {
    pub fn from_ref(arr: &[[T; ROOM_USIZE]; ROOM_USIZE]) -> &Self {
        // SAFETY: XMajor is a repr(transparent) wrapper around [[T; ROOM_USIZE];
        // ROOM_USIZE], so casting references of one to the other is safe.
        unsafe { &*(arr as *const [[T; ROOM_USIZE]; ROOM_USIZE] as *const Self) }
    }

    pub fn from_flat_ref(arr: &[T; ROOM_AREA]) -> &Self {
        // SAFETY: ROOM_AREA = ROOM_USIZE * ROOM_USIZE, so [T; ROOM_AREA] is identical
        // in data layout to [[T; ROOM_USIZE]; ROOM_USIZE].
        Self::from_ref(unsafe {
            &*(arr as *const [T; ROOM_AREA] as *const [[T; ROOM_USIZE]; ROOM_USIZE])
        })
    }

    pub fn from_mut(arr: &mut [[T; ROOM_USIZE]; ROOM_USIZE]) -> &mut Self {
        // SAFETY: XMajor is a repr(transparent) wrapper around [[T; ROOM_USIZE];
        // ROOM_USIZE], so casting references of one to the other is safe.
        unsafe { &mut *(arr as *mut [[T; ROOM_USIZE]; ROOM_USIZE] as *mut Self) }
    }

    pub fn from_flat_mut(arr: &mut [T; ROOM_AREA]) -> &mut Self {
        // SAFETY: ROOM_AREA = ROOM_USIZE * ROOM_USIZE, so [T; ROOM_AREA] is identical
        // in data layout to [[T; ROOM_USIZE]; ROOM_USIZE].
        Self::from_mut(unsafe {
            &mut *(arr as *mut [T; ROOM_AREA] as *mut [[T; ROOM_USIZE]; ROOM_USIZE])
        })
    }
}

impl<T> Index<RoomXY> for XMajor<T> {
    type Output = T;

    fn index(&self, index: RoomXY) -> &Self::Output {
        &self.0[index.x][index.y]
    }
}

impl<T> IndexMut<RoomXY> for XMajor<T> {
    fn index_mut(&mut self, index: RoomXY) -> &mut Self::Output {
        &mut self.0[index.x][index.y]
    }
}

/// A wrapper struct indicating that the inner array should be indexed Y major,
/// i.e. ```
/// use screeps::{
///     constants::ROOM_USIZE,
///     local::{YMajor, XY},
/// };
///
/// let mut y_major = YMajor([[0_u8; ROOM_USIZE]; ROOM_USIZE]);
/// y_major.0[0][10] = 1;
/// let xy = RoomXY::checked_new(10, 0).unwrap();
/// assert_eq!(y_major[xy], 1);
/// ```
#[repr(transparent)]
pub struct YMajor<T>(pub [[T; ROOM_USIZE]; ROOM_USIZE]);

impl<T> YMajor<T> {
    pub fn from_ref(arr: &[[T; ROOM_USIZE]; ROOM_USIZE]) -> &Self {
        // SAFETY: XMajor is a repr(transparent) wrapper around [[T; ROOM_USIZE];
        // ROOM_USIZE], so casting references of one to the other is safe.
        unsafe { &*(arr as *const [[T; ROOM_USIZE]; ROOM_USIZE] as *const Self) }
    }

    pub fn from_flat_ref(arr: &[T; ROOM_AREA]) -> &Self {
        // SAFETY: ROOM_AREA = ROOM_USIZE * ROOM_USIZE, so [T; ROOM_AREA] is identical
        // in data layout to [[T; ROOM_USIZE]; ROOM_USIZE].
        Self::from_ref(unsafe {
            &*(arr as *const [T; ROOM_AREA] as *const [[T; ROOM_USIZE]; ROOM_USIZE])
        })
    }

    pub fn from_mut(arr: &mut [[T; ROOM_USIZE]; ROOM_USIZE]) -> &mut Self {
        // SAFETY: XMajor is a repr(transparent) wrapper around [[T; ROOM_USIZE];
        // ROOM_USIZE], so casting references of one to the other is safe.
        unsafe { &mut *(arr as *mut [[T; ROOM_USIZE]; ROOM_USIZE] as *mut Self) }
    }

    pub fn from_flat_mut(arr: &mut [T; ROOM_AREA]) -> &mut Self {
        // SAFETY: ROOM_AREA = ROOM_USIZE * ROOM_USIZE, so [T; ROOM_AREA] is identical
        // in data layout to [[T; ROOM_USIZE]; ROOM_USIZE].
        Self::from_mut(unsafe {
            &mut *(arr as *mut [T; ROOM_AREA] as *mut [[T; ROOM_USIZE]; ROOM_USIZE])
        })
    }
}

impl<T> Index<RoomXY> for YMajor<T> {
    type Output = T;

    fn index(&self, index: RoomXY) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl<T> IndexMut<RoomXY> for YMajor<T> {
    fn index_mut(&mut self, index: RoomXY) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}
