use crate::constants::StructureType;
use wasm_bindgen::prelude::*;
use js_sys::JsString;

#[wasm_bindgen]
extern "C" {
    /// An object representing a position in a room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition)
    pub type RoomPosition;

    /// Create a new RoomPosition using the normal constructor, taking coordinates and the room name
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.constructor)
    #[wasm_bindgen(constructor)]
    pub fn new(x: u8, y: u8, room_name: &JsString) -> RoomPosition;

    /// Name of the room the position is in, as an owned [`JsString`] reference to a string in Javascript memory
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.roomName)
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &RoomPosition) -> JsString;

    /// Change the room the position refers to; must be a valid room name
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.roomName)
    #[wasm_bindgen(method, setter = roomName)]
    pub fn set_room_name(this: &RoomPosition) -> JsString;

    /// X coordinate of the position within the room
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, getter)]
    pub fn x(this: &RoomPosition) -> u8;

    /// Set a new X coordinate; must be in the range 0..49
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &RoomPosition) -> u8;

    /// Y coordinate of the position within the room
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &RoomPosition) -> u8;

    /// Set a new Y coordinate; must be in the range 0..49
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &RoomPosition) -> u8;

    // todo, get this as native
    /// Gets the efficient internal i32 representation of the position
    #[wasm_bindgen(method, getter = __packedPos)]
    pub fn packed(this: &RoomPosition) -> i32;

    // todo, as native
    /// Sets the position to a new one by passing an i32 that represents a packed position
    #[wasm_bindgen(method, setter = __packedPos)]
    pub fn set_packed(this: &RoomPosition, val: i32);
    
    /// Creates a construction site at this position
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_construction_site(this: &RoomPosition, ty: StructureType) -> i8;

    /// Creates a construction site at this position with a name for the structure
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_named_construction_site(this: &RoomPosition, ty: StructureType, name: &JsString) -> i8;
}
