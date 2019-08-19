use crate::{local::RoomName, macros::*, objects::Nuke};

simple_accessors! {
    Nuke;
    // id from HasID
    (launch_room_name -> launchRoomName -> RoomName),
    (time_to_land -> timeToLand -> u32),
}
