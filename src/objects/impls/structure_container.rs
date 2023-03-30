use wasm_bindgen::prelude::*;

use crate::{
    objects::{RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureContainer`], which can store
    /// resources and does not block creep movement, but requires regular repair
    /// due to decay.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    #[derive(Clone, Debug)]
    pub type StructureContainer;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureContainer) -> Store;

    /// The number of ticks until the container will decay, losing
    /// [`CONTAINER_DECAY`] hits. The time between each decay interval
    /// depends whether the container is in an owned room;
    /// [`CONTAINER_DECAY_TIME_OWNED`] in owned rooms and
    /// [`CONTAINER_DECAY_TIME`] in all other rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer.ticksToDecay)
    ///
    /// [`CONTAINER_DECAY`]: crate::constants::CONTAINER_DECAY
    /// [`CONTAINER_DECAY_TIME_OWNED`]: crate::constants::CONTAINER_DECAY_TIME_OWNED
    /// [`CONTAINER_DECAY_TIME`]: crate::constants::CONTAINER_DECAY_TIME
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureContainer) -> u32;
}

impl CanDecay for StructureContainer {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasStore for StructureContainer {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
