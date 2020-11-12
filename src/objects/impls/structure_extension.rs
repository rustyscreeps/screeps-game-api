use crate::{
    objects::{OwnedStructure, RoomObject, Structure, Store},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureExtension`], which can store energy to be used by spawns in the room to spawn creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtension)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureExtension;

    /// The [`Store`] of the extension, which contains information about the amount of energy in it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtension.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureExtension) -> Store;
}
