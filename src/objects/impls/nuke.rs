use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{objects::RoomObject, prelude::*};

#[wasm_bindgen]
extern "C" {
    /// A [`Nuke`] in flight, which will deal damage in an area and kill all
    /// creeps in the room when it lands.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Nuke)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Nuke;

    /// Object ID of the Nuke, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Nuke.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Nuke) -> JsString;

    /// The name of the room the nuke was fired from.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Nuke.launchRoomName)
    #[wasm_bindgen(method, getter)]
    pub fn launch_room_name(this: &Nuke) -> JsString;

    /// Ticks until the nuke lands.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Nuke.timeToLand)
    #[wasm_bindgen(method, getter = timeToLand)]
    pub fn time_to_land(this: &Nuke) -> u32;
}

impl HasNativeId for Nuke {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}
