use crate::{
    objects::{Owner, RoomObject, Structure},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Parent class for all [`Structure`] objects types which are (or can be) owned by a specific player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type OwnedStructure;

    /// Whether this structure is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &OwnedStructure) -> bool;

    /// The name of the player that owns this structure as a [`JsString`], or `None` if it's an ownable structure currently not under a player's control.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &OwnedStructure) -> Option<Owner>;
}
