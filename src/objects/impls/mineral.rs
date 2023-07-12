use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    constants::{Density, ResourceType},
    objects::RoomObject,
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// A [`Mineral`], which can be harvested for resources with an extractor.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Mineral;

    /// The density of the mineral on the next refill after it's depleted.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.density)
    #[wasm_bindgen(method, getter)]
    pub fn density(this: &Mineral) -> Density;

    /// Type of resource contained in this mineral.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.mineralType)
    #[wasm_bindgen(method, getter = mineralAmount)]
    pub fn mineral_amount(this: &Mineral) -> u32;

    /// Type of resource contained in this mineral.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.mineralType)
    #[wasm_bindgen(method, getter = mineralType)]
    pub fn mineral_type(this: &Mineral) -> ResourceType;

    /// Object ID of the mineral, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Mineral) -> JsString;

    /// The number of ticks until this mineral regenerates from depletion, or
    /// `None` if it's not currently regenerating.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.ticksToRegeneration)
    #[wasm_bindgen(method, getter = ticksToRegeneration)]
    pub fn ticks_to_regeneration(this: &Mineral) -> Option<u32>;
}

impl HasNativeId for Mineral {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}
