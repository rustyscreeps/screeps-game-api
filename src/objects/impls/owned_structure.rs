use crate::{
    objects::{RoomObject, Structure},
    prelude::*,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Parent class for all [`Structure`] objects types which are (or can be)
    /// owned by a specific player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    #[derive(Clone, Debug)]
    pub type OwnedStructure;

    /// Whether this structure is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &OwnedStructure) -> bool;

    /// The [`Owner`] of this structure that contains the owner's username, or
    /// `None` if it's an ownable structure currently not under a player's
    /// control.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &OwnedStructure) -> Option<Owner>;
}

impl<T> OwnedStructureProperties for T
where
    T: AsRef<OwnedStructure>,
{
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}

#[wasm_bindgen]
extern "C" {
    /// Object with owner info for an owned game object.
    #[wasm_bindgen]
    pub type Owner;

    #[wasm_bindgen(method, getter = username)]
    fn username_internal(this: &Owner) -> JsString;
}

impl Owner {
    /// The name of the player that owns this object.
    pub fn username(&self) -> String {
        Self::username_internal(self).into()
    }
}
