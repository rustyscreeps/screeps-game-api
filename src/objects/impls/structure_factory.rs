use wasm_bindgen::prelude::*;

use crate::{
    constants::{ResourceType, ReturnCode},
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureFactory`], which can compress and
    /// decompress resources and produce commodities for sale.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureFactory;

    /// Ticks until [`StructureFactory::produce`] can be used again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.cooldown)
    ///
    /// [`StructureFactory::produce`]: crate::objects::StructureFactory::produce
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureFactory) -> u32;

    /// The level of the factory, which cannot be changed once set by a power
    /// creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &StructureFactory) -> u8;

    /// The [`Store`] of the factory, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureFactory) -> Store;

    /// Produce a commodity in the factory.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.produce)
    #[wasm_bindgen(method)]
    pub fn produce(this: &StructureFactory, ty: ResourceType) -> ReturnCode;
}

impl HasCooldown for StructureFactory {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasStore for StructureFactory {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
