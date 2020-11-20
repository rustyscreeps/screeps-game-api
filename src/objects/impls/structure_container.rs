use crate::objects::{RoomObject, Store, Structure};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureContainer`], which can store
    /// resources and does not block creep movement, but requires regular repair
    /// due to decay.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructureContainer;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureContainer) -> Store;

    /// The number of ticks until the rampart will decay, losing TODO CONSTANT
    /// hits.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureContainer) -> u32;
}
