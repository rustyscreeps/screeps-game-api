use std::convert::TryInto;
use std::ops::{Index, IndexMut};

use crate::objects::CostMatrix;

use super::Position;

#[derive(Clone, Debug)]
pub struct LocalCostMatrix {
    bits: [u8; 2500],
}

#[inline]
fn pos_as_idx(x: u8, y: u8) -> usize {
    (x as usize) * 50 + (y as usize)
}

impl Default for LocalCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalCostMatrix {
    #[inline]
    pub fn new() -> Self {
        LocalCostMatrix {
            bits: [0; 2500],
        }
    }

    #[inline]
    pub fn set(&mut self, x: u8, y: u8, val: u8) {
        assert!(x < 50, "out of bounds x: {}", x);
        assert!(y < 50, "out of bounds y: {}", y);
        // SAFETY: 0 <= x < 50, 0 <= y < 50, 0 <=pos_as_idx(x, y) < 2500
        unsafe { *self.bits.get_unchecked_mut(pos_as_idx(x, y)) = val; }
    }

    #[inline]
    pub fn get(&self, x: u8, y: u8) -> u8 {
        assert!(x < 50, "out of bounds x: {}", x);
        assert!(y < 50, "out of bounds y: {}", y);
        // SAFETY: 0 <= x < 50, 0 <= y < 50, 0 <=pos_as_idx(x, y) < 2500
        unsafe { *self.bits.get_unchecked(pos_as_idx(x, y)) }
    }

    // # Safety
    // Calling this method with x >= 50 or y >= 50 is undefined behaviour.
    #[inline]
    pub unsafe fn get_unchecked(&self, x: u8, y: u8) -> u8 {
        debug_assert!(x < 50, "out of bounds x: {}", x);
        debug_assert!(y < 50, "out of bounds y: {}", y);
        *self.bits.get_unchecked(pos_as_idx(x,y))
    }

    // # Safety
    // Calling this method with x >= 50 or y >= 50 is undefined behaviour.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, x: u8, y: u8, val: u8) {
        debug_assert!(x < 50, "out of bounds x: {}", x);
        debug_assert!(y < 50, "out of bounds y: {}", y);
        *self.bits.get_unchecked_mut(pos_as_idx(x, y)) = val;
    }

    pub fn get_bits(&self) -> &[u8; 2500] {
        &self.bits
    }

    // /// Copies all data into an JavaScript CostMatrix for use.
    // ///
    // /// This is slower than [`as_uploaded`], but much safer.
    // ///
    // /// [`as_uploaded`]: #method.as_uploaded
    // pub fn upload(&self) -> CostMatrix<'static> {
    //     let bits: TypedArray<u8> = self.bits[..].into();

    //     CostMatrix {
    //         inner: (js! {
    //             var matrix = Object.create(PathFinder.CostMatrix.prototype);
    //             matrix._bits = @{bits};
    //             return matrix;
    //         })
    //         .try_into()
    //         .expect("expected function returning CostMatrix to return a Reference"),
    //         lifetime: PhantomData,
    //     }
    // }

    // /// Temporarily exposes the bits of this matrix as a cost matrix.
    // ///
    // /// # Unsafety
    // ///
    // /// There are two main invariants you must uphold after using this function:
    // ///
    // /// 1. The `CostMatrix` can only be used in JS code as long as this
    // /// `LocalCostMatrix` is alive.    Doing otherwise will result in
    // /// undefined behavior, mainly JS being allowed to read/    manipulate
    // /// uninitialized rust memory or rust memory that's been repurposed.
    // ///
    // /// 2. The `set` method of the cost matrix must not be used - it must be
    // /// read only. This takes    &self, but technically allows mutation of
    // /// the inner Vec via JavaScript access. You    should not use this
    // /// method, or you will invoke Rust undefined behavior.
    // ///
    // /// The CostMatrix returned will _reference the internal data of this
    // /// `LocalCostMatrix`_.
    // pub unsafe fn as_uploaded<'a>(&'a self) -> CostMatrix<'a> {
    //     let bits: UnsafeTypedArray<'_, u8> = UnsafeTypedArray::new(&self.bits);

    //     CostMatrix {
    //         inner: (js! {
    //             // using this first is necessary in order to uphold the invariant of
    //             // `UnsafeTypedArray`.
    //             var bits = @{bits};

    //             var matrix = Object.create(PathFinder.CostMatrix.prototype);
    //             matrix._bits = bits;

    //             return matrix;
    //         })
    //         .try_into()
    //         .expect("expected function returning CostMatrix to return a Reference"),
    //         lifetime: PhantomData,
    //     }
    // }
}

impl From<LocalCostMatrix> for Vec<u8> {
    /// Returns a vector of bits length 2500, where each position is
    /// `idx = ((x * 50) + y)`.
    #[inline]
    fn from(lcm: LocalCostMatrix) -> Vec<u8> {
        lcm.bits.into()
    }
}

impl From<CostMatrix> for LocalCostMatrix {
    fn from(js_matrix: CostMatrix) -> Self {
        let array = js_matrix.get_bits();

        LocalCostMatrix {
            bits: array.to_vec().try_into().expect("JS CostMatrix was not length 2500."),
        }
    }
}

impl Index<(u8, u8)> for LocalCostMatrix {
    type Output = u8;

