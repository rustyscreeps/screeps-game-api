use js_sys::{JsString, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::constants::{ReturnCode, Terrain};

#[wasm_bindgen]
extern "C" {
    /// An object representing a room's terrain held in the javascript heap.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room-Terrain)
    #[wasm_bindgen(js_namespace = Room, js_name = Terrain)]
    pub type RoomTerrain;

    /// Gets the terrain for any room by name, regardless of current visibility
    /// of the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.constructor)
    #[wasm_bindgen(constructor, js_namespace = Room, js_class = Terrain)]
    pub fn new(room_name: &JsString) -> RoomTerrain;

    /// Get the type of terrain at given coordinates.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get(this: &RoomTerrain, x: u8, y: u8) -> Terrain;

    //TODO: wiarchbe: Need to handle return code?
    /// Get a copy of the underlying Uint8Array with the data about the room's
    /// terrain.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.getRawBuffer)
    #[wasm_bindgen(method, js_name = getRawBuffer)]
    pub fn get_raw_buffer(this: &RoomTerrain) -> Uint8Array;

    /// Copy the data about the room's terrain into an existing [`Uint8Array`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.getRawBuffer)
    #[wasm_bindgen(method, js_name = getRawBuffer)]
    pub fn get_raw_buffer_to_array(this: &RoomTerrain, destination: &Uint8Array) -> ReturnCode;
}
