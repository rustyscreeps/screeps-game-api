use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::EffectType,
    local::Position,
    objects::{Room, RoomPosition},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// Parent class for all objects in rooms in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room)
    #[derive(Clone, Debug)]
    pub type RoomObject;

    /// Effects applied to the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.effects)
    #[wasm_bindgen(method, getter = effects)]
    fn effects_internal(this: &RoomObject) -> Option<Array>;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type Effect;

    #[wasm_bindgen(method, getter)]
    pub fn effect(this: &Effect) -> EffectType;

    #[wasm_bindgen(method, getter)]
    pub fn level(this: &Effect) -> Option<u8>;

    #[wasm_bindgen(method, getter = ticksRemaining)]
    pub fn ticks_remaining(this: &Effect) -> EffectType;
}

impl<T> HasPosition for T
where
    T: AsRef<RoomObject>,
{
    fn pos(&self) -> Position {
        RoomObject::pos(self.as_ref()).into()
    }
}

impl<T> RoomObjectProperties for T
where
    T: AsRef<RoomObject>,
{
    fn effects(&self) -> Vec<Effect> {
        RoomObject::effects_internal(self.as_ref())
            .map(|arr| arr.iter().map(JsCast::unchecked_into).collect())
            .unwrap_or_else(Vec::new)
    }

    fn effects_raw(&self) -> Option<Array> {
        RoomObject::effects_internal(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
