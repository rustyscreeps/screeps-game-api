use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    enums::action_error_codes::structure_observer::*,
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

    #[wasm_bindgen(method, js_name = observeRoom)]
    fn observe_room_internal(this: &StructureObserver, target: &JsString) -> i8;
}

impl StructureObserver {
    /// Set the [`StructureObserver`] to provide vision of a target room next
    /// tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureObserver.observeRoom)
    pub fn observe_room(&self, target: RoomName) -> Result<(), ObserveRoomErrorCode> {
        let target = target.into();

        ObserveRoomErrorCode::result_from_i8(self.observe_room_internal(&target))
    }
}

impl Attackable for StructureObserver {}
impl Dismantleable for StructureObserver {}
impl Repairable for StructureObserver {}
