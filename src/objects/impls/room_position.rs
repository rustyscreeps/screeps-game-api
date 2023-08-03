use std::convert::TryInto;

use js_sys::{Array, JsString, Object};
use num_traits::*;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::{find::*, look::*, Color, Direction, ErrorCode, StructureType},
    local::{Position, RoomCoordinate, RoomName},
    objects::{CostMatrix, FindPathOptions, Path},
    pathfinder::RoomCostResult,
    prelude::*,
    prototypes::ROOM_POSITION_PROTOTYPE,
};

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

    #[wasm_bindgen(method, js_name = createConstructionSite)]
    fn create_construction_site_internal(
        this: &RoomPosition,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> i8;

    #[wasm_bindgen(method, catch, js_name = createFlag)]
    fn create_flag_internal(
        this: &RoomPosition,
        name: Option<&JsString>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> Result<JsValue, JsValue>;

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

    #[wasm_bindgen(method, js_name = findPathTo)]
    fn find_path_to_internal(
        this: &RoomPosition,
        target: &JsValue,
        options: Option<&Object>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = findPathTo)]
    fn find_path_to_xy_internal(
        this: &RoomPosition,
        x: u8,
        y: u8,
        options: Option<&Object>,
    ) -> JsValue;

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

    #[wasm_bindgen(method, catch, js_name = look)]
    fn look_internal(this: &RoomPosition) -> Result<Array, JsValue>;

    #[wasm_bindgen(method, catch, js_name = lookFor)]
    fn look_for_internal(this: &RoomPosition, ty: Look) -> Result<Option<Array>, JsValue>;
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

    /// Creates a [`ConstructionSite`] at this position. If it's a
    /// [`StructureSpawn`], a name can optionally be assigned for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    ///
    /// [`ConstructionSite`]: crate::objects::ConstructionSite
    /// [`StructureSpawn`]: crate::objects::StructureSpawn
    pub fn create_construction_site(
        &self,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(Self::create_construction_site_internal(self, ty, name))
    }

    /// Creates a [`Flag`] at this position. If successful, returns the name of
    /// the created flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createFlag)
    ///
    /// [`Flag`]: crate::objects::Flag
    pub fn create_flag(
        &self,
        name: Option<&JsString>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> Result<JsString, ErrorCode> {
        match self.create_flag_internal(name, color, secondary_color) {
            Ok(result) => {
                if result.is_string() {
                    Ok(result.unchecked_into())
                } else {
                    Err(ErrorCode::from_f64(
                        result
                            .as_f64()
                            .expect("expected non-string flag return to be a number"),
                    )
                    .expect("expected valid error code"))
                }
            }
            Err(_) => {
                // js code threw an exception; we're going to assume it's a non-visible room.
                Err(ErrorCode::NotInRange)
            }
        }
    }

    // todo typed options and version that allows passing target roomobjects
    /// Find the closest object by path among a list of objects, or use
    /// a [`find` constant] to search for all objects of that type in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByPath)
    ///
    /// [`find` constant]: crate::constants::find
    pub fn find_closest_by_path<T>(&self, find: T, options: Option<&Object>) -> Option<T::Item>
    where
        T: FindConstant,
    {
        self.find_closest_by_path_internal(find.find_code(), options)
            .map(|reference| T::convert_and_check_item(reference.into()))
    }

    // todo version for passing target roomobjects
    /// Find the closest object by range among a list of objects, or use
    /// a [`find` constant] to search for all objects of that type in the room.
    /// Will not work for objects in other rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByRange)
    ///
    /// [`find` constant]: crate::constants::find
    pub fn find_closest_by_range<T>(&self, find: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        self.find_closest_by_range_internal(find.find_code(), None)
            .map(|reference| T::convert_and_check_item(reference.into()))
    }

    // todo version for passing target roomobjects
    /// Find all relevant objects within a certain range among a list of
    /// objects, or use a [`find` constant] to search all objects of that type
    /// in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findInRange)
    ///
    /// [`find` constant]: crate::constants::find
    pub fn find_in_range<T>(&self, find: T, range: u8) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        self.find_in_range_internal(find.find_code(), range, None)
            .map(|arr| arr.iter().map(T::convert_and_check_item).collect())
            .unwrap_or_default()
    }

    /// Find a path from this position to a position or room object, with an
    /// optional options object
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    pub fn find_path_to<T, F, R>(&self, target: &T, options: Option<FindPathOptions<F, R>>) -> Path
    where
        T: HasPosition,
        F: FnMut(RoomName, CostMatrix) -> R,
        R: RoomCostResult,
    {
        let target: RoomPosition = target.pos().into();

        if let Some(options) = options {
            options.into_js_options(|js_options| {
                serde_wasm_bindgen::from_value(
                    self.find_path_to_internal(&target, Some(js_options.unchecked_ref())),
                )
                .expect("invalid path from RoomPosition.findPathTo")
            })
        } else {
            serde_wasm_bindgen::from_value(self.find_path_to_internal(&target, None))
                .expect("invalid path from RoomPosition.findPathTo")
        }
    }

    /// Find a path from this position to the given coordinates in the same
    /// room, with an optional options object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    pub fn find_path_to_xy<F, R>(
        &self,
        x: RoomCoordinate,
        y: RoomCoordinate,
        options: Option<FindPathOptions<F, R>>,
    ) -> Path
    where
        F: FnMut(RoomName, CostMatrix) -> R,
        R: RoomCostResult,
    {
        if let Some(options) = options {
            options.into_js_options(|js_options| {
                serde_wasm_bindgen::from_value(self.find_path_to_xy_internal(
                    x.into(),
                    y.into(),
                    Some(js_options.unchecked_ref()),
                ))
                .expect("invalid path from RoomPosition.findPathTo")
            })
        } else {
            serde_wasm_bindgen::from_value(self.find_path_to_xy_internal(x.into(), y.into(), None))
                .expect("invalid path from RoomPosition.findPathTo")
        }
    }

    /// Get all objects at this position. Will fail if the position is in a room
    /// that's not visible during the current tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.look)
    pub fn look(&self) -> Result<Vec<LookResult>, ErrorCode> {
        match self.look_internal() {
            Ok(array) => Ok(array
                .iter()
                .map(LookResult::from_jsvalue_unknown_type)
                .collect()),
            Err(_) => Err(ErrorCode::NotInRange),
        }
    }

    /// Get all objects of a given type at this position, if any. Will fail if
    /// the position is in a room that's not visible during the current tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.lookFor)
    pub fn look_for<T>(&self, _ty: T) -> Result<Vec<T::Item>, ErrorCode>
    where
        T: LookConstant,
    {
        match self.look_for_internal(T::look_code()) {
            Ok(array) => Ok(array
                .map(|arr| arr.iter().map(T::convert_and_check_item).collect())
                .unwrap_or_else(Vec::new)),
            Err(_) => Err(ErrorCode::NotInRange),
        }
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
