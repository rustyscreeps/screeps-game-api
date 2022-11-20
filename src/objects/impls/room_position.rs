use std::convert::TryInto;

use crate::{
    constants::{look::*, Color, Direction, Look, ReturnCode, StructureType},
    local::Position,
    prelude::*,
    prototypes::ROOM_POSITION_PROTOTYPE,
    Find, FindConstant, LookConstant, RoomName,
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a position in a room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition)
    pub type RoomPosition;

    #[wasm_bindgen(constructor)]
    fn new_internal(x: u8, y: u8, room_name: &JsString) -> RoomPosition;

    #[wasm_bindgen(method, getter = roomName)]
    fn room_name_internal(this: &RoomPosition) -> JsString;

    /// Change the room the position refers to; must be a valid room name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.roomName)
    #[wasm_bindgen(method, setter = roomName)]
    pub fn set_room_name(this: &RoomPosition) -> JsString;

    /// X coordinate of the position within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, getter)]
    pub fn x(this: &RoomPosition) -> u8;

    /// Set a new X coordinate; must be in the range 0..49.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &RoomPosition) -> u8;

    /// Y coordinate of the position within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &RoomPosition) -> u8;

    /// Set a new Y coordinate; must be in the range 0..49.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &RoomPosition) -> u8;

    // todo, get this as native
    /// Gets the efficient internal i32 representation of the position.
    #[wasm_bindgen(method, getter = __packedPos)]
    pub fn packed(this: &RoomPosition) -> u32;

    // todo, as native
    /// Sets the position to a new one by passing an i32 that represents a
    /// packed position.
    #[wasm_bindgen(method, setter = __packedPos)]
    pub fn set_packed(this: &RoomPosition, val: u32);

    /// Creates a [`ConstructionSite`] at this position. If it's a
    /// [`StructureSpawn`], a name can optionally be assigned for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    ///
    /// [`ConstructionSite`]: crate::objects::ConstructionSite
    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_construction_site(
        this: &RoomPosition,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> ReturnCode;

    // todo we need to handle the fact that if this succeeds the name of the flag is
    // returned, and maybe also the fact that it'll throw a js exception when
    // created in a non visible room.. hmm
    /// Creates a [`Flag`] at this position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createFlag)
    ///
    /// [`Flag`]: crate::objects::Flag
    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag(
        this: &RoomPosition,
        name: Option<&JsString>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> ReturnCode;

    // todo FindOptions
    #[wasm_bindgen(method, js_name = findClosestByPath)]
    fn find_closest_by_path_internal(
        this: &RoomPosition,
        goal: Find,
        options: Option<&Object>,
    ) -> Option<Object>;

    // todo FindOptions
    #[wasm_bindgen(method, js_name = findClosestByRange)]
    fn find_closest_by_range_internal(
        this: &RoomPosition,
        goal: Find,
        options: Option<&Object>,
    ) -> Option<Object>;

    // todo FindOptions
    #[wasm_bindgen(method, js_name = findInRange)]
    fn find_in_range_internal(
        this: &RoomPosition,
        goal: Find,
        range: u8,
        options: Option<&Object>,
    ) -> Option<Array>;

    // todo FindPathOptions
    /// Find a path from this position to a position or room object, with an
    /// optional options object
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[wasm_bindgen(method, js_name = findPathTo)]
    pub fn find_path_to(this: &RoomPosition, goal: &JsValue, options: Option<&Object>) -> Array;

    // todo FindPathOptions
    /// Find a path from this position to the given coordinates in the same
    /// room, with an optional options object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[wasm_bindgen(method, js_name = findPathTo)]
    pub fn find_path_to_xy(this: &RoomPosition, x: u8, y: u8, options: Option<&Object>) -> Array;

    /// Get the [`Direction`] toward a position or room object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getDirectionTo)
    #[wasm_bindgen(method, js_name = getDirectionTo)]
    pub fn get_direction_to(this: &RoomPosition, goal: &JsValue) -> Direction;

    /// Get the [`Direction`] toward the given coordinates in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getDirectionTo)
    #[wasm_bindgen(method, js_name = getDirectionTo)]
    pub fn get_direction_to_xy(this: &RoomPosition, x: u8, y: u8) -> Direction;

    /// Get the range to a position or room object in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getRangeTo)
    #[wasm_bindgen(method, js_name = getRangeTo)]
    pub fn get_range_to(this: &RoomPosition, goal: &JsValue) -> u32;

    /// Get the range to the given coordinates in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getRangeTo)
    #[wasm_bindgen(method, js_name = getRangeTo)]
    pub fn get_range_to_xy(this: &RoomPosition, x: u8, y: u8) -> u32;

    /// Get the range to a position or room object in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.inRangeTo)
    #[wasm_bindgen(method, js_name = inRangeTo)]
    pub fn in_range_to(this: &RoomPosition, goal: &JsValue, range: u8) -> bool;

    /// Get the range to the given coordinates in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.inRangeTo)
    #[wasm_bindgen(method, js_name = inRangeTo)]
    pub fn in_range_to_xy(this: &RoomPosition, x: u8, y: u8, range: u8) -> bool;

    /// Determine whether this position is at the same position as another
    /// position or room object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isEqualTo)
    #[wasm_bindgen(method, js_name = isEqualTo)]
    pub fn is_equal_to(this: &RoomPosition, goal: &JsValue) -> bool;

    /// Determine whether this position is at the given coordinates in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isEqualTo)
    #[wasm_bindgen(method, js_name = isEqualTo)]
    pub fn is_equal_to_xy(this: &RoomPosition, x: u8, y: u8) -> bool;

    /// Determine whether this position is within 1 range of another position or
    /// room object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isNearTo)
    #[wasm_bindgen(method, js_name = isNearTo)]
    pub fn is_near_to(this: &RoomPosition, goal: &JsValue) -> bool;

    /// Determine whether this position is within 1 range of the given
    /// coordinates in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isNearTo)
    #[wasm_bindgen(method, js_name = isNearTo)]
    pub fn is_near_to_xy(this: &RoomPosition, x: u8, y: u8) -> bool;

    #[wasm_bindgen(method, js_name = look)]
    fn look_internal(this: &RoomPosition) -> Array;

    #[wasm_bindgen(method, js_name = lookFor)]
    fn look_for_internal(this: &RoomPosition, ty: Look) -> Option<Array>;
}

impl RoomPosition {
    /// Create a new RoomPosition using the normal constructor, taking
    /// coordinates and the room name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.constructor)
    pub fn new(x: u8, y: u8, room_name: RoomName) -> RoomPosition {
        let room_name = room_name.into();

        Self::new_internal(x, y, &room_name)
    }

    /// Name of the room the position is in, as an owned [`JsString`] reference
    /// to a string in Javascript memory.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.roomName)
    pub fn room_name(&self) -> RoomName {
        Self::room_name_internal(self)
            .try_into()
            .expect("expected parseable room name")
    }

    /// Find the closest object by path among an [`Array`] of objects, or among
    /// a [`FindType`] to search for all objects of that type in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByPath)
    pub fn find_closest_by_path<T>(&self, find: T, options: Option<&Object>) -> Option<T::Item>
    where
        T: FindConstant,
    {
        self.find_closest_by_path_internal(find.find_code(), options)
            .map(|reference| T::convert_and_check_item(reference.into()))
    }

    /// Find the closest object by range among an [`Array`] of objects, or among
    /// a [`FindType`] to search for all objects of that type in the room. Will
    /// not work for objects in other rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByRange)
    pub fn find_closest_by_range<T>(&self, find: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        self.find_closest_by_range_internal(find.find_code(), None)
            .map(|reference| T::convert_and_check_item(reference.into()))
    }

    /// Find all relevant objects within a certain range among an [`Array`] of
    /// objects, or among a [`FindType`] to search all objects of that type in
    /// the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findInRange)
    pub fn find_in_range<T>(&self, find: T, range: u8) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        self.find_in_range_internal(find.find_code(), range, None)
            .map(|arr| arr.iter().map(T::convert_and_check_item).collect())
            .unwrap_or_else(Vec::new)
    }

    /// Get all objects at this position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.look)
    pub fn look(&self) -> Vec<LookResult> {
        self.look_internal()
            .iter()
            .map(LookResult::from_jsvalue_unknown_type)
            .collect()
    }

    /// Get all objects of a given type at this position, if any.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.lookFor)
    pub fn look_for<T>(&self, _ty: T) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        self.look_for_internal(T::look_code())
            .map(|arr| arr.iter().map(T::convert_and_check_item).collect())
            .unwrap_or_else(Vec::new)
    }
}

impl Clone for RoomPosition {
    fn clone(&self) -> Self {
        let new_pos = RoomPosition::from(JsValue::from(Object::create(&ROOM_POSITION_PROTOTYPE)));
        new_pos.set_packed(self.packed());
        new_pos
    }
}

impl HasPosition for RoomPosition {
    fn pos(&self) -> Position {
        self.into()
    }
}

impl From<Position> for RoomPosition {
    fn from(pos: Position) -> Self {
        let js_pos = RoomPosition::from(JsValue::from(Object::create(&ROOM_POSITION_PROTOTYPE)));
        js_pos.set_packed(pos.packed_repr());
        js_pos
    }
}

impl From<&Position> for RoomPosition {
    fn from(pos: &Position) -> Self {
        let js_pos = RoomPosition::from(JsValue::from(Object::create(&ROOM_POSITION_PROTOTYPE)));
        js_pos.set_packed(pos.packed_repr());
        js_pos
    }
}
