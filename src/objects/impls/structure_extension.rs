use wasm_bindgen::prelude::*;

use crate::{
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureExtension`], which can store energy
    /// to be used by spawns in the room to spawn creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtension)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureExtension;

    /// The [`Store`] of the extension, which contains information about the
    /// amount of energy in it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtension.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureExtension) -> Store;
}

impl HasStore for StructureExtension {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl Attackable for StructureExtension {}
impl Dismantleable for StructureExtension {}
impl Repairable for StructureExtension {}
impl Transferable for StructureExtension {}
impl Withdrawable for StructureExtension {}
