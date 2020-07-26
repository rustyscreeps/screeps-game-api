//! Contains different ways to access the content of [`Game.rooms`] in Screeps.
//!
//! [#Game.rooms]: http://docs.screeps.com/api/#Game.rooms
use js_sys::{Object, Reflect};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use crate::game::Game;
pub use crate::{local::RoomName, objects::Room};

/// Retrieve the full `HashMap<RoomName, Room>`.
pub fn hashmap() -> HashMap<RoomName, Room> {
    let rooms = Game::rooms();
    let mut map = HashMap::new();
    for room in Object::values(&rooms).iter() {
        let room = Room::from(room);
        map.insert(room.name(), room);
    }
    map
}

/// Retrieve the string keys of this object.
pub fn keys() -> Vec<RoomName> {
    let rooms = Game::rooms();
    let mut room_name_vec = vec![];
    for room in Object::values(&rooms).iter() {
        room_name_vec.push(Room::from(room).name());
    }
    room_name_vec
}

/// Retrieve all values in this object.
pub fn values() -> Vec<Room> {
    let rooms = Game::rooms();
    let mut room_vec = vec![];
    for room in Object::values(&rooms).iter() {
        room_vec.push(room.into());
    }
    room_vec
}

/// Retrieve a specific value by key, or None if the room is not currently
/// visible
pub fn get(name: RoomName) -> Option<Room> {
    let rooms = Game::rooms();
    match Reflect::get(&rooms, &JsValue::from_str(&name.to_string())) {
        Ok(room) => {
            if !room.is_undefined() {
                Some(room.into())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