    fn index(&self, idx: (u8, u8)) -> &Self::Output {
        assert!(idx.0 < 50, "out of bounds x: {}", idx.0);
        assert!(idx.1 < 50, "out of bounds y: {}", idx.1);
        &self.bits[pos_as_idx(idx.0, idx.1)]
    }
}

impl IndexMut<(u8, u8)> for LocalCostMatrix {
    fn index_mut(&mut self, idx: (u8, u8)) -> &mut Self::Output {
        assert!(idx.0 < 50, "out of bounds x: {}", idx.0);
        assert!(idx.1 < 50, "out of bounds y: {}", idx.1);
        &mut self.bits[pos_as_idx(idx.0, idx.1)]
    }
}

// TODO: Remove the casts when #346 is merged.
impl Index<Position> for LocalCostMatrix {
    type Output = u8;

    fn index(&self,  idx: Position) -> &Self::Output {
        // SAFETY: Position always gives a valid in-room coordinate.
        unsafe { self.bits.get_unchecked(pos_as_idx(idx.x() as u8, idx.y() as u8)) }
    }
}

impl IndexMut<Position> for LocalCostMatrix {
    fn index_mut(&mut self, idx: Position) -> &mut Self::Output {
        // SAFETY: Position always gives a valid in-room coordinate.
        unsafe { self.bits.get_unchecked_mut(pos_as_idx(idx.x() as u8, idx.y() as u8)) }
    }
}

// impl<'a> CostMatrixSet for LocalCostMatrix {
//     fn set_multi<D, B, P, V>(&mut self, data: D) where D: IntoIterator<Item = B>, B: Borrow<(P, V)>, P: HasLocalPosition, V: Borrow<u8> {
//         let iter = data.into_iter();

//         for entry in iter {
//             let (pos, cost) = entry.borrow();
            
//             self.set(pos.x(), pos.y(), *cost.borrow());
//         }
//     }
// }

// /// A `CostMatrix` that's valid to pass as a result from a `PathFinder.search`
// /// room callback.
// ///
// /// Lives as long as `'a` lifetime. It's unsound to leak to JS past this
// /// lifetime if this matrix was created by [`LocalCostMatrix::as_uploaded`].
// ///
// /// [`LocalCostMatrix::as_uploaded`]:
// /// struct.LocalCostMatrix.html#method.as_uploaded
// pub struct CostMatrix<'a> {
//     pub(crate) inner: Reference,
//     pub(crate) lifetime: PhantomData<&'a ()>,
// }

// impl Default for CostMatrix<'static> {
//     fn default() -> Self {
//         CostMatrix {
//             inner: js_unwrap!(new PathFinder.CostMatrix()),
//             lifetime: PhantomData,
//         }
//     }
// }

// impl<'a> Into<MultiRoomCostResult<'a>> for CostMatrix<'a> {
//     fn into(self) -> MultiRoomCostResult<'a> {
//         MultiRoomCostResult::CostMatrix(self)
//     }
// }

// impl<'a> Into<SingleRoomCostResult<'a>> for CostMatrix<'a> {
//     fn into(self) -> SingleRoomCostResult<'a> {
//         SingleRoomCostResult::CostMatrix(self)
//     }
// }

// pub trait HasLocalPosition {
//     fn x(&self) -> u8;
//     fn y(&self) -> u8;
// }

// pub trait CostMatrixSet {
//     fn set<P, V>(&mut self, position: P, cost: V) where P: HasLocalPosition, V: Borrow<u8> {
//         self.set_multi(&[(position, cost)])
//     }

//     fn set_multi<D, B, P, V>(&mut self, data: D) where D: IntoIterator<Item = B>, B: Borrow<(P, V)>, P: HasLocalPosition, V: Borrow<u8>;
// }

// impl<'a> CostMatrixSet for CostMatrix<'a> {
//     fn set_multi<D, B, P, V>(&mut self, data: D) where D: IntoIterator<Item = B>, B: Borrow<(P, V)>, P: HasLocalPosition, V: Borrow<u8> {
//         let iter = data.into_iter();
//         let (minimum_size, _maximum_size) = iter.size_hint();
//         let mut storage: Vec<u8> = Vec::with_capacity(minimum_size * 3);

//         for entry in iter {
//             let (pos, cost) = entry.borrow();
//             storage.push(pos.x());
//             storage.push(pos.y());
//             storage.push(*cost.borrow());
//         }

//         let bits: TypedArray<u8> = storage.as_slice().into();

//         js!(
//             let matrix = @{&self.inner};
//             let raw_data = @{bits};

//             const element_count = raw_data.length / 3;

//             for (let index = 0; index < element_count; ++index) {
//                 const offset = index * 3;

//                 matrix.set(raw_data[offset + 0], raw_data[offset + 1], raw_data[offset + 2]);
//             }
//         );
//     }
// }

// need custom implementation in order to ensure length of 'bits' is always 2500
mod serde_impls {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::convert::TryInto;

    use super::LocalCostMatrix;

    impl Serialize for LocalCostMatrix {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.bits.serialize(s)
        }
    }

    impl<'de> Deserialize<'de> for LocalCostMatrix {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec_bits: Vec<u8> = Vec::deserialize(deserializer)?;

            if vec_bits.len() != 2500 {
                return Err(D::Error::invalid_length(
                    vec_bits.len(),
                    &"a vec of length 2500",
                ));
            }

            // SAFETY: If the length wasn't right, we would have hit the check above
            Ok(LocalCostMatrix { bits: vec_bits.try_into().unwrap() })
        }
    }
}
