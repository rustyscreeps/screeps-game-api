use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::{
    objects::CostMatrix,
    traits::{CostMatrixGet, CostMatrixSet},
};

use super::{linear_index_to_xy, xy_to_linear_index, Position, RoomXY, ROOM_AREA};

/// A matrix of pathing costs for a room, stored in Rust memory.
///
/// Use [`CostMatrix`] if a reference to data stored in JavaScript memory is
/// preferred.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LocalCostMatrix {
    #[serde(with = "serde_impls")]
    bits: [u8; ROOM_AREA],
}

impl Default for LocalCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalCostMatrix {
    #[inline]
    pub const fn new() -> Self {
        LocalCostMatrix {
            bits: [0; ROOM_AREA],
        }
    }

    // # Notes
    // This method does no bounds checking for the passed-in `RoomXY`, you may use
    // `RoomXY::unchecked_new` to skip all bounds checking.
    #[inline]
    pub fn set(&mut self, xy: RoomXY, val: u8) {
        self[xy] = val;
    }

    // # Notes
    // This method does no bounds checking for the passed-in `RoomXY`, you may use
    // `RoomXY::unchecked_new` to skip all bounds checking.
    #[inline]
    pub fn get(&self, xy: RoomXY) -> u8 {
        self[xy]
    }

    pub const fn get_bits(&self) -> &[u8; ROOM_AREA] {
        &self.bits
    }

    pub fn iter(&self) -> impl Iterator<Item = (RoomXY, u8)> + '_ {
        self.bits
            .iter()
            .enumerate()
            .map(|(idx, &val)| (linear_index_to_xy(idx), val))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (RoomXY, &mut u8)> {
        self.bits
            .iter_mut()
            .enumerate()
            .map(|(idx, val)| (linear_index_to_xy(idx), val))
    }
}

impl From<LocalCostMatrix> for Vec<u8> {
    /// Returns a vector of bits length ROOM_AREA, where each position is
    /// `idx = ((x * ROOM_SIZE) + y)`.
    #[inline]
    fn from(lcm: LocalCostMatrix) -> Vec<u8> {
        lcm.bits.into()
    }
}

impl From<&LocalCostMatrix> for Vec<u8> {
    fn from(lcm: &LocalCostMatrix) -> Vec<u8> {
        lcm.bits.into()
    }
}

impl From<&CostMatrix> for LocalCostMatrix {
    fn from(js_matrix: &CostMatrix) -> Self {
        let mut bits = [0; ROOM_AREA];
        js_matrix.get_bits().copy_to(&mut bits);

        LocalCostMatrix { bits }
    }
}

impl Index<RoomXY> for LocalCostMatrix {
    type Output = u8;

    fn index(&self, xy: RoomXY) -> &Self::Output {
        // SAFETY: RoomXY is always a valid coordinate.
        unsafe { self.bits.get_unchecked(xy_to_linear_index(xy)) }
    }
}

impl IndexMut<RoomXY> for LocalCostMatrix {
    fn index_mut(&mut self, xy: RoomXY) -> &mut Self::Output {
        // SAFETY: RoomXY is always a valid coordinate.
        unsafe { self.bits.get_unchecked_mut(xy_to_linear_index(xy)) }
    }
}

impl Index<Position> for LocalCostMatrix {
    type Output = u8;

    fn index(&self, idx: Position) -> &Self::Output {
        &self[RoomXY::from(idx)]
    }
}

impl IndexMut<Position> for LocalCostMatrix {
    fn index_mut(&mut self, idx: Position) -> &mut Self::Output {
        &mut self[RoomXY::from(idx)]
    }
}

impl CostMatrixSet for LocalCostMatrix {
    fn set_xy(&mut self, xy: RoomXY, cost: u8) {
        LocalCostMatrix::set(self, xy, cost);
    }
}

impl CostMatrixGet for LocalCostMatrix {
    fn get_xy(&mut self, xy: RoomXY) -> u8 {
        LocalCostMatrix::get(self, xy)
    }
}

// need custom implementation in order to ensure length of 'bits' is always
// ROOM_AREA
mod serde_impls {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::convert::TryInto;

    use super::ROOM_AREA;

    pub(super) fn serialize<S>(bits: &[u8; ROOM_AREA], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bits[..].serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<[u8; ROOM_AREA], D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits_slice: &[u8] = <&[u8]>::deserialize(deserializer)?;

        if bits_slice.len() != ROOM_AREA {
            return Err(D::Error::invalid_length(
                bits_slice.len(),
                &format!("a vec of length {ROOM_AREA}").as_str(),
            ));
        }

        // SAFETY: If the length wasn't right, we would have hit the check above
        Ok(bits_slice.try_into().unwrap())
    }
}
