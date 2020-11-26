use crate::{
    prelude::*,
    objects::{OwnedStructure, RoomObject, Store, Structure}
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureStorage`], which can store large
    /// amounts of resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureStorage)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureStorage;

    /// The [`Store`] of the storage, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureStorage.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureStorage) -> Store;
}

impl Attackable for StructureStorage {}
impl IsStructure for StructureStorage {}
impl HasStore for StructureStorage {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
