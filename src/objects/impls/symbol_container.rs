use crate::{
    constants::ResourceType,
    objects::{RoomObject, Store},
    prelude::*,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`SymbolContainer`], which appears randomly
    /// around the map and contains symbol resources which can be
    /// collected.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolContainer)
    ///
    /// [`ResourceType::Score`]: crate::constants::ResourceType::Score
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type SymbolContainer;

    /// Object ID of the collector, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolContainer.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &SymbolContainer) -> JsString;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolContainer.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &SymbolContainer) -> Store;

    /// The number of ticks until the [`SymbolContainer`] will decay,
    /// disappearing completely.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolContainer.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &SymbolContainer) -> u32;

    /// The [`ResourceType`] contained within this [`SymbolContainer`] to score
    /// points.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolContainer.resourceType)
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &SymbolContainer) -> ResourceType;

}

impl CanDecay for SymbolContainer {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasNativeId for SymbolContainer {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}

impl HasStore for SymbolContainer {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
