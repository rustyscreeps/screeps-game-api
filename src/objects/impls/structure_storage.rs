use wasm_bindgen::prelude::*;

use crate::{
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureStorage`], which can store large
    /// amounts of resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureStorage)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureStorage;

    /// The [`Store`] of the storage, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureStorage.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureStorage) -> Store;
}

impl HasStore for StructureStorage {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
