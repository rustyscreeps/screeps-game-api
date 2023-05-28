use wasm_bindgen::prelude::*;

use crate::{
    objects::{RoomObject, Store},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`Reactor`], which process [`Thorium`] into
    /// season score.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Reactor)
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Reactor;
    // TODO also OwnedRoomObject

    /// Ticks of continuous work this reactor has done.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Reactor.continuousWork)
    #[wasm_bindgen(method, getter = continuousWork)]
    pub fn continuous_work(this: &Reactor) -> u32;

    /// The [`Store`] of the reactor, which contains information about what
    /// [`Thorium`] it is it holding.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Reactor.store)
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &Reactor) -> Store;
}

impl HasStore for Reactor {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
