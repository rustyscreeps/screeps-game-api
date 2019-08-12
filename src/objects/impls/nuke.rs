use crate::{local::RoomName, macros::*, objects::Nuke};

simple_accessors! {
    Nuke;
    // id from HasID
    (lauch_room_name -> lauchRoomName -> RoomName),
    (time_to_land -> timeToLand -> u32),
}
