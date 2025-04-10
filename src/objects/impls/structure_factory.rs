use wasm_bindgen::prelude::*;

use crate::{
    constants::ResourceType,
    enums::action_error_codes::structure_factory::*,
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

    #[wasm_bindgen(method, js_name = produce)]
    fn produce_internal(this: &StructureFactory, ty: ResourceType) -> i8;
}

impl StructureFactory {
    /// Produce a commodity in the factory.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.produce)
    pub fn produce(&self, ty: ResourceType) -> Result<(), ProduceErrorCode> {
        ProduceErrorCode::result_from_i8(self.produce_internal(ty))
    }
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

impl Attackable for StructureFactory {}
impl Dismantleable for StructureFactory {}
impl Repairable for StructureFactory {}
impl Transferable for StructureFactory {}
impl Withdrawable for StructureFactory {}
