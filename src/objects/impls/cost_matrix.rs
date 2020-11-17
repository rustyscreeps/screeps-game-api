use wasm_bindgen::prelude::*;

use js_sys::{Array, Uint8Array};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`CostMatrix`] held in the javascript heap.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder-CostMatrix)
    #[wasm_bindgen(js_namespace = PathFinder)]
    pub type CostMatrix;

    /// Create a new reference to a CostMatrix, containing 0s in all positions, using the normal constructor.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.constructor)
    #[wasm_bindgen(constructor, js_namespace = PathFinder)]
    pub fn new() -> CostMatrix;

    // TODO make a new_with_bits
    // https://github.com/rustwasm/wasm-bindgen/blob/master/crates/js-sys/tests/wasm/Object.rs#L36
    //     #[wasm_bindgen(js_name = prototype, js_namespace = Foo)]
    //     static FOO_PROTOTYPE: Object;

    /// Gets a reference to the [`Uint8Array`] underlying this [`CostMatrix`].
    #[wasm_bindgen(method, getter = _bits)]
    pub fn get_bits(this: &CostMatrix) -> Uint8Array;

    /// Sets a [`Uint8Array`] to this [`CostMatrix`], overwriting any current contents.
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

    /// Get an [`Array`] of numbers representing the [`CostMatrix`] that's appropriate for memory serialization.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.serialize)
    #[wasm_bindgen(method)]
    pub fn serialize(this: &CostMatrix) -> Array;
    
    /// Get a new [`CostMatrix`] using the array representation from [`CostMatrix::serialize`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.CostMatrix.deserialize)
    #[wasm_bindgen(static_method_of = CostMatrix, js_namespace = PathFinder)]
    pub fn deserialize(val: Array) -> CostMatrix;
}
