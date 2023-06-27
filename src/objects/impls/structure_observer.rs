use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    constants::ErrorCode,
    local::RoomName,
    objects::{OwnedStructure, RoomObject, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureObserver`], which can grant vision
    /// to remote rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureObserver)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureObserver;

    /// Set the [`StructureObserver`] to provide vision of a target room next
    /// tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureObserver.observeRoom)
    #[wasm_bindgen(method, js_name = observeRoom)]
    fn observe_room_internal(this: &StructureObserver, target: &JsString) -> i8;
}

impl StructureObserver {
    pub fn observe_room(&self, target: RoomName) -> Result<(), ErrorCode> {
        let target = target.into();

        ErrorCode::result_from_i8(self.observe_room_internal(&target))
    }
}
