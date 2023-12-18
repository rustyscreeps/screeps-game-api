use js_sys::{Array, Object, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::{
    local::{LocalCostMatrix, RoomXY},
    prototypes::COST_MATRIX_PROTOTYPE,
    traits::{CostMatrixGet, CostMatrixSet},
};

#[wasm_bindgen]
extern "C" {
    /// A reference to a matrix of pathing costs for a room, stored in
    /// JavaScript memory.
    ///
    /// Use [`LocalCostMatrix`] to store and access the same data in Rust
    /// memory.
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

impl CostMatrixSet for CostMatrix {
    fn set_xy(&mut self, xy: RoomXY, cost: u8) {
        CostMatrix::set(self, xy.x.u8(), xy.y.u8(), cost);
    }
}

impl CostMatrixGet for CostMatrix {
    fn get_xy(&mut self, xy: RoomXY) -> u8 {
        CostMatrix::get(self, xy.x.u8(), xy.y.u8())
    }
}
