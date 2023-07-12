use js_sys::{JsString, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::{
    constants::{ErrorCode, Terrain},
    local::RoomName,
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a room's terrain held in the javascript heap.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room-Terrain)
    #[wasm_bindgen(js_namespace = Room, js_name = Terrain)]
    pub type RoomTerrain;

    #[wasm_bindgen(constructor, js_namespace = Room, js_class = Terrain)]
    fn new_internal(room_name: &JsString) -> RoomTerrain;

    /// Get the type of terrain at given coordinates.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get(this: &RoomTerrain, x: u8, y: u8) -> Terrain;

    // when called without a destination array, can't fail - no error code possible
    #[wasm_bindgen(method, js_name = getRawBuffer)]
    fn get_raw_buffer_internal(this: &RoomTerrain) -> Uint8Array;

    // and when called with a destination, it can only ever return a return code int
    #[wasm_bindgen(method, js_name = getRawBuffer)]
    fn get_raw_buffer_to_array_internal(this: &RoomTerrain, destination: &Uint8Array) -> i8;
}

impl RoomTerrain {
    /// Gets the terrain for any room by name, regardless of current visibility
    /// of the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.constructor)
    pub fn new(room_name: RoomName) -> RoomTerrain {
        let name = room_name.into();

        Self::new_internal(&name)
    }

    /// Get a copy of the underlying Uint8Array with the data about the room's
    /// terrain.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.getRawBuffer)
    #[inline]
    pub fn get_raw_buffer(&self) -> Uint8Array {
        self.get_raw_buffer_internal()
    }

    /// Copy the data about the room's terrain into an existing [`Uint8Array`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.getRawBuffer)
    #[inline]
    pub fn get_raw_buffer_to_array(&self, destination: &Uint8Array) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.get_raw_buffer_to_array_internal(destination))
    }
}
