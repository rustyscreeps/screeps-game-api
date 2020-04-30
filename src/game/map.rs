//! See [http://docs.screeps.com/api/#Game.map]
//!
//! [http://docs.screeps.com/api/#Game.map]: http://docs.screeps.com/api/#Game.map
use std::{borrow::Cow, collections, mem, str::FromStr};

use num_traits::FromPrimitive;
use parse_display::FromStr;
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize,
};
use stdweb::Value;

use crate::{
    constants::{Direction, ExitDirection, ReturnCode},
    local::RoomName,
    objects::RoomTerrain,
    traits::{TryFrom, TryInto},
};

/// See [http://docs.screeps.com/api/#Game.map.describeExits]
///
/// [http://docs.screeps.com/api/#Game.map.describeExits]: http://docs.screeps.com/api/#Game.map.describeExits
pub fn describe_exits(room_name: RoomName) -> collections::HashMap<Direction, RoomName> {
    let orig: collections::HashMap<String, RoomName> =
        js_unwrap!(Game.map.describeExits(@{room_name}) || {});

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
pub fn get_room_linear_distance(room1: RoomName, room2: RoomName, continuous: bool) -> u32 {
    js_unwrap!(Game.map.getRoomLinearDistance(@{room1}, @{room2}, @{continuous}))
}

pub fn get_room_terrain(room_name: RoomName) -> RoomTerrain {
    js_unwrap!(Game.map.getRoomTerrain(@{room_name}))
}

/// See [http://docs.screeps.com/api/#Game.map.getWorldSize]
///
/// [http://docs.screeps.com/api/#Game.map.getWorldSize]: http://docs.screeps.com/api/#Game.map.getWorldSize
pub fn get_world_size() -> u32 {
    js_unwrap!(Game.map.getWorldSize())
}

/// See [http://docs.screeps.com/api/#Game.map.getRoomStatus]
///
/// [http://docs.screeps.com/api/#Game.map.getRoomStatus]: http://docs.screeps.com/api/#Game.map.getRoomStatus
pub fn get_room_status(room_name: RoomName) -> MapRoomStatus {
    js_unwrap!(Game.map.getRoomStatus(@{room_name}))
}

/// Represents the availability and respawn/novice state of a room on the map
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapRoomStatus {
    pub status: RoomStatus,
    pub timestamp: Option<u64>,
}
js_deserializable!(MapRoomStatus);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromStr)]
#[display(style = "camelCase")]
pub enum RoomStatus {
    Normal,
    Closed,
    Novice,
    Respawn,
}

impl<'de> Deserialize<'de> for RoomStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Cow<'de, str> = Cow::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a known getRoomStatus status string")
        })
    }
}

/// Implements `Game.map.findExit`.
pub fn find_exit(from_room: RoomName, to_room: RoomName) -> Result<ExitDirection, ReturnCode> {
    let code: i32 = js_unwrap! {Game.map.findExit(@{from_room}, @{to_room})};
    ExitDirection::from_i32(code)
        .ok_or_else(|| ReturnCode::from_i32(code).expect("find_exit: Error code not recognized."))
}

pub fn find_exit_with_callback(
    from_room: RoomName,
    to_room: RoomName,
    route_callback: impl FnMut(RoomName, RoomName) -> f64,
) -> Result<ExitDirection, ReturnCode> {
    let mut raw_callback = route_callback;

    let mut callback_boxed = move |to_name: RoomName, from_name: RoomName| -> f64 {
        raw_callback(to_name, from_name).into()
    };

    // Type erased and boxed callback: no longer a type specific to the closure
    // passed in, now unified as &Fn
    let callback_type_erased: &mut (dyn FnMut(RoomName, RoomName) -> f64) =
        &mut callback_boxed;

    // Overwrite lifetime of reference so it can be stuck in scoped_thread_local
    // storage: it's now pretending to be static data. This should be entirely safe
    // because we're only sticking it in scoped storage and we control the
    // only use of it, but it's still necessary because "some lifetime above
    // the  current scope but otherwise unknown" is not a valid lifetime to
    // have PF_CALLBACK have.
    let callback_lifetime_erased: &'static mut dyn FnMut(RoomName, RoomName) -> f64 =
        unsafe { mem::transmute(callback_type_erased) };

    let code: i32 = js!(
        let cb = @{callback_lifetime_erased};
        
        let res = Game.map.findExit(@{from_room}, @{to_room}, cb);

        cb.drop();

        return res;
    )
    .try_into()
    .expect("expected int from findExit");

    ExitDirection::from_i32(code)
        .map(Ok)
        .or_else(|| ReturnCode::from_i32(code).map(Err))
        .unwrap_or_else(|| {
            panic!(
                "find_exit: return value {:?} not recognized as either Exit nor ReturnCode",
                code
            )
        })
}

pub fn find_route(from_room: &str, to_room: &str) -> Result<Vec<RoomRouteStep>, ReturnCode> {
    let v = js!(return Game.map.findRoute(@{from_room}, @{to_room}););
    parse_find_route_returned_value(v)
}

pub fn find_route_with_callback(
    from_room: RoomName,
    to_room: RoomName,
    route_callback: impl FnMut(RoomName, RoomName) -> f64,
) -> Result<Vec<RoomRouteStep>, ReturnCode> {
    let mut raw_callback = route_callback;

    let mut callback_boxed = move |to_name: RoomName, from_name: RoomName| -> f64 {
        raw_callback(to_name, from_name).into()
    };

    // Type erased and boxed callback: no longer a type specific to the closure
    // passed in, now unified as &Fn
    let callback_type_erased: &mut (dyn FnMut(RoomName, RoomName) -> f64) =
        &mut callback_boxed;

    // Overwrite lifetime of reference so it can be stuck in scoped_thread_local
    // storage: it's now pretending to be static data. This should be entirely safe
    // because we're only sticking it in scoped storage and we control the
    // only use of it, but it's still necessary because "some lifetime above
    // the  current scope but otherwise unknown" is not a valid lifetime to
    // have PF_CALLBACK have.
    let callback_lifetime_erased: &'static mut dyn FnMut(RoomName, RoomName) -> f64 =
        unsafe { mem::transmute(callback_type_erased) };

    let v = js!(
        let cb = @{callback_lifetime_erased};

        let res = Game.map.findRoute(@{from_room}, @{to_room}, cb);

        cb.drop();

        return res;
    );

    parse_find_route_returned_value(v)
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
    pub exit: ExitDirection,
    pub room: RoomName,
}
js_deserializable!(RoomRouteStep);
