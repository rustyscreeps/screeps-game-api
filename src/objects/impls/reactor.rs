use wasm_bindgen::prelude::*;

use crate::{
    objects::{Owner, RoomObject, Store},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`Reactor`], which process [`Thorium`] to gain
    /// season score.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Reactor)
    ///
    /// [`Thorium`]: crate::constants::ResourceType::Thorium
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Reactor;

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

    // owner and my are on OwnedStructure and we'd usually inherit them, but since
    // it inherits Structure, and Reactor is not a Structure, implementing these
    // directly.
    /// Whether this reactor is owned by the player.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Reactor.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &Reactor) -> bool;

    /// The [`Owner`] of this reactor that contains the owner's username, or
    /// `None` if it's currently not under a player's
    /// control.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Reactor.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &Reactor) -> Option<Owner>;
}

impl HasStore for Reactor {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
