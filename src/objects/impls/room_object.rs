use crate::{Position, objects::{Room, RoomPosition}, prelude::*};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Parent class for all objects in rooms in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room)
    #[derive(Clone)]
    pub type RoomObject;

    /// Effects applied to the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.effects)
    #[wasm_bindgen(method, getter)]
    pub fn effects(this: &RoomObject) -> Array;

    /// Position of the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.pos)
    #[wasm_bindgen(method, getter)]
    pub fn pos(this: &RoomObject) -> RoomPosition;

    /// A link to the room that the object is currently in, or `None` if the
    /// object is a power creep not spawned on the current shard, or a flag or
    /// construction site not in a visible room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.room)
    #[wasm_bindgen(method, getter)]
    pub fn room(this: &RoomObject) -> Option<Room>;
}

impl<T> HasPosition for T where T: AsRef<RoomObject> {
    fn pos(&self) -> Position {
        RoomObject::pos(self.as_ref()).into()
    }
}

impl<T> RoomObjectProperties for T where T: AsRef<RoomObject> {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}