//! Game map related functionality.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game-map)

use js_sys::{Array, JsString, Object};
use num_traits::*;
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::ExitDirection, js_collections::JsHashMap, objects::RoomTerrain, Direction,
    ReturnCode, RoomName,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "map")]
    type Map;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = describeExits)]
    fn describe_exits(room_name: &JsString) -> Object;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = findExit)]
    fn find_exit(from_room: &JsString, to_room: &JsString, options: &JsValue) -> i32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = findRoute)]
    fn find_route(from_room: &JsString, to_room: &JsString, options: &JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = getRoomLinearDistance)]
    fn get_room_linear_distance(room_1: &JsString, room_2: &JsString, continuous: bool) -> u32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = getRoomTerrain)]
    fn get_room_terrain(room_name: &JsString) -> RoomTerrain;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = getWorldSize)]
    fn get_world_size() -> u32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "map", static_method_of = Map, js_name = getRoomStatus, catch)]
    fn get_room_status(room_name: &JsString) -> Result<JsRoomStatusResult, JsValue>;
}

/// Get an object with information about the exits from a given room, with
/// [`JsString`] versions of direction integers as keys and [`JsString`]
/// room names as values.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.describeExits)
pub fn describe_exits(room_name: RoomName) -> JsHashMap<Direction, RoomName> {
    let room_name = room_name.into();

    Map::describe_exits(&room_name).into()
}

/// Get the distance used for range calculations between two rooms,
/// optionally setting `continuous` to true to consider the world borders to
/// wrap around, which is used for terminal calculations.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getRoomLinearDistance)
pub fn get_room_linear_distance(from_room: RoomName, to_room: RoomName, continuous: bool) -> u32 {
    let from_room = from_room.into();
    let to_room = to_room.into();

    Map::get_room_linear_distance(&from_room, &to_room, continuous)
}

/// Get the [`RoomTerrain`] object for any room, even one you don't have
/// vision in.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getRoomTerrain)
pub fn get_room_terrain(room_name: RoomName) -> RoomTerrain {
    let name = room_name.into();

    Map::get_room_terrain(&name)
}

/// Get the size of the world map.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getWorldSize)
pub fn get_world_size() -> u32 {
    Map::get_world_size()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type JsRoomStatusResult;

    #[wasm_bindgen(method, getter = status)]
    pub fn status(this: &JsRoomStatusResult) -> RoomStatus;

    #[wasm_bindgen(method, getter = timestamp)]
    pub fn timestamp(this: &JsRoomStatusResult) -> Option<f64>;
}

pub struct RoomStatusResult {
    status: RoomStatus,
    timestamp: Option<f64>,
}

impl RoomStatusResult {
    pub fn status(&self) -> RoomStatus {
        self.status
    }

    pub fn timestamp(&self) -> Option<f64> {
        self.timestamp
    }
}

impl Default for RoomStatusResult {
    fn default() -> Self {
        RoomStatusResult {
            status: RoomStatus::Normal,
            timestamp: None,
        }
    }
}

