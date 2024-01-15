use wasm_bindgen::prelude::*;

use crate::{
    objects::{RoomObject, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// Parent class for all [`Structure`] objects types which are (or can be)
    /// owned by a specific player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    #[derive(Clone, Debug)]
    pub type OwnedStructure;

    // For controllers (possibly other structures?) user can be set to null, even
    // though it's meant to always be owned. This internal method is used to map
    // that case to false.
    #[wasm_bindgen(method, getter = my)]
    fn my_internal(this: &OwnedStructure) -> Option<bool>;

    /// The [`Owner`] of this structure that contains the owner's username, or
    /// `None` if it's an ownable structure currently not under a player's
    /// control.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &OwnedStructure) -> Option<Owner>;
}

impl OwnedStructure {
    /// Whether this structure is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.my)
    pub fn my(&self) -> bool {
        // If there is no user assigned, like in unowned controllers, `my` returns
        // undefined. That should be `false`, since that's not owned by the caller.
        self.my_internal().unwrap_or(false)
    }
}

impl Attackable for OwnedStructure {}

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

    /// The name of the player that owns this object.
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Owner) -> String;
}
