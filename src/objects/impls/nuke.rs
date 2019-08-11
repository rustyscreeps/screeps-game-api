use crate::{local::LocalRoomName, macros::*, objects::Nuke};

simple_accessors! {
    Nuke;
    // id from HasID
    (lauch_room_name -> lauchRoomName -> LocalRoomName),
    (time_to_land -> timeToLand -> u32),
}
