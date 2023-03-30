use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    objects::{RoomObject, Store},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`ScoreContainer`], which appears randomly
    /// around the map and contains [`ResourceType::Score`] which can be
    /// collected.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer)
    ///
    /// [`ResourceType::Score`]: crate::constants::ResourceType::Score
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type ScoreContainer;

    /// Object ID of the collector, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &ScoreContainer) -> JsString;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &ScoreContainer) -> Store;

    /// The number of ticks until the [`ScoreContainer`] will decay,
    /// disappearing completely.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &ScoreContainer) -> u32;
}

impl CanDecay for ScoreContainer {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasNativeId for ScoreContainer {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}

impl HasStore for ScoreContainer {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