impl From<JsRoomStatusResult> for RoomStatusResult {
    fn from(val: JsRoomStatusResult) -> Self {
        RoomStatusResult {
            status: val.status(),
            timestamp: val.timestamp(),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RoomStatus {
    Normal = "normal",
    Closed = "closed",
    Novice = "novice",
    Respawn = "respawn",
}

/// Get the status of a given room, determining whether it's in a special
/// area or currently inaccessible.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getRoomStatus)
pub fn get_room_status(room_name: RoomName) -> RoomStatusResult {
    let name = room_name.into();

    Map::get_room_status(&name)
        .ok()
        .map(RoomStatusResult::from)
        .unwrap_or_default()
}

#[wasm_bindgen]
extern "C" {
    /// Object that represents a set of options for a call to [`find_route`].
    #[wasm_bindgen]
    pub type JsFindRouteOptions;

    /// Route callback, which determines the cost of entering a given room (the
    /// first parameter) from a given neighbor room (the second parameter), or
    /// [`f64::INFINITY`] to block entry into the room.
    #[wasm_bindgen(method, setter = routeCallback)]
    pub fn route_callback(
        this: &JsFindRouteOptions,
        callback: &Closure<dyn FnMut(JsString, JsString) -> f64>,
    );
}

impl JsFindRouteOptions {
    pub fn new() -> JsFindRouteOptions {
        Object::new().unchecked_into()
    }
}

impl Default for JsFindRouteOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FindRouteOptions<F>
where
    F: FnMut(RoomName, RoomName) -> f64,
{
    route_callback: F,
}

impl<F> FindRouteOptions<F>
where
    F: FnMut(RoomName, RoomName) -> f64,
{
    pub(crate) fn into_js_options<R>(self, callback: impl Fn(&JsFindRouteOptions) -> R) -> R {
        let mut raw_callback = self.route_callback;

        let mut owned_callback = move |to_room: RoomName, from_room: RoomName| -> f64 {
            raw_callback(to_room, from_room)
        };

        //
        // Type erased and boxed callback: no longer a type specific to the closure
        // passed in, now unified as &Fn
        //

        let callback_type_erased: &mut (dyn FnMut(RoomName, RoomName) -> f64) = &mut owned_callback;

        // Overwrite lifetime of reference so it can be passed to javascript.
        // It's now pretending to be static data. This should be entirely safe
        // because we control the only use of it and it remains valid during the
        // pathfinder callback. This transmute is necessary because "some lifetime
        // above the current scope but otherwise unknown" is not a valid lifetime.
        //

        let callback_lifetime_erased: &'static mut (dyn FnMut(RoomName, RoomName) -> f64) =
            unsafe { std::mem::transmute(callback_type_erased) };

        let boxed_callback = Box::new(move |to_room: JsString, from_room: JsString| -> f64 {
            let to_room = to_room
                .try_into()
                .expect("expected 'to' room name in route callback");
            let from_room = from_room
                .try_into()
                .expect("expected 'rom' room name in route callback");

            callback_lifetime_erased(to_room, from_room)
        }) as Box<dyn FnMut(JsString, JsString) -> f64>;

        let closure = Closure::wrap(boxed_callback);

        //
        // Create JS object and set properties.
        //

        let js_options = JsFindRouteOptions::new();

        js_options.route_callback(&closure);

        callback(&js_options)
    }
}

impl Default for FindRouteOptions<fn(RoomName, RoomName) -> f64> {
    fn default() -> Self {
        fn room_cost(_to_room: RoomName, _from_room: RoomName) -> f64 {
            1.0
        }

        FindRouteOptions {
            route_callback: room_cost,
        }
    }
}

impl FindRouteOptions<fn(RoomName, RoomName) -> f64> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<F> FindRouteOptions<F>
where
    F: FnMut(RoomName, RoomName) -> f64,
{
    pub fn room_callback<F2>(self, route_callback: F2) -> FindRouteOptions<F2>
    where
        F2: FnMut(RoomName, RoomName) -> f64,
    {
        let FindRouteOptions { route_callback: _ } = self;

        FindRouteOptions { route_callback }
    }
}

#[derive(Deserialize)]
pub struct RouteStep {
    pub exit: ExitDirection,
    pub room: RoomName,
}

/// Get the route from a given room leading toward a destination room, with
/// an optional [`FindRouteOptions`] parameter allowing control over the
/// costs to enter rooms.
///
/// Returns an [`Array`] with an object per room in the route, with keys
/// `exit` containing an [`ExitDirection`] and `room` containing room name
/// as a [`JsString`].
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.findRoute)
pub fn find_route<F>(
    from: RoomName,
    to: RoomName,
    options: Option<FindRouteOptions<F>>,
) -> Result<Vec<RouteStep>, ReturnCode>
where
    F: FnMut(RoomName, RoomName) -> f64,
{
    let from: JsString = from.into();
    let to: JsString = to.into();

    let result = if let Some(options) = options {
        options.into_js_options(|js_options| Map::find_route(&from, &to, js_options))
    } else {
        Map::find_route(&from, &to, &JsValue::UNDEFINED)
    };

    if result.is_object() {
        let result: &Array = result.unchecked_ref();

        let steps: Vec<RouteStep> = result
            .iter()
            .map(|step| serde_wasm_bindgen::from_value(step).expect("expected route step"))
            .collect();

        Ok(steps)
    } else {
        let return_code =
            ReturnCode::try_from(result).expect("expected return code for pathing failure");

        Err(return_code)
    }
}

/// Get the exit direction from a given room leading toward a destination
/// room, with an optional [`FindRouteOptions`] parameter allowing control
/// over the costs to enter rooms.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.map.findExit)
pub fn find_exit<F>(
    from: RoomName,
    to: RoomName,
    options: Option<FindRouteOptions<F>>,
) -> Result<ExitDirection, ReturnCode>
where
    F: FnMut(RoomName, RoomName) -> f64,
{
    let from: JsString = from.into();
    let to: JsString = to.into();

    let result = if let Some(options) = options {
        options.into_js_options(|js_options| Map::find_exit(&from, &to, js_options))
    } else {
        Map::find_exit(&from, &to, &JsValue::UNDEFINED)
    };

    if result >= 0 {
        Ok(ExitDirection::from_i32(result).expect("expected exit direction for pathing"))
    } else {
        Err(ReturnCode::from_i32(result).expect("expected return code for pathing failure"))
    }
}
