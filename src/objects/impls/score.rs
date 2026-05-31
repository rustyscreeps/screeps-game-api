use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    objects::{RoomObject, Store},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An reference to a javascript object representing a [`Score`], which
    /// appears randomly around the map and adds to your score when stepped on
    /// by one of your creeps.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Score)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Score;

    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Score) -> JsString;

    /// The amount of [`Score`] that this object is worth when collected by a
    /// creep.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Score.score)
    #[wasm_bindgen(method, getter)]
    pub fn score(this: &Score) -> u32;

    /// The number of ticks until the [`Score`] will decay, disappearing
    /// completely.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Score.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &Score) -> u32;
}

impl CanDecay for Score {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasId for Score {
    fn js_raw_id(&self) -> JsString {
        Self::id_internal(self)
    }
}
