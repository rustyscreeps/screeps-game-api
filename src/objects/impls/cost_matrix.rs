use std::borrow::Borrow;
use wasm_bindgen::prelude::*;

use crate::{constants::ROOM_SIZE, local::LocalCostMatrix, prototypes::COST_MATRIX_PROTOTYPE};

use js_sys::{Array, Object, Uint8Array};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`CostMatrix`] held in the javascript heap.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder-CostMatrix)
    #[wasm_bindgen(js_namespace = PathFinder)]
    pub type CostMatrix;

    /// Create a new reference to a CostMatrix, containing 0s in all positions,
    /// using the normal constructor.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.constructor)
    #[wasm_bindgen(constructor, js_namespace = PathFinder)]
    pub fn new() -> CostMatrix;

    /// Gets a reference to the [`Uint8Array`] underlying this [`CostMatrix`].
    #[wasm_bindgen(method, getter = _bits)]
    pub fn get_bits(this: &CostMatrix) -> Uint8Array;

    /// Sets a [`Uint8Array`] to this [`CostMatrix`], overwriting any current
    /// contents.
    #[wasm_bindgen(method, setter = _bits)]
    pub fn set_bits(this: &CostMatrix, arr: &Uint8Array);

    /// Sets a new value for a specific position in this [`CostMatrix`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.set)
    #[wasm_bindgen(method)]
    pub fn set(this: &CostMatrix, x: u8, y: u8, cost: u8);

    /// Get the value of a specific position in this [`CostMatrix`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.get)
    #[wasm_bindgen(method)]
    pub fn get(this: &CostMatrix, x: u8, y: u8) -> u8;

    /// Get a new [`CostMatrix`] with data copied from the current one
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.clone)
    #[wasm_bindgen(method)]
    pub fn clone(this: &CostMatrix) -> CostMatrix;

    /// Get an [`Array`] of numbers representing the [`CostMatrix`] that's
    /// appropriate for memory serialization.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.serialize)
    #[wasm_bindgen(method)]
    pub fn serialize(this: &CostMatrix) -> Array;

    /// Get a new [`CostMatrix`] using the array representation from
    /// [`CostMatrix::serialize`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.deserialize)
    #[wasm_bindgen(static_method_of = CostMatrix, js_namespace = PathFinder)]
    pub fn deserialize(val: Array) -> CostMatrix;
}

impl CostMatrix {
    /// Create a new [`CostMatrix`], taking a u8 slice with 2500 members such as
    /// that returned from [`LocalCostMatrix::get_bits`] which will be copied
    /// across the memory boundary.
    ///
    /// [`LocalCostMatrix::get_bits`]: crate::local::LocalCostMatrix
    pub fn new_from_bits(bits: &[u8]) -> CostMatrix {
        let matrix = CostMatrix::from(JsValue::from(Object::create(&COST_MATRIX_PROTOTYPE)));
        matrix.set_bits(&Uint8Array::from(bits));
        matrix
    }

    // todo also a function that takes the unsafe view into wasm linear mem with
    // view for a matrix that'll easily go bad
}

impl From<LocalCostMatrix> for CostMatrix {
    fn from(matrix: LocalCostMatrix) -> Self {
        CostMatrix::new_from_bits(matrix.get_bits())
    }
}

pub trait HasLocalPosition {
    fn x(&self) -> u8;
    fn y(&self) -> u8;
}

pub trait CostMatrixSet {
    fn set<P, V>(&mut self, position: P, cost: V)
    where
        P: HasLocalPosition,
        V: Borrow<u8>;

    fn set_multi<D, B, P, V>(&mut self, data: D)
    where
        D: IntoIterator<Item = B>,
        B: Borrow<(P, V)>,
        P: HasLocalPosition,
        V: Borrow<u8>;
}

#[inline]
fn pos_as_idx(x: u8, y: u8) -> usize {
    (x as usize) * ROOM_SIZE as usize + (y as usize)
}

impl CostMatrixSet for CostMatrix {
    fn set<P, V>(&mut self, position: P, cost: V)
    where
        P: HasLocalPosition,
        V: Borrow<u8>,
    {
        CostMatrix::set(self, position.x(), position.y(), *cost.borrow());
    }

    fn set_multi<D, B, P, V>(&mut self, data: D)
    where
        D: IntoIterator<Item = B>,
        B: Borrow<(P, V)>,
        P: HasLocalPosition,
        V: Borrow<u8>,
    {
        let matrix_buffer = self.get_bits();

        for entry in data.into_iter() {
            let (pos, cost) = entry.borrow();

            let offset = pos_as_idx(pos.x(), pos.y());

            matrix_buffer.set_index(offset as u32, *cost.borrow());
        }
    }
}
