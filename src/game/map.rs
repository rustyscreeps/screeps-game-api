//! See [http://docs.screeps.com/api/#Game.map]
//!
//! [http://docs.screeps.com/api/#Game.map]: http://docs.screeps.com/api/#Game.map
use std::{collections, mem};

use num_traits::FromPrimitive;
use scoped_tls::scoped_thread_local;
use serde::Deserialize;
use stdweb::Value;

use crate::{
    constants::{Direction, ExitDirection, ReturnCode},
    local::LocalRoomName,
    macros::*,
    objects::RoomTerrain,
    traits::{TryFrom, TryInto},
};

/// See [http://docs.screeps.com/api/#Game.map.describeExits]
///
/// [http://docs.screeps.com/api/#Game.map.describeExits]: http://docs.screeps.com/api/#Game.map.describeExits
pub fn describe_exits(room_name: LocalRoomName) -> collections::HashMap<Direction, String> {
    let orig: collections::HashMap<String, String> =
        js_unwrap!(Game.map.describeExits(@{room_name}));

    orig.into_iter()
        .map(|(key, value)| {
            let key: u32 = key.parse().expect(
                "expected all directions returned from Game.map.describeExits to be integers",
            );
            (
                Direction::from_u32(key).expect(
                    "expected all directions returned from Game.map.describeExits to be directions",
                ),
                value,
            )
        })
        .collect()
}

/// See [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]
///
/// [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]: http://docs.screeps.com/api/#Game.map.getRoomLinearDistance
pub fn get_room_linear_distance(
    room1: LocalRoomName,
    room2: LocalRoomName,
    continuous: bool,
) -> u32 {
    js_unwrap!(Game.map.getRoomLinearDistance(@{room1}, @{room2}, @{continuous}))
}

pub fn get_room_terrain(room_name: LocalRoomName) -> RoomTerrain {
    js_unwrap!(Game.map.getRoomTerrain(@{room_name}))
}

/// See [http://docs.screeps.com/api/#Game.map.getWorldSize]
///
/// [http://docs.screeps.com/api/#Game.map.getWorldSize]: http://docs.screeps.com/api/#Game.map.getWorldSize
pub fn get_world_size() -> u32 {
    js_unwrap!(Game.map.getWorldSize())
}

/// See [http://docs.screeps.com/api/#Game.map.isRoomAvailable]
///
/// [http://docs.screeps.com/api/#Game.map.isRoomAvailable]: http://docs.screeps.com/api/#Game.map.isRoomAvailable
pub fn is_room_available(room_name: LocalRoomName) -> bool {
    js_unwrap!(Game.map.isRoomAvailable(@{room_name}))
}

/// Implements `Game.map.findExit`.
pub fn find_exit(
    from_room: LocalRoomName,
    to_room: LocalRoomName,
) -> Result<ExitDirection, ReturnCode> {
    let code: i32 = js_unwrap! {Game.map.findExit(@{from_room}, @{to_room})};
    ExitDirection::from_i32(code)
        .ok_or_else(|| ReturnCode::from_i32(code).expect("find_exit: Error code not recognized."))
}

pub fn find_exit_with_callback(
    from_room: LocalRoomName,
    to_room: LocalRoomName,
    route_callback: impl Fn(LocalRoomName, LocalRoomName) -> f64,
) -> Result<ExitDirection, ReturnCode> {
    // Actual callback
    fn callback(room_name: String, from_room_name: String) -> f64 {
        FR_CALLBACK.with(|callback| {
            callback(
                room_name.parse().expect(
                    "expected room name passed into Game.map.findExit \
                     callback to be a valid room name",
                ),
                from_room_name.parse().expect(
                    "expected room name passed into Game.map.findExit \
                     callback to be a valid room name",
                ),
            )
        })
    }

    let callback_type_erased: Box<dyn Fn(LocalRoomName, LocalRoomName) -> f64> =
        Box::new(route_callback);

    let callback_lifetime_erased: Box<dyn Fn(LocalRoomName, LocalRoomName) -> f64 + 'static> =
        unsafe { mem::transmute(callback_type_erased) };

    FR_CALLBACK.set(&callback_lifetime_erased, || {
        let code: i32 = js_unwrap! {Game.map.findExit(@{from_room}, @{to_room}, @{callback})};
        ExitDirection::from_i32(code)
            .map(Ok)
            .or_else(|| ReturnCode::from_i32(code).map(Err))
            .unwrap_or_else(|| {
                panic!(
                    "find_exit: return value {:?} not recognized as either Exit nor ReturnCode",
                    code
                )
            })
    })
}

pub fn find_route(from_room: &str, to_room: &str) -> Result<Vec<RoomRouteStep>, ReturnCode> {
    let v = js!(return Game.map.findRoute(@{from_room}, @{to_room}););
    parse_find_route_returned_value(v)
}

scoped_thread_local!(static FR_CALLBACK: Box<(dyn Fn(LocalRoomName, LocalRoomName) -> f64)>);

pub fn find_route_with_callback(
    from_room: LocalRoomName,
    to_room: LocalRoomName,
    route_callback: impl Fn(LocalRoomName, LocalRoomName) -> f64,
) -> Result<Vec<RoomRouteStep>, ReturnCode> {
    // Actual callback
    fn callback(room_name: String, from_room_name: String) -> f64 {
        FR_CALLBACK.with(|callback| {
            callback(
                room_name.parse().expect(
                    "expected room name passed into Game.map.findRoute \
                     callback to be a valid room name",
                ),
                from_room_name.parse().expect(
                    "expected room name passed into Game.map.findRoute \
                     callback to be a valid room name",
                ),
            )
        })
    }

    let callback_type_erased: Box<dyn Fn(LocalRoomName, LocalRoomName) -> f64> =
        Box::new(route_callback);

    let callback_lifetime_erased: Box<dyn Fn(LocalRoomName, LocalRoomName) -> f64 + 'static> =
        unsafe { mem::transmute(callback_type_erased) };

    FR_CALLBACK.set(&callback_lifetime_erased, || {
        let v = js!(return Game.map.findRoute(@{from_room}, @{to_room}, @{callback}););
        parse_find_route_returned_value(v)
    })
}

fn parse_find_route_returned_value(v: Value) -> Result<Vec<RoomRouteStep>, ReturnCode> {
    match v {
        Value::Number(x) => Err(ReturnCode::try_from(Value::Number(x)).unwrap_or_else(|e| {
            panic!(
                "parse_find_route_returned_value: unknown return value: {:?} (err: {})",
                x, e
            )
        })),
        Value::Reference(_) => Ok(v.try_into().expect("Error on parsing exit directions.")),
        _ => panic!(
            "Game.map.findRoute expected Number or Reference, found {:?}.",
            v
        ),
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomRouteStep {
    exit: ExitDirection,
    room: String,
}
js_deserializable!(RoomRouteStep);
