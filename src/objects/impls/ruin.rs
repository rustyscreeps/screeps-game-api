use crate::{
    objects::{RoomObject, Store, Structure},
    prelude::*,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Ruin`], which represents a destroyed structure and can have
    /// resources withdrawn from it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Ruin;

    /// The tick that the structure was destroyed
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.destroyTime)
    #[wasm_bindgen(method, getter = destroyTime)]
    pub fn destroy_time(this: &Ruin) -> u32;

    /// Object ID of the ruin, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Ruin) -> JsString;

    /// The [`Store`] of the ruin, which contains any resources in the ruin.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &Ruin) -> Store;

    /// The destroyed [`Structure`] that this ruin represents. Note that this
    /// object is not fully safe to use as a [`Structure`], missing critical
    /// properties such as position; it's only safe to access basic information
    /// about the structure on this object, like the structure type, owner name,
    /// and id.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.structure)
    #[wasm_bindgen(method, getter)]
    pub fn structure(this: &Ruin) -> Structure;

    /// The number of ticks until this ruin disappears.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &Ruin) -> u32;
}

impl CanDecay for Ruin {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasNativeId for Ruin {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}

impl HasStore for Ruin {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
