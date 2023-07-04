use std::{
    collections::HashMap,
    iter::IntoIterator,
    ops::{Index, IndexMut},
};

use serde::{Deserialize, Serialize};

use crate::objects::CostMatrix;

use super::{linear_index_to_xy, xy_to_linear_index, Position, RoomXY, ROOM_AREA};

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

    // Takes all non-zero entries in `src`, and inserts them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_dense(&mut self, src: &LocalCostMatrix) {
        for i in 0..ROOM_AREA {
            let val = unsafe { *src.bits.get_unchecked(i) };
            if val > 0 {
                unsafe {
                    *self.bits.get_unchecked_mut(i) = val;
                }
            }
        }
    }

    // Takes all entries in `src` and merges them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_sparse(&mut self, src: &SparseCostMatrix) {
        for (xy, val) in src.iter() {
            unsafe {
                *self.bits.get_unchecked_mut(xy_to_linear_index(xy)) = val;
            }
        }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SparseCostMatrix {
    inner: HashMap<RoomXY, u8>,
}

impl Default for SparseCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseCostMatrix {
    pub fn new() -> Self {
        SparseCostMatrix {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, xy: RoomXY) -> u8 {
        *self.inner.get(&xy).unwrap_or(&0)
    }

    pub fn set(&mut self, xy: RoomXY, val: u8) {
        self.inner.insert(xy, val);
    }

    pub fn iter(&self) -> impl Iterator<Item = (RoomXY, u8)> + '_ {
        self.inner.iter().map(|(&pos, &val)| (pos, val))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (RoomXY, &mut u8)> {
        self.inner.iter_mut().map(|(&pos, val)| (pos, val))
    }

    // Takes all non-zero entries in `src`, and inserts them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_dense(&mut self, src: &LocalCostMatrix) {
        self.inner.extend(src.iter().filter_map(
            |(xy, val)| {
                if val > 0 {
                    Some((xy, val))
                } else {
                    None
                }
            },
        ))
    }

    // Takes all entries in `src` and merges them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_sparse(&mut self, src: &SparseCostMatrix) {
        self.inner.extend(src.inner.iter());
    }
}

impl From<HashMap<RoomXY, u8>> for SparseCostMatrix {
    fn from(inner: HashMap<RoomXY, u8>) -> Self {
        SparseCostMatrix { inner }
    }
}

impl From<&HashMap<RoomXY, u8>> for SparseCostMatrix {
    fn from(map: &HashMap<RoomXY, u8>) -> Self {
        SparseCostMatrix { inner: map.clone() }
    }
}

impl From<&HashMap<Position, u8>> for SparseCostMatrix {
    fn from(map: &HashMap<Position, u8>) -> Self {
        SparseCostMatrix {
            inner: map.iter().map(|(&pos, &val)| (pos.into(), val)).collect(),
        }
    }
}

impl From<&CostMatrix> for SparseCostMatrix {
    fn from(js_matrix: &CostMatrix) -> Self {
        let vals: Vec<u8> = js_matrix.get_bits().to_vec();
        assert!(
            vals.len() == ROOM_AREA,
            "JS CostMatrix had length {} instead of {}.",
            vals.len(),
            ROOM_AREA
        );

        SparseCostMatrix {
            inner: vals
                .into_iter()
                .enumerate()
                .filter_map(|(idx, val)| {
                    // 0 is the same as unset, so filtering it out
                    if val > 0 {
                        Some((linear_index_to_xy(idx), val))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl From<&LocalCostMatrix> for SparseCostMatrix {
    fn from(lcm: &LocalCostMatrix) -> Self {
        SparseCostMatrix {
            inner: lcm
                .iter()
                .filter_map(|(xy, val)| if val > 0 { Some((xy, val)) } else { None })
                .collect(),
        }
    }
}

impl From<SparseCostMatrix> for LocalCostMatrix {
    fn from(mut scm: SparseCostMatrix) -> Self {
        let mut lcm = LocalCostMatrix::new();
        for (pos, val) in scm.inner.drain() {
            lcm[pos] = val;
        }
        lcm
    }
}

impl From<&SparseCostMatrix> for LocalCostMatrix {
    fn from(scm: &SparseCostMatrix) -> Self {
        let mut lcm = LocalCostMatrix::new();
        for (&pos, &val) in scm.inner.iter() {
            lcm[pos] = val;
        }
        lcm
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
